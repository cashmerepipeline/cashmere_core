/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-10-10 08:11
Introduction:
*/

use crate::jwt;

use tonic::{Request, Status};
use chrono::{Utc};

///检查授权token
pub fn check_auth_token(request: Request<()>) -> Result<Request<()>, Status> {
    // token检查
    let auth_meta = request.metadata();
    let auth_token = match super::get_auth_token(auth_meta) {
        Some(token) => token,
        None => return Err(Status::unauthenticated("请先登录")),
    };

    //  1. token校验
    if !jwt::validate_jwt_token(&auth_token) {
        return Err(Status::unauthenticated("权限验证错误，请重新登录"));
    }

    let (_account_id, _groups) = match super::get_claims_account_and_roles(&auth_token) {
        Some(r) => r,
        None => return Err(Status::unauthenticated("请先登录")),
    };

    let claims = match super::jwt::get_claims(&auth_token) {
        Some(r) => r,
        None => return Err(Status::unauthenticated("请先登录")),
    };

    // 2. 过期校验
    let now = Utc::now().timestamp();
    if now > claims.exp as i64{
        return Err(Status::cancelled("登录已过期，请重新登录"));
    }

    // // 3. 登录限制校验, mongodb使用async-std
    // let account_doc = match get_account_by_id(&account_id)
    // {
    //     Ok(r) => r,
    //     Err(_e) => return Err(Status::data_loss("账号不存在"))
    // };
    //
    // let timestamps = match account::get_account_login_timestamps(&account_doc) {
    //     Some(r) => r,
    //     None => return Err(Status::data_loss("请先登录"))
    // };
    // for stamp in timestamps {
    //     if stamp > claims.exp as i64{
    //         return Err(Status::cancelled("登录过期，可能已在不同设备登录"));
    //     }
    // }
    //
    // // 4. 账户是否停用
    // if account::is_account_stopped(&account_doc) {
    //     return Err(Status::unauthenticated("账号已停用"));
    // }

    Ok(request)
}

// fn get_account_by_id(
//     id: &String
// ) -> Result<Document, ()>{
//     let configs = configs::get_configs();
//     let options = ClientOptions::builder()
//                     .hosts(vec![StreamAddress {
//                         hostname: configs.database.address.clone(),
//                         port: Some(configs.database.port.clone()),
//                     }])
//                     .build();
//
//                 let client = match Client::with_options(options) {
//                     Ok(r) => r,
//                     Err(_e) => panic!("连接到数据库失败"),
//                 };
//     let database = client.database(configs.database.name.as_str());
//     let collection = database.collection(ACCOUNTS_MANAGE_ID.to_string());
//
//     match collection.find_one(
//         doc! {
//             "_id":id.clone()
//         },
//         None
//     ) {
//         Ok(r) => match r{
//             Some(e) => Ok(e),
//             None => Err(())
//         },
//         Err(_e) => Err(())
//     }
// }

// 检查用户状态
// fn check_member_is_active(request: Request<()>) -> Result<Request<()>, Status> {
//     let auth_meta = request.metadata().get("authorization");
//     let auth_token = match server::metadata::get_auth_token(&auth_meta) {
//         Ok(token) => token,
//         Err(_e) => return Err(Status::unauthenticated("权限验证错误，请重新登录"))
//     };
//
//     let (user_id, user) = match super::get_claims_user(&auth_token) {
//         Some(user) => user,
//         None => return Err(Status::unauthenticated("权限验证错误，请重新登录"))
//     };
//
//     let member_database = sled::open(server::config::MEMBER_DATABASE_PATH)
//         .expect("open member database failed");
//     let user = server::member::get_member_by_phone(&user_id, &member_database)
//         .expect("fetch member from database failed");
//
//     if user.status == 0 {
//         return Err(Status::unauthenticated("账户已停用"));
//     }
//
//     Ok(request)
// }
