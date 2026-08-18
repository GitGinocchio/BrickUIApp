use super::props::Prop;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::is_empty_opt_string;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, TS)]
#[schemars(
    description = "A Brick represents a modular component with metadata, configuration, and properties."
)]
pub struct Brick {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Brick definition.")]
    pub schema: String,

    #[schemars(description = "The unique name identifier for the Brick.")]
    #[schemars(regex(pattern = "^[a-zA-Z][a-zA-Z0-9]*(?:_[a-zA-Z0-9]+)*$"))]
    pub name: String,

    #[serde(default, skip_serializing_if = "is_empty_opt_string")]
    #[schemars(description = "A brief textual description of the Brick.")]
    pub description: Option<String>,

    #[serde(default = "default_tags", skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "A list of tags for categorizing or labeling the Brick.")]
    pub tags: Vec<String>,

    #[serde(default = "default_dependencies", skip_serializing_if = "Vec::is_empty")]
    #[schemars(description = "List of dependencies required by this Brick.")]
    pub dependencies: Vec<String>,

    #[schemars(description = "Optional license information for the Brick.")]
    #[serde(default, skip_serializing_if = "is_empty_opt_string")]
    pub license: Option<String>,

    #[schemars(description = "Optional icon path or URL representing the Brick.")]
    #[serde(default, skip_serializing_if = "is_empty_opt_string")]
    pub icon: Option<String>,

    #[serde(default, skip_serializing_if = "is_empty_opt_string")]
    #[schemars(description = "Optional banner path or URL representing the Brick.")]
    pub banner: Option<String>,

    #[serde(default, skip_serializing_if = "is_empty_opt_string")]
    #[schemars(description = "Author or creator of the Brick.")]
    pub author: Option<String>,

    #[serde(default = "default_version")]
    #[schemars(description = "Semantic version of the Brick as [major, minor, patch].")]
    pub version: [u8; 3],

    #[serde(default = "default_enabled")]
    #[schemars(
        description = "Flag indicating whether the Brick is enabled (true) or disabled (false)."
    )]
    pub enabled: bool,

    #[serde(default = "default_props")]
    #[schemars(description = "Custom properties defined for the Brick.")]
    pub props: Vec<Prop>,
}

fn default_schema() -> String {
    "../../.schemas/brick.schema.json".to_string()
}
fn default_dependencies() -> Vec<String> {
    vec![]
}
fn default_props() -> Vec<Prop> {
    vec![]
}
fn default_tags() -> Vec<String> {
    vec![]
}
fn default_enabled() -> bool {
    true
}
fn default_version() -> [u8; 3] {
    [0, 1, 0]
}
