/*
使用单例模式创建数据库 client
所有操作只使用一个client, 需要进一步测试
*/

use mongodb::bson::{doc, Document};
use mongodb::{options::ClientOptions, options::ServerAddress, Client, Collection, Database};
use std::sync::Arc;

use log;

// use  manage_define::manage_ids::MANAGES_MANAGE_ID;
use cash_result::{operation_failed, operation_succeed, OperationResult};

use manage_define::manage_ids::{IDS_MANAGE_ID, MANAGES_MANAGE_ID};

pub type MongodbResult<T> = mongodb::error::Result<T>;

static mut MONGODB_CLIENT: Option<Arc<Client>> = None;
static mut CASHMERE_DATABASE: Option<Arc<Database>> = None;

/// 取得客户端
pub async fn get_mongodb_client() -> &'static Client {
    unsafe {
        if MONGODB_CLIENT.is_some() {
            MONGODB_CLIENT.as_ref().unwrap()
        } else {
            let configs = configs::get_configs();

            MONGODB_CLIENT.get_or_insert_with(|| {
                let options = ClientOptions::builder()
                    .hosts(vec![ServerAddress::Tcp {
                        host: configs.database.address.clone(),
                        port: Some(configs.database.port),
                    }])
                    .build();

                let client = match Client::with_options(options) {
                    Ok(r) => r,
                    Err(_e) => panic!("连接到数据库失败"),
                };
                Arc::new(client)
            });
            MONGODB_CLIENT.as_ref().unwrap()
        }
    }
}

/// 根据设置文件，取得数据库
pub async fn get_cashmere_database() -> &'static Database {
    unsafe {
        if CASHMERE_DATABASE.is_some() {
            CASHMERE_DATABASE.as_ref().unwrap()
        } else {
            let configs = configs::get_configs();

            CASHMERE_DATABASE.get_or_insert_with(|| {
                let options = ClientOptions::builder()
                    .hosts(vec![ServerAddress::Tcp {
                        host: configs.database.address.clone(),
                        port: Some(configs.database.port),
                    }])
                    .build();

                let client = match Client::with_options(options) {
                    Ok(r) => r,
                    Err(_e) => panic!("连接到数据库失败"),
                };
                Arc::new(client.database(configs.database.name.as_str()))
            });

            CASHMERE_DATABASE.as_ref().unwrap()
        }
    }
}

/// 集合是否存在
pub async fn collection_exists(collection: &String) -> bool {
    let db = get_cashmere_database().await;
    let collections = db.list_collection_names(None).await.unwrap();

    collections.contains(collection)
}

/// 取得集合
pub async fn get_collection_by_id(manage_id: &String) -> Option<Collection<Document>> {
    let cashmere_db = get_cashmere_database().await;

    // 不存在
    if !collection_exists(manage_id).await {
        return None;
    }

    Some(cashmere_db.collection(manage_id))
}

/// 取得管理-管理集合, 不存在则新建
pub async fn get_manages_collection() -> Collection<Document> {
    let cashmere_db = get_cashmere_database().await;

    let manages_id = &MANAGES_MANAGE_ID.to_string();

    // manages 不存在则创建
    if !collection_exists(manages_id).await {
        cashmere_db
            .create_collection(manages_id, None)
            .await
            .expect("创建管理失败");
    }

    cashmere_db.collection(manages_id)
}

/// 取得编号-管理集合, 不存在则新建
pub async fn get_ids_collection() -> Collection<Document> {
    let cashmere_db = get_cashmere_database().await;

    let manages_id = &IDS_MANAGE_ID.to_string();

    // manages 不存在则创建
    if !collection_exists(manages_id).await {
        cashmere_db
            .create_collection(manages_id, None)
            .await
            .expect("创建管理失败");
    }

    cashmere_db.collection(manages_id)
}

/// 初始化实体编号字段
pub async fn init_ids_count_field(manage_id: &String) -> Result<OperationResult, OperationResult> {
    let ids_collection = get_ids_collection().await;

    let f_result = ids_collection
        .find_one(
            doc! {
                "_id": manage_id.clone()
            },
            None,
        )
        .await;

    if let Ok(r) = f_result {
        if r.is_none() {
            match ids_collection
                .insert_one(
                    doc! {
                        "_id": manage_id.clone(),
                        "id_count": 0i32
                    },
                    None,
                )
                .await
            {
                Ok(_r) => (),
                Err(_e) => {
                    return Err(operation_failed(
                        "init_ids_count_field",
                        "初始化序列号记录失败",
                    ))
                }
            };
        } else {
            log::info! {"序号生成器记录已存在 {}", manage_id};
            return Ok(operation_succeed("already exits"));
        }
    } else {
        return Err(operation_failed(
            "init_ids_count_field",
            "初始化序列号记录失败",
        ));
    }

    Ok(operation_succeed("ok"))
}

#[cfg(test)]
mod tests {
    use crate::get_cashmere_database;
    use mongodb::bson::doc;
    use tokio_test::assert_ok;

    #[test]
    fn test_database() {
        let db = tokio_test::block_on(get_cashmere_database());
        tokio_test::block_on(db.create_collection("test", None)).expect("创建测试集合失败");
        let collection = db.collection("test");
        let doc = doc! {
            "name": "Test"
        };
        let result = tokio_test::block_on(collection.insert_one(doc.clone(), None));
        assert_ok!(result);
        let f_doc = tokio_test::block_on(collection.find_one(doc.clone(), None))
            .unwrap()
            .unwrap();
        assert_eq!(doc.get_str("name").unwrap(), f_doc.get_str("name").unwrap());
    }
}
