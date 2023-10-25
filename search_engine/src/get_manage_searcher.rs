use dependencies_sync::tantivy;
use dependencies_sync::tantivy::Searcher;

use super::get_manage_tantivy_index;

pub fn get_manage_searcher(manage_id: i32) -> Option<Searcher> {
    let index = get_manage_tantivy_index(manage_id);

    let reader = index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();
    let searcher = reader.searcher();
    Some(searcher)
}
