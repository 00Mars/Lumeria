use crate::lumeria_loader::Capsule;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MnemonicMap {
    codes: HashMap<String, String>,
}

impl MnemonicMap {
    pub fn from_capsules(capsules: &[Capsule]) -> Self {
    let mut codes = HashMap::new();

    for cap in capsules {
        // Legacy support — pull internal capsule mapping
        for (k, v) in &cap.mnemonic_map {
            codes.insert(k.clone(), v.clone());
        }

        // Scan for actual grammar blocks
        for block in &cap.blocks {
            if block.block_type == "grammar" && block.name == "mnemonic.map" {
                for line in block.content.lines() {
                    if let Some((k, v)) = line.split_once('=') {
                        codes.insert(k.trim().to_string(), v.trim().to_string());
                    }
                }
            }
        }
    }

    Self { codes }
}

    pub fn resolve(&self, key: &str) -> Option<&str> {
        self.codes.get(key).map(|s| s.as_str())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.codes.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.codes.keys().cloned().collect()
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.codes.insert(key, value);
    }

    pub fn is_known_verb(&self, verb: &str) -> bool {
        self.codes.contains_key(verb)
    }
}
