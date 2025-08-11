use std::collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "A collection of plugins with their configurations.")]
pub struct Plugins {
    #[serde(default, rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Plugins definition.")]
    schema: String,

    #[serde(default)]
    #[schemars(description = "List of configured plugins.")]
    plugins: Vec<Plugin>,
}

impl Default for Plugins {
    fn default() -> Self {
        Self {
            schema: "../.schemas/plugins.schema.json".to_string(),
            plugins: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "config")]
#[schemars(description = "Plugin configuration variants, including a generic fallback.")]
pub enum PluginConfig {
    // #[schemars(description = "Configuration for PluginA with specific fields.")]
    // PluginA { field1: String, field2: i32 },
    // #[schemars(description = "Configuration for PluginB with enable flag and level.")]
    // PluginB { enabled: bool, level: u8 },
    
    #[schemars(description = "Generic plugin configuration with arbitrary key-value pairs.")]
    Generic(HashMap<String, serde_json::Value>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(description = "Represents a single plugin with a name and configuration.")]
pub struct Plugin {
    #[schemars(description = "The unique name of the plugin.")]
    name: String,

    #[schemars(description = "The configuration details specific to the plugin.")]
    config: PluginConfig,
}
