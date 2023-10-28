use std::{collections::HashMap, sync::Arc};

use dependencies_sync::{
    cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE},
    jieba_rs::Jieba,
    tantivy::tokenizer::Tokenizer,
};

pub fn get_tokenizers() -> HashMap<&'static str, impl Tokenizer> {
    let mut tokenizers = HashMap::new();

    let cangjie = CangJieTokenizer {
        worker: Arc::new(Jieba::empty()), // empty dictionary
        option: TokenizerOption::Unicode,
    };

    tokenizers.insert(CANG_JIE, cangjie);
    tokenizers
}
