#[macro_export]
macro_rules! declare_handle_new_graph {
    ($server:ty) => {
        pub(crate) async fn handle_new_graph(
            &self,
            request: Request<NewGraphRequest>,
        ) -> UnaryResponseResult<NewGraphResponse> {
            let metadata = request.metadata();
            // 已检查过，不需要再检查正确性
            let token = auth::get_auth_token(metadata).unwrap();
            let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

            let domain_id = &request.get_ref().domain_id;
            let individual_level = &request.get_ref().level;
            let individual_id = &request.get_ref().individual_id;
            let language = &request.get_ref().language;
            let name = &request.get_ref().name;
            let compose_type = &request.get_ref().compose_type;

            if !view::can_manage_write(&account_id, &groups, &GRAPHS_MANAGE_ID.to_string()).await {
                return Err(Status::unauthenticated("用户不具有可写权限"));
            }
            // 取得第一个可写组作为组
            let group_id =
                match view::get_first_write_group(&groups, &GRAPHS_MANAGE_ID.to_string()).await {
                    Some(r) => r,
                    None => return Err(Status::unauthenticated("用户不具有可写权限")),
                };

            let majordomo_arc = get_majordomo().await;
            let manager = majordomo_arc
                .get_manager_by_id(GRAPHS_MANAGE_ID)
                .await
                .unwrap();

            let local_name = doc! {
                local.clone(): name.clone()
            };

            if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert("_id", new_id.to_string());
            new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
            new_entity_doc.insert(NAME_FIELD_ID.to_string(), local_name);
            new_entity_doc.insert(
                GRAPHS_INDIVIDUAL_LEVEL_FIELD_ID.to_string(),
                individual_level,
            );
            new_entity_doc.insert(GRAPHS_INDIVIDUAL_ID_FIELD_ID.to_string(), individual_id);
            new_entity_doc.insert(GRAPHS_DOMAIN_ID_FIELD_ID.to_string(), domain_id);
            new_entity_doc.insert(GRAPHS_COMPOSE_TYPE_FIELD_ID.to_string(), compose_type);

            let result = manager
                .new_entity(&mut new_entity_doc, &account_id, &group_id)
                .await;

            match result {
                Ok(r) => Ok(Response::new(NewGraphResponse {
                    result: "ok".to_string(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
            }else {
            Err(Status::aborted("创建新区域失败"))
            }
        }
    };
}

#[macro_export]
macro_rules! declare_handle_add_link_to_graph {
    ($server:ty) => {
        pub(crate) async fn handle_add_link_to_graph(
            &self,
            request: Request<AddLinkToGraphRequest>,
        ) -> UnaryResponseResult<AddLinkToGraphResponse> {
            unimplemented!()
        }
    };
}

#[macro_export]
macro_rules! declare_handle_remove_link_from_graph {
    ($server:ty) => {
        pub(crate) async fn handle_remove_link_from_graph(
            &self,
            request: Request<RemoveLinkFromGraphRequest>,
        ) -> UnaryResponseResult<RemoveLinkFromGraphResponse> {
            unimplemented!()
        }
    };
}
