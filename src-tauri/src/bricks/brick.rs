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
    #[schemars(description = "A property holding a string value.")]
    String {
        #[schemars(description = "Current string value of the property.")]
        value: String,

        #[schemars(description = "Default string value of the property.")]
        default: String,
    },

    #[serde(rename = "int")]
    #[schemars(description = "A property holding an integer value.")]
    Int {
        #[schemars(description = "Current integer value of the property.")]
        value: i64,

        #[schemars(description = "Default integer value of the property.")]
        default: i64,
    },

    #[serde(rename = "bool")]
    #[schemars(description = "A property holding a boolean value.")]
    Bool {
        #[schemars(description = "Current boolean value of the property.")]
        value: bool,

        #[schemars(description = "Default boolean value of the property.")]
        default: bool,
    },

    #[serde(rename = "array")]
    #[schemars(description = "A property holding an array of JSON values.")]
    Array {
        #[schemars(description = "Current array value of the property.")]
        value: Vec<serde_json::Value>,

        #[schemars(description = "Default array value of the property.")]
        default: Vec<serde_json::Value>,
    },
}
