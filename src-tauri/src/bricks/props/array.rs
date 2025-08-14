use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bricks::props::PropType;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Array property container with typed values.")]
pub struct ArrayPropType<T: Default> {
    #[serde(flatten)]
    base: PropType<Vec<T>>,

    #[schemars(description = "Minimum number of items allowed in the array.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<usize>,

    #[schemars(description = "Maximum number of items allowed in the array.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
}