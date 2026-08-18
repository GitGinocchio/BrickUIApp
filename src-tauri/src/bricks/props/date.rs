use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::PropMeta;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Date property container.")]
pub struct DatePropSpec {
    #[serde(flatten)]
    pub base: PropMeta,

    #[schemars(
        description = "Current date value of the property in the format `DD-MM` or `DD-MM-YYYY`. May be null if `default` is null."
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<NaiveDate>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<NaiveDate>,

    #[serde(default = "default_allow_past")]
    pub allow_past: bool,

    #[serde(default = "default_allow_future")]
    pub allow_future: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Datetime property container.")]
pub struct DateTimePropSpec {
    #[serde(flatten)]
    pub base: PropMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<NaiveDateTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<NaiveDateTime>,

    #[serde(default = "default_allow_past")]
    pub allow_past: bool,

    #[serde(default = "default_allow_future")]
    pub allow_future: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Time property container.")]
pub struct TimePropSpec {
    #[serde(flatten)]
    pub base: PropMeta,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<NaiveTime>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<NaiveTime>,

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
