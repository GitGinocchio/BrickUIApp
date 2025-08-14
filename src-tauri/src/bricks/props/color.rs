use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

use super::PropMeta;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Time property container.")]
pub struct Color {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(description = "Current color value as RGB (0-255 for each channel).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<[u8;4]>,

    #[schemars(description = "Default color value as RGB (0-255 for each channel).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<[u8;4]>,

    #[schemars(description = "Default swatches to show to the user (0-255 for each channel).")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swatches: Option<Vec<[u8;4]>>,

    #[schemars(description = "If true, the alpha channel is ignored.")]
    #[serde(default)]
    pub skip_alpha: bool,
}