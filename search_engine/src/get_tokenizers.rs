use std::{collections::HashMap, sync::Arc};

use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use tantivy::tokenizer::Tokenizer;

pub fn get_tokenizers() -> HashMap<&'static str, impl Tokenizer> {
    let mut tokenizers = HashMap::new();

    let cangjie = CangJieTokenizer {
        worker: Arc::new(Jieba::empty()), // empty dictionary
        option: TokenizerOption::Unicode,
    };

    tokenizers.insert(CANG_JIE, cangjie);
    tokenizers
}
