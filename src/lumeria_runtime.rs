// ===== src/lumeria_runtime.rs =====
use std::collections::HashMap;
use crate::lumeria_loader::Capsule;

pub struct LumeriaRuntime {
    capsules: Vec<Capsule>,
    trigger_map: HashMap<String, Vec<usize>>,
}

impl LumeriaRuntime {
    pub fn new(capsules: Vec<Capsule>) -> Self {
        let mut trigger_map: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, cap) in capsules.iter().enumerate() {
            for trig in &cap.triggers {
                trigger_map.entry(trig.clone()).or_default().push(i);
            }
        }
        Self { capsules, trigger_map }
    }

    pub fn emit(&mut self, signal: &str) {
    println!("\n🚨 Emit: {}", signal);

    let Some(indices) = self.trigger_map.get(signal).cloned() else {
        println!("⚠️ No capsule responds to: {}", signal);
        return;
    };

    for i in indices {
        // Extract the needed fields to break the borrow
        let (name, logic) = {
            let cap = &self.capsules[i];
            (cap.name.clone(), cap.logic.clone())
        };

        println!("📦 Executing: {}", name);
        self.execute(&logic);
    }
}



    fn execute(&mut self, logic: &[String]) {
        for line in logic {
            let trimmed = line.trim();

            if let Some(rest) = trimmed.strip_prefix("> log:") {
                println!("🗣️ {}", rest.trim().trim_matches('"'));
            } else if let Some(rest) = trimmed.strip_prefix("> emit:") {
                self.emit(rest.trim());
            } else {
                println!("⚙️ {}", trimmed);
            }
        }
    }
}
