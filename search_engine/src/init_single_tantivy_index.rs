use dependencies_sync::log;

use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tantivy::{directory, Index};

use managers::get_tokenizers;

use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use crate::tantivy_index::set_tantivy_index;
use crate::{get_manage_tantivy_schema, get_tantivy_schema, get_tokenizers};

use super::{get_manage_tantivy_index_map, get_tantivy_index_dir::get_tantivy_index_dir};

pub fn init_tantivy_index() {
    log::info!("{}", t!("开始初始化索引"));

    let index_dir_string = get_tantivy_index_dir();
    let index_dir_path = Path::new(&index_dir_string);

    if !index_dir_path.exists() {
        log::info!("{}: {}", t!("创建索引目录"), index_dir_string);
        if let Err(err) = create_dir_all(index_dir_path) {
            log::error!("{}: {}", t!("创建索引目录失败"), err.to_string());
            return;
        };
    }

    let index_dir = match directory::MmapDirectory::open(&index_dir_string) {
        Ok(r) => r,
        Err(err) => {
            log::error!("{}: {}", t!("打开索引目录失败"), err.to_string());
            return;
        }
    };

    let index = match Index::open(index_dir) {
        Ok(r) => {
            log::info!("{}: {}", t!("索引目录已存在"), index_dir_string);
            r
        }
        Err(_err) => {
            log::warn!("{}: {}", t!("新建索引"), index_dir_string);
            // 在初始化管理器初始化完成
            let schema = get_tantivy_schema();

            let index_builder = Index::builder();
            match index_builder
                .schema(schema.deref().to_owned())
                .create_in_dir(&index_dir_string)
            {
                Ok(r) => {
                    log::info!("{}-{}", t!("新建索引成功"), index_dir_string);
                    r
                }
                Err(err) => {
                    log::error!("{}: {}, {}", t!("新建索引失败"), index_dir_string, err);
                    panic!("{}", t!("新建索引失败"));
                }
            }
        }
    };

    let tokenizers = get_tokenizers();
    for (k, t) in tokenizers.iter() {
        index.tokenizers().register(k, t.clone());
    }

    set_tantivy_index(Arc::new(RwLock::new(index)));

    // let index_map_arc = get_manage_tantivy_index_map();
    // let mut index_map = index_map_arc.write();

    // match Index::open(index_dir) {
    //     Ok(r) => {
    //         index_map.insert(manage_id, Arc::new(r));
    //     }
    //     Err(_err) => {
    //         log::warn!("{}: {}", t!("新建索引"), manage_id);

    //         // 在初始化管理器初始化完成
    //         let schema = get_manage_tantivy_schema(manage_id)
    //             .unwrap()
    //             .deref()
    //             .to_owned();

    //         let index_builder = Index::builder();
    //         match index_builder
    //             .schema(schema)
    //             .create_in_dir(&index_dir_string)
    //         {
    //             Ok(r) => {
    //                 // let tokenizer = CangJieTokenizer {
    //                 //     worker: Arc::new(Jieba::empty()), // empty dictionary
    //                 //     option: TokenizerOption::Unicode,
    //                 // };
    //                 // r.tokenizers().register(CANG_JIE, tokenizer);

    //                 let tokenizers = get_tokenizers();
    //                 for (k, t) in tokenizers.iter() {
    //                     r.tokenizers().register(k, t.clone());
    //                 }

    //                 log::info!("{}-{}", t!("新建索引成功"), manage_id);
    //                 index_map.insert(manage_id, Arc::new(r));

    //                 // let mut writer: IndexWriter = r.writer(15_000_000).unwrap();
    //                 // let writer_arc = get_manage_index_writer(manage_id);
    //                 // let mut writer = writer_arc.write();
    //                 // writer.commit();
    //             }
    //             Err(err) => {
    //                 log::error!("{}-{}: {}", t!("新建索引失败"), manage_id, err);
    //             }
    //         };
    //     }
    // }

    log::info!("{}", t!("初始化索引完成"));
}
