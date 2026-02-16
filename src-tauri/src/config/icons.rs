use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::save_yaml_async;
use super::load_yaml_async;
use super::write_schema_if_missing;
use super::write_template_if_missing;


// In questo modo viene calcolato solo una volta l'hash del contenuto di un icona
// In base al percorso del file si puo' ottenere l'hash del contenuto (che e' anche il nome del file)
// In questo modo abbiamo un singolo file per ogni icona diversa
#[derive(Serialize, Deserialize, Default, Clone, Debug, JsonSchema)]
pub struct IconsMap {
    #[serde(default = "default_schema", rename = "$schema", skip)]
    #[schemars(description = "The JSON Schema version or URI for this Settings definition.")]
    pub schema: String,

    pub entries: HashMap<String, IconEntry>,
}

impl IconsMap {
    pub async fn load(dir: &PathBuf) -> Result<Self, String> {
        write_schema_if_missing::<IconsMap>(&dir, "icons.map.schema.json").await?;

        let dir = dir.join("cache").join("icons");
        write_template_if_missing::<IconsMap>(&dir, "icons.map.yml").await?;
        load_yaml_async::<IconsMap>(&dir.join("icons.map.yml")).await
    }

    pub async fn save(&self, dir: &PathBuf) -> Result<(), String> {
        save_yaml_async(&dir.join("icons.map.yml"), self).await
    }
}

fn default_schema() -> String {
    "../../.schemas/icons.map.schema.json".to_string()
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, JsonSchema)]
pub struct IconEntry {
    pub hash: String,
    pub created_at: String,
}