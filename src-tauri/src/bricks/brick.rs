use serde::{Deserialize, Serialize};
use serde_yaml::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct Brick {
    name: String,
    #[serde(default = "default_description")] 
    description: String,
    #[serde(default = "default_tags")]
    tags: Vec<String>,
    #[serde(default = "default_dependencies")]
    dependencies: Vec<String>,
    license: Option<String>,
    icon: Option<String>,
    #[serde(default = "default_author")]
    author: String,
    #[serde(default = "default_version")]
    version: [u8;3],
    #[serde(default = "default_enabled")]
    enabled: bool,
    #[serde(default = "default_props")]
    props: Vec<Prop>
}

fn default_author() -> String { /* find the default author */ "".to_string() }
fn default_dependencies() -> Vec<String> { vec![] }
fn default_props() -> Vec<Prop> { vec![] }
fn default_description() -> String { "".to_string() }
fn default_tags() -> Vec<String> { vec![] }
fn default_enabled() -> bool { true }
fn default_version() -> [u8; 3] { [1, 0, 0] }

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "prop_type")]
pub enum Prop {
    #[serde(rename = "string")]
    String {
        value: String,
        default: String,
    },

    #[serde(rename = "int")]
    Int {
        value: i64,
        default: i64,
    },

    #[serde(rename = "bool")]
    Bool {
        value: bool,
        default: bool,
    },

    #[serde(rename = "Array")]
    Array {
        value: Vec<Value>,
        default: Vec<Value>
    }
}