use std::fs;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Capsule {
    pub name: String,
    pub triggers: Vec<String>,
    pub logic: Vec<String>,
    pub mnemonic_map: HashMap<String, String>,
}

pub fn load_capsules_from_dir<P: AsRef<Path>>(dir: P) -> Vec<Capsule> {
    let mut capsules = Vec::new();

pub fn load_capsules_from_dir(dir: &Path) -> Vec<Capsule> {
    let mut capsules = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                capsules.extend(load_capsules_from_dir(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "lore" || ext == "loot" || ext == "arena" {
                    let file_data = fs::read_to_string(&path).unwrap_or_default();
                    capsules.extend(parse_capsules_from_file(&file_data));
                }
            }
        }
    }

    capsules
}

fn parse_capsules_from_file(data: &str) -> Vec<Capsule> {
    let capsule_re = Regex::new(r"\[capsule\s+([^\]]+)\](?s)(.*?)\[/capsule\s+\1\]").unwrap();
    let trigger_re = Regex::new(r"\[trigger\s+([^\s/]+)\s*/\]").unwrap();
    let logic_re = Regex::new(r"\[logic\s+([^\]]+)\](?s)(.*?)\[/logic\s+\1\]").unwrap();

    let mut capsules = Vec::new();

    for cap in capsule_re.captures_iter(data) {
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
                    if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
                        mnemonic_map.insert(key.trim().to_string(), val.trim().to_string());
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

fn parse_capsules_from_file(data: &str) -> Vec<Capsule> {
    let capsule_re = Regex::new(r"\[capsule ([^\]]+)](?s)(.*?)\[/capsule .*?\]").unwrap();
    let trigger_re = Regex::new(r"\[trigger ([^/]+) /]").unwrap();
    let logic_re = Regex::new(r"\[logic [^\]]+](?s)(.*?)\[/logic.*?\]").unwrap();

    let mut results = Vec::new();

    for cap in capsule_re.captures_iter(data) {
        let name = cap.get(1).unwrap().as_str().trim().to_string();
        let inner = cap.get(2).unwrap().as_str();

        let triggers = trigger_re
            .captures_iter(inner)
            .map(|c| c[1].trim().to_string())
            .collect::<Vec<_>>();

        let logic = logic_re
            .captures_iter(inner)
            .flat_map(|c| c[1].lines().map(|l| l.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        results.push(Capsule { name, triggers, logic });
    }

    results
}
