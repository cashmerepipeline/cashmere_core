/*
Author: 闫刚 (yes7rose@sina.com)
macros.rs (c) 2020
Desc: 宏声明
Created:  2020-12-02T14:29:51.128Z
Modified: !date!
*/



#[macro_export]
macro_rules! declare_default_get_manage {
    ($static_manage:ident, $id:ident) => {
        // 取得管理器
        async fn get_manage(&self) -> Arc<RwLock<Manage>> {
            unsafe {
                if $static_manager.is_some() {
                    $static_manager.as_ref().unwrap()
                } else {
                    let manages_collection_name = MANAGES_MANAGE_ID.to_string();
                    let id_str = $id.to_string();
                    let m_doc = match entity::get_entity_by_id(&manages_collection_name, &id_str).await {
                        Ok(r) => r,
                        Err(e) => panic!("{} {}", e.operation(), e.details()),
                    };

                    let manage: Manage = manage_from_document(m_doc).unwrap();
                    $static_manager.replace(Arc::new(RwLock::new(manage)));
                    $static_manager.clone().unwrap()
                }
            }
        }
    }
}

// #[macro_export]
// macro_rules! declare_get_manage_document {
//     ($manage:ty, $static_manage:ident) => {
//         // 取得管理器
//         pub(crate) fn get_manage() -> Arc<Manager> {
//             unsafe {
//                 if $static_manage.is_none() {
//                     let m_object = <$manage>::default();
//                     $static_manage.replace(Arc::new(Manager {
//                         inner: Arc::new(ManagerInner {
//                             manage: Arc::new(m_object),
//                         }),
//                     }));
//                     $static_manage.clone().unwrap()
//                 } else {
//                     $static_manage.clone().unwrap()
//                 }
//             }
//         }
//     }
// }


// #[macro_export]
// macro_rules! declare_get_manage {
//     ($life_:lifetime, $static_manager:ident, $id:ident) => {
//         // 取得管理
//         async fn get_manage(&$life_ self) -> Arc<RwLock<Manage>> {
//             unsafe {
//                 if $static_manager.is_some() {
//                     $static_manager.as_ref().unwrap()
//                 } else {
//                     let collection_name = $id.to_string();
//                     let id_str = $id.to_string();
//                     let m_doc = match entity::get_entity_by_id(&collection_name, &id_str, &[], &[]).await {
//                         Ok(r) => r,
//                         Err(e) => panic!("{} {}", e.operation(), e.details())),
//                     };
//                     let manage: Manage = manage_from_document(m_doc).unwrap(;
//                     $static_manager.replace(Arc::new(RwLock::new(manage)));
//                     $static_manager.clone().unwrap()
//                 }
//             }
//         }
//     }
// }
