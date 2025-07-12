use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Capsule {
    pub name: String,
    pub triggers: Vec<String>,
    pub logic: Vec<String>,
    pub mnemonic_map: HashMap<String, String>,
}

pub struct CapsuleLoader {
    file: PathBuf,
}

impl CapsuleLoader {
    pub fn new<P: AsRef<Path>>(file: P) -> Self {
        Self { file: file.as_ref().to_path_buf() }
    }

    pub fn load_capsules(&self) -> Vec<Capsule> {
        let data = fs::read_to_string(&self.file).expect("Unable to read lore file");
        parse_capsules_from_text(&data)
    }

    // Load all matching `.lore` capsule files recursively
    pub fn load_dir<P: AsRef<Path>>(dir: P) -> Vec<Capsule> {
        let mut all = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    all.extend(Self::load_dir(&path));
                } else if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    if name.ends_with(".lore") || name.ends_with(".loot") || name.ends_with(".arena") {
                        let loader = CapsuleLoader::new(&path);
                        all.extend(loader.load_capsules());
                    }
                }
            }
        }
        all
    }
}

pub fn parse_capsules_from_text(data: &str) -> Vec<Capsule> {
    let mut lines = data.lines().peekable();
    let mut capsules = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.starts_with("[capsule ") && line.ends_with(']') {
            let name = line.trim_start_matches("[capsule ")
                           .trim_end_matches(']')
                           .trim()
                           .to_string();

            let mut triggers = Vec::new();
            let mut logic_blocks = Vec::new();
            let mut mnemonic_map = HashMap::new();
            let end_tag = format!("[/capsule {}]", name);

            let mut logic_buf = String::new();
            let mut in_logic = false;

            while let Some(inner) = lines.next() {
                let inner = inner.trim();
                if inner == end_tag {
                    break;
                }

                if inner.starts_with("[trigger ") && inner.ends_with("/]") {
                    if let Some(trigger) = inner.strip_prefix("[trigger ")
                                                .and_then(|s| s.strip_suffix("/]")) {
                        triggers.push(trigger.trim().to_string());
                    }
                } else if inner.starts_with("[logic ") {
                    in_logic = true;
                    logic_buf.clear(); // reset for this logic block
                } else if inner.starts_with("[/logic") {
                    in_logic = false;
                    logic_blocks.push(logic_buf.clone());
                    logic_buf.clear();
                } else if in_logic {
                    if let Some(rest) = inner.strip_prefix("> mnemonic.map:") {
                        let mut parts = rest.splitn(2, '=');
                        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                            mnemonic_map.insert(k.trim().to_string(), v.trim().to_string());
                        }
                    }
                    logic_buf.push_str(inner);
                    logic_buf.push('\n');
                }
            }

            capsules.push(Capsule {
                name,
                triggers,
                logic: logic_blocks,
                mnemonic_map,
            });
        }
    }

    capsules
}