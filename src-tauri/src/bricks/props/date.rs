use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::PropMeta;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Date property container.")]
pub struct DatePropType {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(
        description = "Current date value of the property in the format `DD-MM` or `DD-MM-YYYY`. May be null if `default` is null."
    )]
    #[schemars(regex(pattern = r"^[0-9]{2}-[0-9]{2}(-[0-9]{4})?$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[schemars(regex(pattern = r"^[0-9]{2}-[0-9]{2}(-[0-9]{4})?$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(default = "default_allow_past")]
    pub allow_past: bool,

    #[serde(default = "default_allow_future")]
    pub allow_future: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Datetime property container.")]
pub struct DateTimePropType {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(regex(pattern = r"^[0-9]{2}-[0-9]{2}(-[0-9]{4})? [0-9]{2}:[0-9]{2}(:[0-9]{2})?$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[schemars(regex(pattern = r"^[0-9]{2}-[0-9]{2}(-[0-9]{4})? [0-9]{2}:[0-9]{2}(:[0-9]{2})?$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(default = "default_allow_past")]
    pub allow_past: bool,

    #[serde(default = "default_allow_future")]
    pub allow_future: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Time property container.")]
pub struct TimePropType {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(regex(pattern = r"^[0-9]{2}:[0-9]{2}(:[0-9]{2})?$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    #[schemars(regex(pattern = r"^[0-9]{2}:[0-9]{2}(:[0-9]{2})?$"))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,

    #[serde(default = "default_allow_past")]
    pub allow_past: bool,

    #[serde(default = "default_allow_future")]
    pub allow_future: bool,
}

fn default_allow_past() -> bool {
    true
}
fn default_allow_future() -> bool {
    true
}
