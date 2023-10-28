use dependencies_sync::cang_jie::CANG_JIE;
use dependencies_sync::tantivy::schema::{IndexRecordOption, TextFieldIndexing, TextOptions};

pub fn get_text_options() -> TextOptions {
    let text_indexing = TextFieldIndexing::default()
        .set_tokenizer(CANG_JIE) // Set custom tokenizer
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    let text_options = TextOptions::default()
        .set_indexing_options(text_indexing)
        .set_stored();

    text_options
}