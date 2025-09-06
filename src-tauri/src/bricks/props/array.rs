use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bricks::props::PropType;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Array property container with typed values.")]
#[serde(tag = "value_type")]
pub enum ArrayPropType {
    String(Array<String, u32>),
    Integer(Array<i32, i32>),
    Float(Array<f32, f32>),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Array property container with typed values.")]
pub struct Array<T: Default, M: Default + PartialOrd> {
    #[serde(flatten)]
    base: PropType<Vec<T>>,

    #[schemars(description = "Minimum number of items allowed in the array.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<usize>,

    #[schemars(description = "Maximum number of items allowed in the array.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,

    #[schemars(description = "Minimum allowed value (inclusive).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<M>,

    #[schemars(description = "Maximum allowed value (inclusive).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<M>,
}
