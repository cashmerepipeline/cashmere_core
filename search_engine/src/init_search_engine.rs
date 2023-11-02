use crate::{init_tantivy_index, get_tantivy_index, get_tantivy_writer, spaw_commit_thread, watch_database};

pub async fn init_search_engine() {
  let _ = init_tantivy_index();
  let _ = get_tantivy_index();
  let _ = get_tantivy_writer();
  
  watch_database().await;
  spaw_commit_thread();
}