use std::fs;

use serde::{Deserialize, Serialize};

use crate::types::{EditorCommand, KeyCode, KeyModifiers};
use schemars::{schema_for, JsonSchema};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Keymap {
    pub key: KeyCode,
    pub command: EditorCommand,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EditorConfig {
    pub keymaps: Vec<Keymap>,
}

impl EditorConfig {
    pub fn new() -> EditorConfig {
        EditorConfig {
            keymaps: vec![Keymap {
                command: EditorCommand::Quit,
                key: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            }],
        }
    }

    // Saves the configuration to a json file
    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let j = serde_json::to_string(&self)?;
        fs::write(path, j)?;
        Ok(())
    }

    pub fn load(path: &str) -> anyhow::Result<EditorConfig> {
        let j = fs::read_to_string(path)?;
        let config: EditorConfig = serde_json::from_str(&j)?;
        Ok(config)
    }

    pub fn generate_schema(&self) -> String {
        let schema = schema_for!(EditorConfig);
        serde_json::to_string_pretty(&schema).unwrap()
    }
}
