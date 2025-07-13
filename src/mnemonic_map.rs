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
            for (k, v) in &cap.mnemonic_map {
                codes.insert(k.clone(), v.clone());
            }
        }
        Self { codes }
    }

    pub fn resolve(&self, key: &str) -> Option<&str> {
        self.codes.get(key).map(|s| s.as_str())
    }

    pub fn keys(&self) -> Vec<String> {
        self.codes.keys().cloned().collect()
    }
}
