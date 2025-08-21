use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Collection of external resources such as scripts and styles to include.")]
pub struct Externals {
    #[serde(default, rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this External definition.")]
    pub schema: String,

    #[schemars(description = "List of external JavaScript scripts to load.")]
    #[serde(default)]
    pub scripts: Vec<ExternalScript>,

    #[schemars(description = "List of external CSS stylesheets to load.")]
    #[serde(default)]
    pub styles: Vec<ExternalStyle>,
}

impl Default for Externals {
    fn default() -> Self {
        Self {
            schema: "../.schemas/externals.schema.json".to_string(),
            scripts: Vec::new(),
            styles: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Defines an external JavaScript script resource.")]
pub struct ExternalScript {
    #[schemars(description = "URL or path to the external script.")]
    src: String,

    #[serde(rename = "type", default = "default_script_type")]
    #[schemars(description = "The MIME type of the script. Defaults to 'text/javascript'.")]
    script_type: String,

    #[serde(rename = "async", default = "default_script_async")]
    #[schemars(description = "If true, script will be loaded asynchronously. Defaults to false.")]
    script_async: bool,

    #[serde(default = "default_script_defer")]
    #[schemars(description = "If true, script execution will be deferred until after parsing. Defaults to false.")]
    defer: bool,

    #[schemars(description = "Optional Subresource Integrity (SRI) hash for the script.")]
    integrity: Option<String>,

    #[serde(default = "default_script_crossorigin")]
    #[schemars(description = "CORS policy for the script request. Defaults to 'anonymous'.")]
    crossorigin: String,
}

fn default_script_crossorigin() -> String { "anonymous".to_string() }
fn default_script_async() -> bool { false }
fn default_script_defer() -> bool { false }
fn default_script_type() -> String { "text/javascript".to_string() }

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Defines an external CSS stylesheet resource.")]
pub struct ExternalStyle {
    #[schemars(description = "URL or path to the external stylesheet.")]
    href: String,

    #[serde(default = "default_style_rel")]
    #[schemars(description = "Relationship attribute for the stylesheet link. Defaults to 'stylesheet'.")]
    rel: String,

    #[serde(default = "default_style_media")]
    #[schemars(description = "Media attribute specifying for which media/device the stylesheet is intended. Defaults to 'all'.")]
    media: String,

    #[schemars(description = "Optional Subresource Integrity (SRI) hash for the stylesheet.")]
    integrity: Option<String>,

    #[serde(default = "default_style_crossorigin")]
    #[schemars(description = "CORS policy for the stylesheet request. Defaults to 'anonymous'.")]
    crossorigin: String,
}

fn default_style_media() -> String { "all".to_string() }
fn default_style_rel() -> String { "stylesheet".to_string() }
fn default_style_crossorigin() -> String { "anonymous".to_string() }
