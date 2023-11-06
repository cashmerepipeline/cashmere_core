use crate::{
    get_tantivy_index, get_tantivy_writer, init_tantivy_index,
    spaw_commit_thread::spaw_commit_thread, watch_database,
};

pub async fn init_search_engine() {
    init_tantivy_index();
    let _ = get_tantivy_index();
    let _ = get_tantivy_writer();

    watch_database().await;
    let _ = spaw_commit_thread();
}
