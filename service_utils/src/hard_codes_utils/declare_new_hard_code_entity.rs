#[macro_export]
/// 通用硬编码实体创建函数
macro_rules! declare_new_hard_code_entity {
    ( $req:ident, $res:ident, $manage_id:ident) => {
        async fn new_hard_code_entity(request: Request<$req>) -> Result<Response<$res>, Status> {
            let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

            let code = &request.get_ref().code;
            let name = &request.get_ref().name;
            let description = &request.get_ref().description;

            let name = name.as_ref().unwrap();
            let name_doc = doc! {name.language.clone():name.name.clone()};

            let majordomo_arc = get_majordomo();
            let manager = majordomo_arc.get_manager_by_id($manage_id).unwrap();

            // 编码不能重复
            let query_doc = doc! {ID_FIELD_ID.to_string():code};
            if manager.entity_exists(&query_doc).await.is_some() {
                return Err(Status::invalid_argument(format!(
                    "{}: {}",
                    t!("编码已存在"),
                    code
                )));
            }

            let mut new_entity_doc = make_new_entity_document(&manager, &account_id).await?;
            new_entity_doc.insert(ID_FIELD_ID.to_string(), code);
            new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
            new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), bson::to_document(description).unwrap());

            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(r) => Ok(Response::new($res { result: r })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        }
    };
}
