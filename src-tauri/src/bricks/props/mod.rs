pub mod numeric;
use super::props::numeric::NumericPropType;

pub mod array;
use super::props::array::ArrayPropType;

pub mod select;
use super::props::select::SelectablePropType;

pub mod date;
use super::props::date::{DatePropType, DateTimePropType, TimePropType};

pub mod color;
use super::props::color::Color;

pub mod gradient;
use super::props::gradient::Gradient;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Metadata property container.")]
pub struct PropMeta {
    #[schemars(description = "The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization.")]
    #[schemars(regex(pattern = "^[a-zA-Z][a-zA-Z0-9]*(?:_[a-zA-Z0-9]+)*$"))]
    prop_name: String,

    #[schemars(description = "A brief textual description providing additional details or context about the property. \nThis helps users understand the purpose or usage of the property.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Generic property container.")]
pub struct PropType<T: Default> {
    #[serde(flatten)]
    meta: PropMeta,

    #[schemars(description = "Current value of the property. May be null if `default` is null.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<T>,

    #[schemars(description = "Default value of the property.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "prop_type")]
#[schemars(description = "Enumeration of supported property types for a Brick.")]
pub enum Prop {
    // Primitives
    #[schemars(description = "Boolean property type.")]
    Bool { #[serde(flatten)] data: PropType<bool> },
    
    #[schemars(description = "String property type.")]
    String { #[serde(flatten)] data: PropType<String> },
    
    #[schemars(description = "Integer property type.")]
    Int { #[serde(flatten)] data: NumericPropType<i64> },

    #[schemars(description = "Floating point property type.")]
    Float { #[serde(flatten)] data: NumericPropType<f64> },

    // Color / Gradient

    #[schemars(description = "Color property type.")]
    Color { #[serde(flatten)] data: Color },

    #[schemars(description = "Gradient property type.")]
    Gradient { #[serde(flatten)] data: Gradient },

    // Dates and times
    
    #[schemars(description = "Date property type.")]
    Date { #[serde(flatten)] data: DatePropType },

    #[schemars(description = "Datetime property type.")]
    Datetime { #[serde(flatten)] data: DateTimePropType },
    
    #[schemars(description = "Time property type.")]
    Time { #[serde(flatten)] data: TimePropType },

    // Arrays

    #[schemars(description = "Array property containing a list of boolean values.")]
    BoolArray { #[serde(flatten)] data: ArrayPropType<bool> },

    #[schemars(description = "Array property containing a list of string values.")]
    StringArray { #[serde(flatten)] data: ArrayPropType<String> },

    #[schemars(description = "Array property containing a list of integer values.")]
    IntArray { #[serde(flatten)] data: ArrayPropType<i64> },

    #[schemars(description = "Array property containing a list of floating point values.")]
    FloatArray { #[serde(flatten)] data: ArrayPropType<f64> },

    // Selects

    #[schemars(description = "Selectable property with predefined string options.")]
    StringSelect { #[serde(flatten)] data: SelectablePropType<String> },

    #[schemars(description = "Selectable property with predefined integer options.")]
    IntSelect { #[serde(flatten)] data: SelectablePropType<i64> },

    #[schemars(description = "Selectable property with predefined floating point options.")]
    FloatSelect { #[serde(flatten)] data: SelectablePropType<f64> },

    // Generics

    #[schemars(description = "Array property containing a list of generic values.")]
    Array { #[serde(flatten)] data: ArrayPropType<serde_json::Value> },

    #[schemars(description = "Selectable property with predefined generic options.")]
    Select { #[serde(flatten)] data: SelectablePropType<serde_json::Value> },

    #[schemars(description = "Generic property type.")]
    Any { #[serde(flatten)] data: serde_json::Value }
}