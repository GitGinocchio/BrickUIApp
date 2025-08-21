use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};

use super::PropMeta;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Gradient stop color property container.")]
pub struct Stop {
    #[schemars(description = "The actual color of the stop color.")]
    #[schemars(regex(pattern = "^#[0-9A-Fa-f]{8}$"))]
    pub color: String,

    #[schemars(description = "The actual position of the stop color from 0 to 100.")]
    #[serde(default)]
    pub position: f32
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Gradient type enum.")]
pub enum GradientType {
    Linear,
    Radial,
    Conic
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Gradient property container.")]
pub struct Gradient {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(description = "Default gradient value as an array of stop colors.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<Stop>>,

    #[schemars(description = "Current gradient value as an array of stop colors.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<Stop>>,

    #[schemars(description = "Gradient type.")]
    #[serde(rename = "type", default="default_gradient_type")]
    pub gradient_type: Option<GradientType>,

    #[schemars(description = "If true, the alpha channel is ignored.")]
    #[serde(default)]
    pub skip_alpha: bool,
}

fn default_gradient_type() -> Option<GradientType> { Some(GradientType::Linear) }