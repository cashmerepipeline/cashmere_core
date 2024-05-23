#[macro_export]
macro_rules! declare_common_manager_interface {
    ($manager_type:ty, $manage:ident, $manage_doc: ident,  $manage_id: ident) => {
        /// Common interface for all managers.
        #[async_trait]
        impl ManagerInterface for $manager_type {
            fn unregister(&self) -> Result<OperationResult, OperationResult> {
                Err(operation_failed(
                    "unregister",
                    format!(
                        "{}-{}-{}",
                        t!("管理不能被注销"),
                        self.get_id(),
                        self.get_name()
                    ),
                ))
            }

            fn get_id(&self) -> &'static str {
                $manage_id
            }

            fn get_name(&self) -> String {
                stringify!($manager_type).to_string()
            }

            async fn get_manage(&self) -> Arc<RwLock<Manage>> {
                match $manage.get() {
                    Some(r) => r.clone(),
                    None => {
                        // 管理的管理
                        let id_str = $manage_id.to_string();
                        let m_doc =
                            match entity::get_entity_by_id(&MANAGES_MANAGE_ID.to_string(), &id_str, &[], &[])
                                .await
                            {
                                Ok(r) => r,
                                Err(e) => panic!("{} {}", e.operation(), e.details()),
                            };

                        let manage: Manage = manage_from_document(m_doc).unwrap();
                        let ma = Arc::new(RwLock::new(manage));
                        if let Err(_err) = $manage.set(ma.clone()) {
                            error!("{} {}", t!("初始化管理缓存失败"), $manage_id);
                            panic!("{} {}", t!("初始化管理缓存失败"), $manage_id);
                        };

                        ma
                    }
                }
            }

            async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
                match $manage_doc.get() {
                    Some(r) => r.clone(),
                    None => {
                        // 管理的管理
                        let id_str = $manage_id.to_string();
                        let m_doc =
                            match entity::get_entity_by_id( &MANAGES_MANAGE_ID, &id_str, &[], &[])
                                .await
                            {
                                Ok(r) => r,
                                Err(e) => panic!("{} {}", e.operation(), e.details()),
                            };

                        let ma = Arc::new(RwLock::new(m_doc));

                        if let Err(_err) = $manage_doc.set(ma.clone()) {
                            error!("{} {}", t!("初始化管理文档缓存失败"), $manage_id);
                            panic!("{} {}", t!("初始化管理文档缓存失败"), $manage_id);
                        };

                        ma
                    }
                }
            }
        }
    };
}
