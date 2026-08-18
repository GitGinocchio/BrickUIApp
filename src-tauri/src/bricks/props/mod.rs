pub mod numeric;
use super::props::numeric::NumericPropSpec;

pub mod array;
use crate::bricks::props::array::ArrayPropKind;

pub mod select;
use crate::bricks::props::select::SelectPropKind;

pub mod date;
use super::props::date::{DatePropSpec, DateTimePropSpec, TimePropSpec};

pub mod color;
use super::props::color::ColorPropSpec;

pub mod gradient;
use super::props::gradient::GradientPropSpec;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Metadata property container.")]
pub struct PropMeta {
    #[schemars(
        description = "The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization."
    )]
    #[schemars(regex(pattern = "^[a-zA-Z][a-zA-Z0-9]*(?:_[a-zA-Z0-9]+)*$"))]
    pub prop_name: String,

    #[schemars(
        description = "A brief textual description providing additional details or context about the property. \nThis helps users understand the purpose or usage of the property."
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Generic property container.")]
pub struct PropSpec<T> {
    #[serde(flatten)]
    pub meta: PropMeta,

    #[schemars(description = "Current value of the property. May be null if `default` is null.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,

    #[schemars(description = "Default value of the property.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(untagged)]
#[ts(export)]
pub enum Prop {
    ValidProp(ValidProp),
    Deprecated(DeprecatedProp),
    InvalidProp(InvalidProp),
}

/// Struttura dedicata per le proprietà deprecate/non riconosciute.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[ts(export)]
pub struct DeprecatedProp {
    /// In deserializzazione cattura il valore originale di "prop_type" (es. "OldCustomProp")
    #[serde(rename(deserialize = "prop_type"))]
    pub deprecated_type: String,

    /// In serializzazione scrive "prop_type": "Deprecated" nel JSON di output
    #[serde(skip_deserializing, default = "default_deprecated_tag")]
    #[ts(type = "\"Deprecated\"")]
    pub prop_type: String,
    
    #[serde(flatten)]
    pub meta: PropMeta,
}

fn default_deprecated_tag() -> String {
    "Deprecated".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "prop_type")]
#[ts(export)]
pub enum InvalidProp {
    /// 2. Fallback finale per strutture totalmente sconosciute o malformate.
    #[schemars(description = "Fallback for completely unknown or malformed structures.")]
    #[serde(other)]
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "prop_type")]
#[schemars(description = "Enumeration of supported property types for a Brick.")]
#[ts(export)]
pub enum ValidProp {
    // Bool
    #[schemars(description = "Boolean property type.")]
    Bool(PropSpec<bool>),

    // String
    #[schemars(description = "String property type.")]
    String(PropSpec<String>),

    // Text
    #[schemars(description = "Multiline text property type.")]
    Text(PropSpec<String>),

    // Int
    #[schemars(description = "Integer property type.")]
    Int(NumericPropSpec<i32>),

    // Float
    #[schemars(description = "Floating point property type.")]
    Float(NumericPropSpec<f32>),

    // Color
    #[schemars(description = "Color property type.")]
    Color(ColorPropSpec),

    // Gradient
    #[schemars(description = "Gradient property type.")]
    Gradient(GradientPropSpec),

    // Date
    #[schemars(description = "Date property type.")]
    Date(DatePropSpec),

    // Datetime
    #[schemars(description = "Datetime property type.")]
    Datetime(DateTimePropSpec),

    // Time
    #[schemars(description = "Time property type.")]
    Time(TimePropSpec),

    // Array
    #[schemars(description = "Array property containing a list of values.")]
    Array(ArrayPropKind),

    // Select
    #[schemars(description = "Selectable property with predefined options.")]
    Select(SelectPropKind),

    // Null
    #[schemars(description = "Null property type.")]
    Null(PropMeta)
}