use bson::Document;

use auth::jwt::hash_password;
use manage_define::field_ids::ACCOUNTS_PASSWORD_FIELD_ID;
use manage_define::general_field_ids::ID_FIELD_ID;
use manage_define::manage_ids::ACCOUNTS_MANAGE_ID;

pub async fn init_root_password(root_id: &String, passswd: &Option<String>) {
    let hashed_passwd = if passswd.is_some() {
        hash_password(passswd.as_ref().unwrap()).await.unwrap()
    } else {
        hash_password(&"root".to_string()).await.unwrap()
    };

    let mut query_doc = Document::new();
    query_doc.insert(ID_FIELD_ID.to_string(), root_id.clone());

    let mut modify_doc = Document::new();
    modify_doc.insert(ACCOUNTS_PASSWORD_FIELD_ID.to_string(), hashed_passwd);

    match entity::update_entity_field(
        &ACCOUNTS_MANAGE_ID.to_string(),
        query_doc,
        modify_doc,
        root_id,
    )
    .await
    {
        Err(r) => println!("{}", r.details()),
        _ => {}
    };
}
