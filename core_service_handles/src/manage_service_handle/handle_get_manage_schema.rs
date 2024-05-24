use std::collections::HashMap;

use dependencies_sync::bson::doc;
use dependencies_sync::futures::{Stream, TryFutureExt};
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio::stream;
use dependencies_sync::tokio_stream;
use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use cash_core::SchemaField as CoreSchemaField;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::hard_coded_cache_interface::HardCodedInterface;
use managers::manager_trait::ManagerInterface;
use request_utils::request_account_context;

use view::{can_field_read, can_field_write};

#[async_trait]
pub trait HandleGetManageSchema {
    /// 取得管理描写
    async fn handle_get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_manage_schema)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetManageSchemaRequest>,
) -> Result<Request<GetManageSchemaRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetManageSchemaRequest>,
) -> Result<Request<GetManageSchemaRequest>, Status> {
    Ok(request)
}

async fn handle_get_manage_schema(
    request: Request<GetManageSchemaRequest>,
) -> Result<Response<GetManageSchemaResponse>, Status> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let fields = manager.get_manage_schema().await;

    let mut field_stream = tokio_stream::iter(&fields);

    // 可见性过滤
    let mut core_fields: Vec<CoreSchemaField> = vec![];
    while let Some(field) = field_stream.next().await {
        if can_field_read(manage_id, &field.id.to_string(), &role_group).await {
            core_fields.push(field.to_owned());
        } else {
            if cfg!(debug_assertions) {
                debug!(
                    "{}:, {}-{}, {}",
                    t!("属性不可见"),
                    &manage_id,
                    &field.id,
                    role_group
                );
            }
            continue;
        }
    }

    let mut streamed_fields = tokio_stream::iter(&core_fields);
    let mut result: Vec<SchemaField> = vec![];
    while let Some(f) = streamed_fields.next().await {
        let mut name_map = HashMap::new();
        f.name_map.iter().for_each(|(k, v)| {
            name_map.insert(k.to_string(), v.to_string());
        });

        let hard_coded = majordomo_arc
            .get_manager_by_id(&manage_id)
            .unwrap()
            .is_hard_coded()
            .await;

        // zh: 是否可编辑
        let editable = can_field_write(
            manage_id,
            f.id.to_string().as_str(),
            hard_coded,
            &role_group,
        )
        .await;

        result.push(SchemaField {
            id: f.id,
            name_map,
            data_type: f.data_type.to_string(),
            removed: f.removed,
            editable,
        });
    }

    // 如果为空则返回空表，无异常发生
    Ok(Response::new(GetManageSchemaResponse { fields: result }))
}
