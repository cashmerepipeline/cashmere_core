/*
Project: cashmere_server
Creator: 闫刚
Create time: 2020-10-16 10:45
Introduction:
*/

// mod database;
mod define;
// mod entity;

use database;
use entity;

use clap::{App, Arg};
use bson::{ doc};
use cash_core::{ids, field};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Wav2AnimServer")
        .arg(
            Arg::new("debug")
                .about("turn on debugging information")
                .short('d'),
        )
        .args(&[
            // // 数据库地址
            // Arg::new("address")
            //     .about("db address")
            //     .takes_value(true)
            //     .short('a')
            //     .long("address"),
            // // 数据库端口
            // Arg::new("port")
            //     .about("manage toml file")
            //     .takes_value(true)
            //     .short('p')
            //     .long("port"),
            // // 数据库端口
            // Arg::new("name")
            // .about("database name")
            // .takes_value(true)
            // .short('n')
            // .long("name"),
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
    if !matches.is_present("file") && !matches.is_present("directory") {
        panic!("需要指定定义文件或者包含定义文件的目录");
    }

    // let address = matches.value_of("address").unwrap().to_string();
    // let port: u16 = matches.value_of("port").unwrap().parse().unwrap();
    // let name = matches.value_of("name").unwrap().to_string();

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
        let mut tomls = define::get_toml_files_of_dir(&path.to_string()).unwrap();
        toml_pathes.append(&mut tomls);
    }

    // 读入所有文件并构造toml映射
    let tomls = define::get_tomls_from_pathes(&toml_pathes).unwrap();
    print!("------读取定义文件完成-----\n\n");


    // 使用root用户和root组初始化管理数据库
    let root_id = &"86100000000000".to_string();
    let root_group_id = &"1000000".to_string();

    // 1. 创建管理集合
    match db.create_collection(&ids::MANAGES_MANAGE_ID.to_string(), None).await{
        Err(e) => println!("创建管理集合失败: {} {:?}", ids::MANAGES_MANAGE_ID, e),
        _ => println!("创建管理成功 {}", ids::MANAGES_MANAGE_ID),
    }

    println!("------创建管理-------");
    for map in &tomls {
        let manage_id = match define::get_id(&map) {
            Some(m) => m.to_string(),
            None => continue,
        };
        // println!("开始创建管理：{}", manage_id);
        let manage_name = match define::get_name(&map) {
            Some(m) => m,
            None => continue,
        };
        let manage_schema = match define::get_schema(&map) {
            Some(s) => s,
            None => continue,
        };

        println!("开始创建管理：{} {}", manage_id, manage_name);

        let mut manage_doc = doc! {
            "_id": manage_id.clone(),
            field::ids::ID_FIELD_ID.to_string(): manage_id.clone(),
            field::ids::NAME_FIELD_ID.to_string(): manage_name.clone(),
            field::ids::MANAGES_SCHEMA_FIELD_ID.to_string(): manage_schema
        };

        // let collection = db.collection(&cash_core::ids::MANAGES_MANAGE_ID.to_string());
        // 创建实体入库
        match entity::insert_entity(
            &ids::MANAGES_MANAGE_ID.to_string(),
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
        if manage_id == ids::MANAGES_MANAGE_ID.to_string() {
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
        let queues = match define::get_queues(&map) {
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
        let manage_id = define::get_id(&map).unwrap();
        // let collection = db.collection(&id.to_string());
        if let Some(items) = define::get_init_items(&map) {
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
        let rule_id = define::get_id(&map).unwrap();
        let rule_name = 
            match define::get_name(&map){
            Some(m) => m,
            None => continue,
        };

        // let collection = db.collection(&collect_name);

        let view_rules = match define::get_init_view_rules(&map) {
            Some(m) => m,
            None => continue,
        };

        let mut rulse_doc = doc! {
            "_id": rule_id.to_string(),
            field::ids::ID_FIELD_ID.to_string(): rule_id.to_string(),
            field::ids::NAME_FIELD_ID.to_string(): rule_name.clone(),
            field::ids::VIEW_RULES_MANAGE_FIELD_ID.to_string(): bson::to_bson(&view_rules.manage).unwrap(),
            field::ids::VIEW_RULES_COLLECTION_FIELD_ID.to_string(): bson::to_bson(&view_rules.entity).unwrap(),
            field::ids::VIEW_RULES_ENTITY_FIELD_ID.to_string(): bson::to_bson(&view_rules.schema).unwrap(),
        };

        match entity::insert_entity(
            &ids::VIEW_RULES_MANAGE_ID.to_string(),
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