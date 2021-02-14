/*
Project: minit
Creator: 闫刚
Create time: 2020-10-16 10:45
Introduction:
*/

// pub mod database;
// mod entity;

use database;
use entity;
use configs;

use clap::{App, Arg};
use bson::{ doc};
use manage_define::manage_ids::*;
use manage_define::general_field_ids::*;
use manage_define::field_ids::*;

use define_utils as utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Wav2AnimServer")
        .arg(
            Arg::new("debug")
                .about("turn on debugging information")
                .short('d'),
        )
        .args(&[
            // 数据库地址
            Arg::new("configs")
                .about("configs file path")
                .takes_value(true)
                .short('c')
                .long("configs"),
            // 指定单个文件
            Arg::new("file")
                .about("manage toml file")
                .takes_value(true)
                .short('f')
                .long("file"),
            // 指定目录
            Arg::new("directory")
                .about("toml files directory")
                .takes_value(true)
                .short('r')
                .long("directory"),
        ])
        .get_matches();

    // 没有指定则退出
    if !matches.is_present("file")
        && !matches.is_present("directory")
        && !matches.is_present("configs")
    {
        panic!("需要指定项目配置文件、定义文件或者包含定义文件的目录");
    }

    if let Some(cfg_path) = matches.value_of("configs"){
        configs::init_configs_path(cfg_path.to_string()).expect("初始化设置文件路径失败");
    }

    // 数据库检查
    let db = database::get_cashmere_database().await;
    let db_name = db.name();
    println!("连接到数据库：{}", db_name);

    // 文件列表
    let mut toml_pathes: Vec<String> = vec![];
    print!("------开始读取定义文件------\n");
    // 添加单文件
    if let Some(path) = matches.value_of("file") {
        toml_pathes.push(path.to_string());
    }
    // 添加目录
    if let Some(path) = matches.value_of("directory") {
        let mut tomls = utils::get_toml_files_of_dir(&path.to_string()).unwrap();
        toml_pathes.append(&mut tomls);
    }

    // 读入所有文件并构造toml映射
    let tomls = utils::get_tomls_from_pathes(&toml_pathes).unwrap();
    print!("------读取定义文件完成-----\n\n");


    // 使用root用户和root组初始化管理数据库
    let root_id = &"86100000000000".to_string();
    let root_group_id = &"1000000".to_string();

    // 1. 创建管理集合
    match db.create_collection(&MANAGES_MANAGE_ID.to_string(), None).await{
        Err(e) => println!("创建管理集合失败: {} {:?}", MANAGES_MANAGE_ID, e),
        _ => println!("创建管理成功 {}", MANAGES_MANAGE_ID),
    }

    println!("------创建管理-------");
    for map in &tomls {
        let manage_id = match utils::get_id(&map) {
            Some(m) => m.to_string(),
            None => continue,
        };
        // println!("开始创建管理：{}", manage_id);
        let manage_name = match utils::get_name(&map) {
            Some(m) => m,
            None => continue,
        };
        let manage_schema = match utils::get_schema(&map) {
            Some(s) => s,
            None => continue,
        };

        println!("开始创建管理：{} {}", manage_id, manage_name);

        let mut manage_doc = doc! {
            "_id": manage_id.clone(),
            ID_FIELD_ID.to_string(): manage_id.clone(),
            NAME_FIELD_ID.to_string(): manage_name.clone(),
            MANAGES_SCHEMA_FIELD_ID.to_string(): manage_schema
        };

        // let collection = db.collection(&cash_core::MANAGES_MANAGE_ID.to_string());
        // 创建实体入库
        match entity::insert_entity(
            &MANAGES_MANAGE_ID.to_string(),
            &mut manage_doc,
            &root_id,
            root_group_id,
        ).await {
            Ok(r) => {
                println!("添加管理实体 {} {} 成功", manage_id, manage_name);
                Some(r)
            }
            Err(_e) => {
                println!("添加管理实体 {} {} 失败 {}", manage_id, manage_name, _e.details());
                None
            }
        };

        // 管理集合已经创建
        if manage_id == MANAGES_MANAGE_ID.to_string() {
            continue
        }

        // 创建集合
        match db.create_collection(&manage_id.clone(), None).await{
            Err(e) => println!("创建管理集合失败: {} {:?}", manage_id, e),
            _ => println!("创建管理成功 {}", manage_id),
        }
    }
    println!("------创建管理完成-------");

    // 2. 创建队列集合
    println!("------创建队列-------");
    for map in &tomls {
        let queues = match utils::get_queues(&map) {
            Some(q) => q,
            None => continue,
        };
        for q in queues {
            db.create_collection(q.as_str(), None).await
                .expect("创建管理集合失败");
        }
    }
    println!("------创建队列完成-------");

    // 3. 添加初始实体数据
    println!("------开始插入初始数据-------");
    for map in &tomls {
        let manage_id = utils::get_id(&map).unwrap();
        // let collection = db.collection(&id.to_string());
        if let Some(items) = utils::get_init_items(&map) {
            for mut item in items {
                if let Ok(_r) =
                    entity::insert_entity(&manage_id.to_string(), &mut item, root_id, root_group_id).await
                {
                    continue;
                } else {
                    println!("插入记录失败, {}", item);
                }
            }
        }
    }
    println!("------插入初始数据结束-------");

    // 4. 添加映像规则
    println!("------开始添加映像规则-------");
    for map in &tomls {
        let rule_id = utils::get_id(&map).unwrap();
        let rule_name = 
            match utils::get_name(&map){
            Some(m) => m,
            None => continue,
        };

        // let collection = db.collection(&collect_name);

        let view_rules = match utils::get_init_view_rules(&map) {
            Some(m) => m,
            None => continue,
        };

        let mut rulse_doc = doc! {
            "_id": rule_id.to_string(),
            ID_FIELD_ID.to_string(): rule_id.to_string(),
            NAME_FIELD_ID.to_string(): rule_name.clone(),
            VIEW_RULES_MANAGE_FIELD_ID.to_string(): bson::to_bson(&view_rules.manage).unwrap(),
            // VIEW_RULES_COLLECTION_FIELD_ID.to_string(): bson::to_bson(&view_rules.entity).unwrap(),
            VIEW_RULES_ENTITY_FIELD_ID.to_string(): bson::to_bson(&view_rules.schema).unwrap(),
        };

        match entity::insert_entity(
            &VIEW_RULES_MANAGE_ID.to_string(),
            &mut rulse_doc,
            root_id,
            root_group_id,
        ).await{
            Ok(_r) => continue,
            Err(e) => println!("{} {}", e.operation(), e.details()),
        }
    }
    println!("------添加映像规则完成-------");
    Ok(())
}
