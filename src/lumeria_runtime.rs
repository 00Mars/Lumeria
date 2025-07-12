use crate::lumeria_loader::Capsule;

pub struct LumeriaRuntime {
    capsules: Vec<Capsule>,
}

impl LumeriaRuntime {
    pub fn new(capsules: Vec<Capsule>) -> Self {
        Self { capsules }
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

    fn execute_logic(&mut self, logic: &str) {
        for line in logic.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("> log:") {
                let msg = rest.trim().trim_matches('"');
                println!("{}", msg);
            } else if let Some(rest) = line.strip_prefix("> emit:") {
                let sig = rest.trim();
                self.emit(sig);
            }
        }
    }
}

