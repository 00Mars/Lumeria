use crate::lumeria_loader::Capsule;
use crate::mnemonic_map::MnemonicMap;
use std::collections::HashMap;

pub struct MnemonicEngine {
    capsules: Vec<Capsule>,
    pub mnemonic: MnemonicMap,
    trigger_map: HashMap<String, Vec<usize>>,
}

impl MnemonicEngine {
    pub fn new(capsules: Vec<Capsule>) -> Self {
        let mnemonic = MnemonicMap::from_capsules(&capsules);
        let mut trigger_map: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, cap) in capsules.iter().enumerate() {
            for trig in &cap.triggers {
                trigger_map.entry(trig.clone()).or_default().push(i);
            }
        }
        Self { capsules, mnemonic, trigger_map }
    }

    pub fn emit(&mut self, signal: &str) {
        println!("\n🚨 emit: {}", signal);
        if let Some(indices) = self.trigger_map.get(signal).cloned() {
            for idx in indices {
                let capsule = &self.capsules[idx];
                println!("📦 {}", capsule.name);
                let logic_blocks = capsule.logic.clone();
                for logic in logic_blocks {
                    self.execute_logic(&logic);
                }
            }
        } else {
            println!("⚠️ no capsule for {}", signal);
        }
    }

    fn execute_logic(&mut self, logic: &str) {
        for line in logic.lines() {
            let line = line.trim();
            if let Some(dir) = line.strip_prefix('>') {
                let parts: Vec<_> = dir.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let verb = parts[0].trim();
                    let arg = parts[1].trim();
                    if let Some(code) = self.mnemonic.resolve(verb) {
                        println!("   {verb} ({code}) => {arg}");
                        if verb == "emit" || verb == "emit.signal" {
                            self.emit(arg);
                        } else if verb == "log" {
                            println!("{arg}");
                        }
                    } else {
                        println!("   🚫 unknown verb: {verb}");
                    }
                }
            }
        }
    }
}
