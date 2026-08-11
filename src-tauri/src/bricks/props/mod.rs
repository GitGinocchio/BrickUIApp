pub mod numeric;
use super::props::numeric::NumericPropType;

pub mod array;
use super::props::array::ArrayPropType;

pub mod select;
use super::props::select::SelectPropType;

pub mod date;
use super::props::date::{DatePropType, DateTimePropType, TimePropType};

pub mod color;
use super::props::color::ColorPropType;

pub mod gradient;
use super::props::gradient::GradientPropType;

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
#[ts(export)]
pub struct PropType<T> {
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
    Bool(PropType<bool>),

    // String
    #[schemars(description = "String property type.")]
    String(PropType<String>),

    // Text
    #[schemars(description = "Multiline text property type.")]
    Text(PropType<String>),

    // Int
    #[schemars(description = "Integer property type.")]
    Int(NumericPropType<i32>),

    // Float
    #[schemars(description = "Floating point property type.")]
    Float(NumericPropType<f32>),

    // Color
    #[schemars(description = "Color property type.")]
    Color(ColorPropType),

    // Gradient
    #[schemars(description = "Gradient property type.")]
    Gradient(GradientPropType),

    // Date
    #[schemars(description = "Date property type.")]
    Date(DatePropType),

    // Datetime
    #[schemars(description = "Datetime property type.")]
    Datetime(DateTimePropType),

    // Time
    #[schemars(description = "Time property type.")]
    Time(TimePropType),

    // Array
    #[schemars(description = "Array property containing a list of values.")]
    Array(ArrayPropType),

    // Select
    #[schemars(description = "Selectable property with predefined options.")]
    Select(SelectPropType),

    // Null
    #[schemars(description = "Null property type.")]
    Null(PropMeta)
}