use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::rust_i18n::{self, t};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::{entity_interface::EntityInterface};
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;
use validates::{validate_entity_id, validate_manage_id, validate_name};

#[async_trait]
pub trait HandleNewContact {
    async fn handle_new_contact(
        &self,
        request: Request<NewContactRequest>,
    ) -> UnaryResponseResult<NewContactResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_contact)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewContactRequest>,
) -> Result<Request<NewContactRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = PURPOSES_MANAGE_ID;

        let (account_id, groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewContactRequest>,
) -> Result<Request<NewContactRequest>, Status> {
    let target_manage_id = &request.get_ref().manange_id;
    let target_entity_id = &request.get_ref().entity_id;
    let purpose = &request.get_ref().purpose;

    validate_manage_id(target_manage_id).await?; 
    validate_entity_id(&target_manage_id, &target_entity_id).await?;
    validate_entity_id(PURPOSES_MANAGE_ID, &purpose).await?;
    
    Ok(request)
}

async fn handle_new_contact(request: Request<NewContactRequest>) -> UnaryResponseResult<NewContactResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;
    let manager_id = PURPOSES_MANAGE_ID;

    let target_manage_id = &request.get_ref().manange_id;
    let target_entity_id = &request.get_ref().entity_id;
    let purpose = &request.get_ref().purpose;
    let account = &request.get_ref().account;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manager_id)
        .unwrap();

    // 是否存在，存在则返回
    if manager
        .entity_exists(&doc! {
            CONTACTS_MANAGE_FIELD_ID.to_string(): target_manage_id,
            CONTACTS_ENTITY_FIELD_ID.to_string(): target_entity_id,
            CONTACTS_PURPOSE_FIELD_ID.to_string(): purpose,
            CONTACTS_ACCOUNT_FIELD_ID.to_string(): account,
        })
        .await.is_some()
    {
        return Err(Status::aborted(format!(
            "{}: {}-{}-{}-{}",
            t!("联系方式已经存在"),
            target_manage_id,
            target_entity_id,
            purpose,
            account,
        )));
    }

    if let Ok(mut new_entity_doc) = make_new_entity_document(manager, &account_id).await {
        new_entity_doc.insert(CONTACTS_MANAGE_FIELD_ID.to_string(), target_manage_id);
        new_entity_doc.insert(CONTACTS_ENTITY_FIELD_ID.to_string(), target_entity_id);
        new_entity_doc.insert(CONTACTS_PURPOSE_FIELD_ID.to_string(), purpose);
        new_entity_doc.insert(CONTACTS_ACCOUNT_FIELD_ID.to_string(), account);

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        let _new_id = new_entity_doc.get_str(ID_FIELD_ID.to_string()).unwrap();

        match result {
            Ok(_r) => Ok(Response::new(NewContactResponse {
                result: _r,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!("{}: {}", t!("获取新实体失败"), "new_contact")))
    }
}
