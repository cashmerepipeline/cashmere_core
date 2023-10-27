use std::sync::Arc;

use dependencies_sync::{
    cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE},
    jieba_rs::Jieba,
};

pub fn cangjie_tokenizer() -> CangJieTokenizer {
  CangJieTokenizer {
      worker: Arc::new(Jieba::empty()), // empty dictionary
      option: TokenizerOption::Unicode,
  }
}