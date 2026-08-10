pub mod numeric;
use super::props::numeric::NumericPropType;

pub mod array;
use super::props::array::ArrayPropType;

pub mod select;
use super::props::select::SelectPropType;

pub mod date;
use super::props::date::{DatePropType, DateTimePropType, TimePropType};

pub mod color;
use super::props::color::Color;

pub mod gradient;
use super::props::gradient::Gradient;

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
    prop_name: String,

    #[schemars(
        description = "A brief textual description providing additional details or context about the property. \nThis helps users understand the purpose or usage of the property."
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(description = "Generic property container.")]
pub struct PropType<T> {
    #[serde(flatten)]
    meta: PropMeta,

    #[schemars(description = "Current value of the property. May be null if `default` is null.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    value: Option<T>,

    #[schemars(description = "Default value of the property.")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(untagged)] 
pub enum Prop {
    /// 1. Prova a matchare i tipi conosciuti (Bool, String, ecc.)
    Known(KnownProp),

    #[schemars(description = "Inferred Null property (missing prop_type).")]
    Null {
        #[serde(flatten)]
        data: PropMeta,
    },

    /// 2. Se 'prop_type' esiste ma non è in KnownProp, catturalo qui.
    /// Questa variante funge da "Deprecated" perché cattura la stringa originale.
    #[schemars(description = "Deprecated property.")]
    Deprecated {
        #[serde(default, rename = "prop_type")]
        deprecated_type: String,
        #[serde(flatten)]
        data: PropMeta,
    },

    /// 3. Fallback finale: se non ha nemmeno la struttura di PropMeta.
    /// In un enum untagged, l'ultima variante senza campi cattura tutto il resto.
    #[schemars(description = "Fallback for completely unknown or malformed structures.")]
    Unknown(serde_json::Value),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "prop_type")]
#[schemars(description = "Enumeration of supported property types for a Brick.")]
pub enum KnownProp {
    // Bool
    #[schemars(description = "Boolean property type.")]
    Bool {
        #[serde(flatten)]
        data: PropType<bool>,
    },

    // String
    #[schemars(description = "String property type.")]
    String {
        #[serde(flatten)]
        data: PropType<String>,
    },

    // Text
    #[schemars(description = "Multiline text property type.")]
    Text {
        #[serde(flatten)]
        data: PropType<String>,
    },

    // Int
    #[schemars(description = "Integer property type.")]
    Int {
        #[serde(flatten)]
        data: NumericPropType<i64>,
    },

    // Float
    #[schemars(description = "Floating point property type.")]
    Float {
        #[serde(flatten)]
        data: NumericPropType<f64>,
    },

    // Color
    #[schemars(description = "Color property type.")]
    Color {
        #[serde(flatten)]
        data: Color,
    },

    // Gradient
    #[schemars(description = "Gradient property type.")]
    Gradient {
        #[serde(flatten)]
        data: Gradient,
    },

    // Date
    #[schemars(description = "Date property type.")]
    Date {
        #[serde(flatten)]
        data: DatePropType,
    },

    // Datetime
    #[schemars(description = "Datetime property type.")]
    Datetime {
        #[serde(flatten)]
        data: DateTimePropType,
    },

    // Time
    #[schemars(description = "Time property type.")]
    Time {
        #[serde(flatten)]
        data: TimePropType,
    },

    // Array
    #[schemars(description = "Array property containing a list of values.")]
    Array {
        #[serde(flatten)]
        data: ArrayPropType,
    },

    // Select
    #[schemars(description = "Selectable property with predefined options.")]
    Select {
        #[serde(flatten)]
        data: SelectPropType,
    },

    // Null
    #[schemars(description = "Null property type.")]
    Null {
        #[serde(flatten)]
        data: PropMeta
    }
}