/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-10-10 08:11
Introduction:
*/

mod check_auth_token;

pub use check_auth_token::check_auth_token;



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
