#[macro_export]
macro_rules! declare_handle_get_manages {
    ($server:ty) => {
        impl $server {
            /// 取得管理
            pub(crate) async fn handle_get_manages(
                &self,
                request: Request<GetManagesRequest>,
            ) -> StreamResponseResult<EntityResponseStream> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                let managers_ids: Vec<i32> = get_majordomo().await.get_manager_ids().await;

                let (mut tx, rx) = mpsc::channel(4);

                tokio::spawn(async move {
                    for id in managers_ids {
                        let manager = get_majordomo().await.get_manager_by_id(id).await.unwrap();
                        let doc = manager.get_manage_document().await.read().clone();
                        let mut data: Vec<u8> = Vec::new();
                        doc.to_writer(&mut data).unwrap();
                        tx.send(Ok(Entity { data: data })).await.unwrap();
                    }
                });

                Ok(Response::new(rx))
            }
        }
    };
}