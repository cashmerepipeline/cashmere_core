use cang_jie::CANG_JIE;
use tantivy::schema::{IndexRecordOption, TextFieldIndexing, TextOptions};

pub fn get_text_options() -> TextOptions {
    let text_indexing = TextFieldIndexing::default()
        .set_tokenizer(CANG_JIE) // Set custom tokenizer
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);

    

    TextOptions::default()
        .set_indexing_options(text_indexing)
        .set_stored()
}
