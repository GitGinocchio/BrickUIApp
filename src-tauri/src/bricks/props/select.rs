use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bricks::props::PropType;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Selectable property container with typed options.")]
pub struct SelectablePropType<T: Default> {
    #[serde(flatten)]
    base: PropType<T>,

    #[schemars(description = "List of selectable options.")]
    pub options: Vec<T>,

    #[schemars(description = "Minimum number of selections allowed.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<usize>,

    #[schemars(description = "Maximum number of selections allowed.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
}
