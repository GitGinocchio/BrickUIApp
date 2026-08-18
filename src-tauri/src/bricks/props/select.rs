use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::bricks::props::PropSpec;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Selectable property container with typed options.")]
#[serde(tag = "value_type")]
pub enum SelectPropKind {
    String(SelectPropSpec<String, u32>),
    Int(SelectPropSpec<i32, i32>),
    Float(SelectPropSpec<i32, i32>),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Selectable property container with typed options.")]
pub struct SelectPropSpec<T, M: PartialOrd> {
    #[serde(flatten)]
    base: PropSpec<Vec<T>>,

    #[schemars(description = "List of selectable options.")]
    pub options: Vec<T>,

    #[schemars(description = "Minimum number of selections allowed.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<usize>,

    #[schemars(description = "Maximum number of selections allowed.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,

    #[schemars(description = "Minimum allowed value (inclusive).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<M>,

    #[schemars(description = "Maximum allowed value (inclusive).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<M>,
}
