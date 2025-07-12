use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
pub struct Capsule {
    pub name: String,
    pub triggers: Vec<String>,
    pub logic: Vec<String>,
    pub mnemonic_map: HashMap<String, String>,
}

pub struct CapsuleLoader {
    file: String,
}

impl CapsuleLoader {
    pub fn new<P: AsRef<Path>>(file: P) -> Self {
        Self { file: file.as_ref().to_string_lossy().to_string() }
    }

    pub fn load_capsules(&self) -> Vec<Capsule> {
        let data = fs::read_to_string(&self.file)
            .expect("Unable to read lore file");
        let capsule_re = Regex::new(r"\[capsule\s+([^\]]+)\](?s)(.*?)\[/capsule\s+\1\]").unwrap();
        let trigger_re = Regex::new(r"\[trigger\s+([^\s/]+)\s*/\]").unwrap();
        let logic_re = Regex::new(r"\[logic\s+([^\]]+)\](?s)(.*?)\[/logic\s+\1\]").unwrap();

        let mut capsules = Vec::new();
        for cap in capsule_re.captures_iter(&data) {
            let cap_name = cap.get(1).unwrap().as_str().trim().to_string();
            let body = cap.get(2).unwrap().as_str();
            let mut triggers = Vec::new();
            let mut logic_blocks = Vec::new();
            let mut mnemonic_map = HashMap::new();
            for tcap in trigger_re.captures_iter(body) {
                triggers.push(tcap.get(1).unwrap().as_str().trim().to_string());
            }
            for lcap in logic_re.captures_iter(body) {
                let logic_text = lcap.get(2).unwrap().as_str().trim().to_string();
                for line in logic_text.lines() {
                    let line = line.trim();
                    if let Some(rest) = line.strip_prefix("> mnemonic.map:") {
                        let mut parts = rest.trim().splitn(2, '=');
                        if let Some(key) = parts.next() {
                            if let Some(val) = parts.next() {
                                mnemonic_map.insert(key.trim().to_string(), val.trim().to_string());
                            }
                        }
                    }
                }
                logic_blocks.push(logic_text);
            }
            let capsule = Capsule {
                name: cap_name,
                triggers,
                logic: logic_blocks,
                mnemonic_map,
            };
            println!("🧠 Capsule loaded: {}", capsule.name);
            capsules.push(capsule);
        }
        capsules
    }
}

