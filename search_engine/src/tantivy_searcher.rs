use dependencies_sync::log;
use tantivy::Searcher;

use crate::get_tantivy_index;

pub fn get_tantivy_searcher() -> Option<Searcher> {
    let index_arc = get_tantivy_index();
    let index = index_arc.read();

    let reader = match index
        .reader_builder()
        .reload_policy(tantivy::ReloadPolicy::OnCommitWithDelay)
        .try_into()
    {
        Ok(reader) => reader,
        Err(err) => {
            log::error!("Failed to get tantivy searcher: {}", err);
            return None;
        }
    };

    let searcher = reader.searcher();
    Some(searcher)
}
