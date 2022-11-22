use async_trait::async_trait;
use tonic::{Request, Response, Status};
use bson::Document;
use chrono::Utc;
use manage_define::field_ids::{PERSONS_DEPARTMENTS_FIELD_ID, PERSONS_ORGANIZATIONS_FIELD_ID};
use manage_define::manage_ids::{ACCOUNTS_MANAGE_ID, PERSONS_MANAGE_ID};
use managers::traits::ManagerTrait;
use crate::{LoginRequest, LoginResponse, UnaryResponseResult};

#[async_trait]
pub trait HandleLogin {
    async fn handle_login(
        &self,
        request: Request<LoginRequest>,
    ) -> UnaryResponseResult<LoginResponse> {
        let country_code = &request.get_ref().country_code;
        let phone = &request.get_ref().phone;
        let password = &request.get_ref().password;

        info!("account try login: {}", phone);

        // 取得账户记录
        let id: String = format!("{}{}", country_code, phone);

        let mut doc_op: Option<Document> = None;
        {
            let majordomo_lock_arc = majordomo::get_majordomo().await;
            let manager_arc = majordomo_lock_arc
                // .read()
                .get_manager_by_id(ACCOUNTS_MANAGE_ID)
                .await
                .unwrap();

            let account_doc = manager_arc.get_entity_by_id(&id.clone()).await;
            let account_doc = match account_doc {
                Ok(d) => d,
                Err(e) => {
                    return Err(Status::data_loss(format!(
                        "取得账户数据错误{} {}",
                        e.operation(),
                        e.details()
                    )));
                }
            };
            doc_op.replace(account_doc);
        }

        let account_doc = doc_op.unwrap();

        // 验证密码
        let password_hash = match account::get_account_passwd_hash(&account_doc) {
            Some(d) => d,
            None => return Err(Status::data_loss("取得账户密码错误")),
        };
        let pw_ok = (auth::jwt::verify_passwd(password, &password_hash).await).unwrap_or(false);
        if !pw_ok {
            return Err(Status::permission_denied("用户名或者密码错误"));
        }

        // 个人信息
        let mut person_bytes: Vec<u8> = Vec::new();
        let person_result = entity::get_entity_by_id(&PERSONS_MANAGE_ID.to_string(), &id).await;
        let person_doc = match person_result {
            Ok(p) => p,
            Err(_e) => return Err(Status::data_loss("取得个人信息失败")),
        };

        let orgnizations: Vec<String> = bson::from_bson(
            person_doc
                .get(PERSONS_ORGANIZATIONS_FIELD_ID.to_string().as_str())
                .unwrap()
                .clone(),
        )
        .unwrap();
        let departments: Vec<String> = bson::from_bson(
            person_doc
                .get(PERSONS_DEPARTMENTS_FIELD_ID.to_string().as_str())
                .unwrap()
                .clone(),
        )
        .unwrap();

        person_doc
            .to_writer(&mut person_bytes)
            .expect("转换记录到bytes失败");

        // 构造token
        let groups = match account::get_account_groups(&account_doc) {
            Some(a) => a,
            None => return Err(Status::data_loss("取得group数据失败")),
        };

        let token = match auth::jwt::gen_jwt_token(&id, phone, &orgnizations, &departments, &groups)
            .await
        {
            Some(t) => t,
            None => return Err(Status::data_loss("取得token数据失败")),
        };

        // 更新登录时间点
        let now = Utc::now().timestamp();
        let timestamps = match account::get_account_login_timestamps(&account_doc) {
            Some(r) => r.clone(),
            None => return Err(Status::data_loss("获取时间戳失败")),
        };
        match account::update_account_login_timestamps(&id, &timestamps, now).await {
            Ok(_) => (),
            Err(_e) => return Err(Status::data_loss("更新时间戳失败")),
        };

        info!("account login succeed: {}", phone);

        // 返回
        Ok(Response::new(LoginResponse {
            person: person_bytes,
            token,
        }))
    }
}
