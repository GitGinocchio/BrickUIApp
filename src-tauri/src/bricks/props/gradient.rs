use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::PropMeta;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Gradient stop color property container.")]
pub struct GradientStopSpec {
    #[schemars(description = "The actual color of the stop color.")]
    #[schemars(regex(pattern = "^#[0-9A-Fa-f]{8}$"))]
    pub color: String,

    #[schemars(description = "The actual position of the stop color from 0 to 100.")]
    #[serde(default)]
    pub position: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Gradient property container.")]
pub struct GradientPropSpec {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(description = "Default gradient value as an array of stop colors.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<GradientStopSpec>>,

    #[schemars(description = "Current gradient value as an array of stop colors.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<GradientStopSpec>>,

    #[schemars(description = "The gradient type to render.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GradientType>,

    #[schemars(description = "If true, the alpha channel is ignored.")]
    #[serde(default)]
    pub skip_alpha: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "PascalCase")]
pub enum GradientType {
    Linear,
    Radial,
    Conic,
}
