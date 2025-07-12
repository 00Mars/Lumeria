use crate::lumeria_loader::Capsule;
use std::collections::HashMap;

pub struct LumeriaRuntime {
    capsules: Vec<Capsule>,
    pub mnemonic_map: HashMap<String, String>,
}

impl LumeriaRuntime {
    pub fn new(capsules: Vec<Capsule>) -> Self {
        let mut mnemonic_map = HashMap::new();
        for cap in &capsules {
            for (k, v) in &cap.mnemonic_map {
                mnemonic_map.insert(k.clone(), v.clone());
            }
        }
        Self { capsules, mnemonic_map }
    }

    pub fn emit(&mut self, signal: &str) {
        for cap in &self.capsules {
            for (idx, trig) in cap.triggers.iter().enumerate() {
                if trig == signal {
                    if let Some(logic) = cap.logic.get(idx) {
                        self.execute_logic(logic);
                    }
                }
            }
        }
    }

    pub fn mnemonic_keys(&self) -> Vec<String> {
        self.mnemonic_map.keys().cloned().collect()
    }

    fn execute_logic(&mut self, logic: &str) {
        for line in logic.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("> log:") {
                let msg = rest.trim().trim_matches('"');
                println!("{}", msg);
            } else if let Some(rest) = line.strip_prefix("> emit:") {
                let sig = rest.trim();
                self.emit(sig);
            } else if let Some(rest) = line.strip_prefix("> mnemonic.map:") {
                let mut parts = rest.trim().splitn(2, '=');
                if let Some(key) = parts.next() {
                    if let Some(val) = parts.next() {
                        self.mnemonic_map.insert(key.trim().to_string(), val.trim().to_string());
                    }
                }
            }
        }
    }
}

