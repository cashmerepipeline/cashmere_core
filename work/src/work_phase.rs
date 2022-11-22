use serde::{Deserialize, Serialize};

/// 过程阶段
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkPhase{
    pub index: i32,
    pub name: String,
}

