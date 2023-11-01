
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::{Response, Status, Request};
use dependencies_sync::bson::{self, doc};

use dependencies_sync::log::{error};
use dependencies_sync::tonic::async_trait;


use majordomo::get_majordomo;
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use managers::ManagerTrait;
use request_utils::request_account_context;




#[async_trait]
pub trait HandleEditTemplate {
    async fn handle_edit_entity_template(
        &self,
        request: Request<EditTemplateRequest>,
    ) -> Result<Response<EditTemplateResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_entity_template)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditTemplateRequest>,
) -> Result<Request<EditTemplateRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = TEMPLATES_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_entit_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<EditTemplateRequest>,
) -> Result<Request<EditTemplateRequest>, Status> {
    Ok(request)
}

async fn handle_edit_entity_template(
    request: Request<EditTemplateRequest>,
) -> Result<Response<EditTemplateResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let template_id = &request.get_ref().template_id;
    let fields = &request.get_ref().fields;

    let majordomo_arc = get_majordomo();
    let template_manager = majordomo_arc
        .get_manager_by_id(TEMPLATES_MANAGE_ID)
        .unwrap();

    let query_doc = doc! {
      ID_FIELD_ID.to_string(): template_id,
    };

    let mut modify_doc = doc! {
      TEMPLATES_PRESETS_FIELD_ID.to_string(): bson::to_bson(fields).unwrap(),
    };

    let result = template_manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditTemplateResponse {
            result: "ok".to_string(),
        })),
        Err(e) => {
            error!("handle_edit_entity_template error: {:?}", e);
            Err(Status::internal("handle_edit_entity_template error"))
        }
    }
}
