use dependencies_sync::log;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tantivy::{directory, Index, IndexWriter, Document as TantivyDocument};
use dependencies_sync::{
    cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE},
    jieba_rs::Jieba,
};

use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use crate::cang_jie_tokenizer::cangjie_tokenizer;
use crate::{get_manage_tantivy_schema, get_manage_index_writer};

use super::{get_manage_tantivy_index_map, get_tantivy_index_dir::get_tantivy_index_dir};

pub fn init_tantivy_index(manage_id: i32) {
    log::info!("{}: {}", t!("开始初始化索引"), manage_id);

    let index_dir_string = get_tantivy_index_dir(manage_id);
    let index_dir_path = Path::new(&index_dir_string);

    if !index_dir_path.exists() {
        log::info!("{}: {}", t!("创建索引目录"), index_dir_string);
        create_dir_all(&index_dir_path);
    }

    let index_dir = match directory::MmapDirectory::open(&index_dir_string) {
        Ok(r) => r,
        Err(err) => {
            log::error!("{}: {}", t!("打开索引目录失败"), err.to_string());
            return;
        }
    };

    let index_map_arc = get_manage_tantivy_index_map();
    let mut index_map = index_map_arc.write();

    match Index::open(index_dir) {
        Ok(r) => {
            index_map.insert(manage_id, Arc::new(r));
        }
        Err(err) => {
            log::warn!("{}: {}", t!("新建索引"), manage_id);

            // 在初始化管理器初始化完成
            let schema = get_manage_tantivy_schema(manage_id)
                .unwrap()
                .deref()
                .to_owned();

            let index_builder = Index::builder();
            match index_builder
                .schema(schema)
                .create_in_dir(&index_dir_string)
            {
                Ok(r) => {
                    let tokenizer = CangJieTokenizer {
                        worker: Arc::new(Jieba::empty()), // empty dictionary
                        option: TokenizerOption::Unicode,
                    };

                    r.tokenizers().register(CANG_JIE, tokenizer); 

                    log::info!("{}-{}", t!("新建索引成功"), manage_id);
                    index_map.insert(manage_id, Arc::new(r));

                    // let mut writer: IndexWriter = r.writer(15_000_000).unwrap();
                    // let writer_arc = get_manage_index_writer(manage_id);
                    // let mut writer = writer_arc.write();
                    // writer.commit();
                }
                Err(err) => {
                    log::error!("{}-{}: {}", t!("新建索引失败"), manage_id, err);
                }
            };
        }
    }

    log::info!("{}: {}", t!("初始化索引完成"), manage_id);
}
