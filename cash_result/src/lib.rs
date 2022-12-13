/*
Author: 闫刚 (yes7rose@sina.com)
results.rs (c) 2020
Desc: 返回结果
Created:  2020-11-24T10:33:37.591Z
Modified: !date!
*/

#[derive(Debug)]
pub struct Failed {
    pub operation: String,
    pub details: String,
}

#[derive(Debug)]
pub enum OperationResult {
    Succeed(String),
    Failed(Failed),
}

impl OperationResult {
    pub fn operation(&self) -> String {
        match self {
            OperationResult::Succeed(r) => r.clone(),
            OperationResult::Failed(e) => e.operation.clone(),
        }
    }

    pub fn details(&self) -> String {
        match self {
            OperationResult::Succeed(r) => r.clone(),
            OperationResult::Failed(e) => e.details.clone(),
        }
    }
}

// 操作成功
pub fn operation_succeed(result: impl Into<String>) -> OperationResult {
    OperationResult::Succeed(result.into())
}

// 集合不存在
pub fn collection_not_exists(operation: impl Into<String>) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: "集合不存在".to_string(),
    };

    OperationResult::Failed(result)
}

// 实体不存在
pub fn entity_not_exists(
    operation: impl Into<String>,
    entity: impl Into<String>,
) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("实体不存在: {}", entity.into()),
    };

    OperationResult::Failed(result)
}

/// 实体属性不存在
pub fn field_not_exists(operation: impl Into<String>, field: impl Into<String>) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("不能取得属性: {}", field.into()),
    };

    OperationResult::Failed(result)
}
/// 属性不需要更新
pub fn field_edited_already(
    operation: impl Into<String>,
    field: impl Into<String>,
) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("属性已经是想要的值: {}", field.into()),
    };

    OperationResult::Failed(result)
}

/// 目标已存在
pub fn target_already_exists(operation: impl Into<String>) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: "目标已经存在".to_string(),
    };

    OperationResult::Failed(result)
}

// 操作失败
pub fn operation_failed(
    operation: impl Into<String>,
    details: impl Into<String>,
) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: details.into(),
    };

    OperationResult::Failed(result)
}

// 添加调用链环节
pub fn add_call_name_to_chain(e: OperationResult, new_operation: String) -> OperationResult {
    let mut new_op = e.operation();
    new_op.insert_str(0, " -> ");
    new_op.insert_str(0, new_operation.as_str());

    OperationResult::Failed(Failed {
        operation: new_op,
        details: e.details(),
    })
}
