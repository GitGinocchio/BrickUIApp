use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bricks::props::PropType;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Property container for numeric types, including optional bounds.")]
pub struct NumericPropType<T: Default + PartialOrd> {
    #[serde(flatten)]
    base: PropType<T>,

    #[schemars(description = "Minimum allowed value (inclusive).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min: Option<T>,

    #[schemars(description = "Maximum allowed value (inclusive).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max: Option<T>,
}
