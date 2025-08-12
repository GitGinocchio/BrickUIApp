use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "A Brick represents a modular component with metadata, configuration, and properties.")]
pub struct Brick {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Brick definition.")]
    schema: String,

    #[schemars(description = "The unique name identifier for the Brick.")]
    name: String,

    #[serde(default = "default_description")]
    #[schemars(description = "A brief textual description of the Brick.")]
    description: String,

    #[serde(default = "default_tags")]
    #[schemars(description = "A list of tags for categorizing or labeling the Brick.")]
    tags: Vec<String>,

    #[serde(default = "default_dependencies")]
    #[schemars(description = "List of dependencies required by this Brick.")]
    dependencies: Vec<String>,

    #[schemars(description = "Optional license information for the Brick.")]
    license: Option<String>,

    #[schemars(description = "Optional icon path or URL representing the Brick.")]
    icon: Option<String>,

    #[serde(default = "default_author")]
    #[schemars(description = "Author or creator of the Brick.")]
    author: String,

    #[serde(default = "default_version")]
    #[schemars(description = "Semantic version of the Brick as [major, minor, patch].")]
    version: [u8; 3],

    #[serde(default = "default_enabled")]
    #[schemars(description = "Flag indicating whether the Brick is enabled (true) or disabled (false).")]
    enabled: bool,

    #[serde(default = "default_props")]
    #[schemars(description = "Custom properties defined for the Brick.")]
    props: Vec<Prop>,
}

fn default_schema() -> String { "../.schemas/brick.schema.json".to_string() }
fn default_author() -> String { "".to_string() }
fn default_dependencies() -> Vec<String> { vec![] }
fn default_props() -> Vec<Prop> { vec![] }
fn default_description() -> String { "".to_string() }
fn default_tags() -> Vec<String> { vec![] }
fn default_enabled() -> bool { true }
fn default_version() -> [u8; 3] { [1, 0, 0] }

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "prop_type")]
#[schemars(description = "Enumeration of supported property types for a Brick.")]
pub enum Prop {
    #[serde(rename = "string")]
    #[schemars(description = "String property type.")]
    String {
        #[serde(flatten)]
        data: PropType<String>,
    },
    #[serde(rename = "int")]
    #[schemars(description = "Integer property type.")]
    Int {
        #[serde(flatten)]
        data: NumericPropType<i64>,
    },
    #[serde(rename = "float")]
    #[schemars(description = "Floating point property type.")]
    Float {
        #[serde(flatten)]
        data: NumericPropType<f64>,
    },
    #[serde(rename = "string-array")]
    #[schemars(description = "Array property containing a list of string values.")]
    StringArray {
        #[serde(flatten)]
        data: ArrayPropType<String>,
    },
    #[serde(rename = "int-array")]
    #[schemars(description = "Array property containing a list of integer values.")]
    IntArray {
        #[serde(flatten)]
        data: ArrayPropType<i64>,
    },
    #[serde(rename = "float-array")]
    #[schemars(description = "Array property containing a list of floating point values.")]
    FloatArray {
        #[serde(flatten)]
        data: ArrayPropType<f64>,
    },
    #[serde(rename = "string-select")]
    #[schemars(description = "Selectable property with predefined string options.")]
    StringSelect {
        #[serde(flatten)]
        data: SelectablePropType<String>,
    },
    #[serde(rename = "int-select")]
    #[schemars(description = "Selectable property with predefined integer options.")]
    IntSelect {
        #[serde(flatten)]
        data: SelectablePropType<i64>,
    },
    #[serde(rename = "float-select")]
    #[schemars(description = "Selectable property with predefined floating point options.")]
    FloatSelect {
        #[serde(flatten)]
        data: SelectablePropType<f64>,
    },
    #[serde(rename = "array")]
    #[schemars(description = "Array property containing a list of generic values.")]
    Array {
        #[serde(flatten)]
        data: ArrayPropType<serde_json::Value>,  
    },
    #[serde(rename = "select")]
    #[schemars(description = "Selectable property with predefined generic options.")]
    Select {
        #[serde(flatten)]
        data: SelectablePropType<serde_json::Value>,  
    },
    #[serde(rename = "any")]
    #[schemars(description = "Generic property type.")]
    Any {
        #[serde(flatten)]
        data: serde_json::Value
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Generic property container.")]
pub struct PropType<T> {
    #[schemars(description = "The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization.")]
    prop_name: String,

    #[schemars(description = "A brief textual description providing additional details or context about the property. \nThis helps users understand the purpose or usage of the property.")]
    #[serde(default)]
    description: String,

    #[schemars(description = "Current value of the property. May be null if `nullable` is true.")]
    value: Option<T>,

    #[schemars(description = "Default value of the property.")]
    default: Option<T>,

    #[serde(default)]
    #[schemars(description = "Indicates whether this property can be null.")]
    nullable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Property container for numeric types, including optional bounds.")]
pub struct NumericPropType<T> {
    #[serde(flatten)]
    #[schemars(description = "Base property data.")]
    base: PropType<T>,

    #[schemars(description = "Minimum allowed value (inclusive).")]
    min: Option<T>,

    #[schemars(description = "Maximum allowed value (inclusive).")]
    max: Option<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Array property container with typed values.")]
pub struct ArrayPropType<T> {
    #[schemars(description = "The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization.")]
    prop_name: String,

    #[schemars(description = "A brief textual description providing additional details or context about the property. \nThis helps users understand the purpose or usage of the property.")]
    #[serde(default)]
    description: String,

    #[serde(default)]
    #[schemars(description = "Indicates whether this array property can be null.")]
    pub nullable: bool,

    #[schemars(description = "Default value of the array property.")]
    pub default: Option<Vec<T>>,

    #[schemars(description = "Current value of the array property.")]
    pub values: Option<Vec<T>>,

    #[schemars(description = "Minimum number of items allowed in the array.")]
    pub min_items: Option<usize>,

    #[schemars(description = "Maximum number of items allowed in the array.")]
    pub max_items: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Selectable property container with typed options.")]
pub struct SelectablePropType<T> {
    #[schemars(description = "The unique identifier for this property. Must exactly match the `prop_name` used in the corresponding `brick.vue` file to ensure proper binding and synchronization.")]
    prop_name: String,

    #[schemars(description = "A brief textual description providing additional details or context about the property. \nThis helps users understand the purpose or usage of the property.")]
    #[serde(default)]
    description: String,

    #[schemars(description = "List of selectable options.")]
    pub options: Vec<T>,

    #[schemars(description = "Currently selected option.")]
    pub selected: Option<T>,

    #[schemars(description = "Default selected option.")]
    pub default_selected: Option<T>,

    #[serde(default)]
    #[schemars(description = "Indicates whether this property can be null.")]
    pub nullable: bool,

    #[schemars(description = "Minimum number of selections allowed.")]
    pub min_selections: Option<usize>,

    #[schemars(description = "Maximum number of selections allowed.")]
    pub max_selections: Option<usize>,
}
