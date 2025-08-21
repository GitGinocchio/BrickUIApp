use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};

use super::PropMeta;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Color property container.")]
pub struct Color {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(description = "Current color value as hex string (#RRGGBBAA).")]
    #[schemars(regex(pattern = "^#[0-9A-Fa-f]{8}$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[schemars(description = "Default color value as hex string (#RRGGBBAA).")]
    #[schemars(regex(pattern = "^#[0-9A-Fa-f]{8}$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[schemars(description = "Default swatches to show to the user as hex strings (#RRGGBBAA).")]
    #[schemars(regex(pattern = "^#[0-9A-Fa-f]{8}$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swatches: Option<Vec<String>>,

    #[schemars(description = "Color saved by the user as hex strings (#RRGGBBAA).")]
    #[schemars(regex(pattern = "^#[0-9A-Fa-f]{8}$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved: Option<Vec<String>>,

    #[schemars(description = "If true, the alpha channel is ignored.")]
    #[serde(default)]
    pub skip_alpha: bool,
}