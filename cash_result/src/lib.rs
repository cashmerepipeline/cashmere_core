use dependencies_sync::rust_i18n::{self, i18n, t};
i18n!("locales");

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

/// 操作成功
pub fn operation_succeed(result: impl Into<String>) -> OperationResult {
    OperationResult::Succeed(result.into())
}

/// 集合不存在
pub fn collection_not_exists(collection: &str, operation: impl Into<String>) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("{}: {}", t!("集合不存在"), collection),
    };

    OperationResult::Failed(result)
}

/// 实体不存在
pub fn entity_not_exists(
    operation: impl Into<String>,
    entity: impl Into<String>,
) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("{}: {}", t!("实体不存在"), entity.into()),
    };

    OperationResult::Failed(result)
}

/// 实体属性不存在
pub fn field_not_exists(operation: impl Into<String>, field: impl Into<String>) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("{}: {}", t!("不能取得属性"), field.into()),
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
        details: format!("{}: {}", t!("属性已经是想要的值"), field.into()),
    };

    OperationResult::Failed(result)
}

/// 目标已存在
pub fn target_already_exists(target: &str, operation: impl Into<String>) -> OperationResult {
    let result = Failed {
        operation: operation.into(),
        details: format!("{}: {}",  t!("目标已经存在"), target),
    };

    OperationResult::Failed(result)
}

/// 操作失败
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
