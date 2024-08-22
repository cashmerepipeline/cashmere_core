use dependencies_sync::bson::{self, oid::ObjectId};

pub fn get_new_oid() -> ObjectId {
    dependencies_sync::mongodb::bson::oid::ObjectId::new()
    // bson::oid::ObjectId::new().to_string()
}
