use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Range<T> {
    min: T,
    max: T,
    value: T,
}
