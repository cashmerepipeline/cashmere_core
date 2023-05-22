use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewCountry {
    async fn handle_new_country(
        &self,
        request: Request<NewCountryRequest>,
    ) -> UnaryResponseResult<NewCountryResponse> {
                let (account_id, _groups, role_group ) = request_account_context(request.metadata());


        let name = &request.get_ref().name;
        let native = &request.get_ref().native;
        let code = &request.get_ref().code;
        let phone_area_code = &request.get_ref().phone_area_code;
        let languages = &request.get_ref().languages;

        if !view::can_collection_write(&account_id, &role_group, &AREAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(COUNTRIES_MANAGE_ID)
            .await
            .unwrap();

        let local_name =match name {
            Some(n)=>n,
            None => {
                return Err(Status::aborted(format!("没有指定名称--{}", code)));
            }
        };

        let name_doc = doc!{local_name.language.clone():local_name.name.clone()};

        // 是否存在，存在则返回
        if manager
            .entity_exists(&doc! {
                COUNTRIES_CODE_FIELD_ID.to_string():code.clone(),
            })
            .await
        {
            return Err(Status::aborted("国家已经存在"));
        }

        if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert(ID_FIELD_ID.to_string(), code);
            new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
            new_entity_doc.insert(COUNTRIES_NATIVE_FIELD_ID.to_string(), native.clone());
            new_entity_doc.insert(COUNTRIES_CODE_FIELD_ID.to_string(), code);
            new_entity_doc.insert(COUNTRIES_PHONE_AREA_CODE_FIELD_ID.to_string(), phone_area_code);
            new_entity_doc.insert(COUNTRIES_LANGUAGES_FIELD_ID.to_string(), languages);

            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(_r) => Ok(Response::new(NewCountryResponse {
                    result: code.to_string(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("创建国家域失败"))
        }
    }
}

