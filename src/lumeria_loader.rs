use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Capsule {
    pub name: String,
    pub triggers: Vec<String>,
    pub logic: Vec<String>,
    pub mnemonic_map: HashMap<String, String>,
    pub memory_blocks: Vec<String>,
    pub meta_blocks: Vec<String>,
    pub ui_blocks: Vec<String>,
    pub rule_blocks: Vec<String>,
    pub grammar_blocks: Vec<String>,
    pub raw: String, // optional: store entire raw capsule text if needed
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBlock(String),     // e.g., [capsule boot.loader]
    CloseBlock(String),    // e.g., [/capsule boot.loader]
    KeyValue(String, String), // e.g., set: version = 1.0
    Directive(String, String), // e.g., > emit: signal.boot
    Text(String),          // plain text or logic body lines
}

#[derive(Debug)]
pub struct CapsuleNode {
    pub name: String,
    pub blocks: Vec<BlockNode>,
}

#[derive(Debug)]
pub struct BlockNode {
    pub block_type: String,
    pub name: String,
    pub lines: Vec<String>,
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
        parse_capsules_from_file(&data)
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

pub fn parse_capsules_from_file(data: &str) -> Vec<Capsule> {
    let mut capsules = Vec::new();
    let mut pos = 0;

    while let Some(start_idx) = data[pos..].find("[capsule ") {
        let start = pos + start_idx;
        let name_start = start + "[capsule ".len();
        let name_end = data[name_start..].find(']').unwrap() + name_start;
        let capsule_name = data[name_start..name_end].trim().to_string();

        let end_tag = format!("[/capsule {}]", capsule_name);
        let body_start = name_end + 1;
        let body_end = data[body_start..].find(&end_tag).unwrap() + body_start;

        let body = &data[body_start..body_end];
        pos = body_end + end_tag.len();

        let triggers = extract_blocks(body, "[trigger ", "/]");
        let logic_blocks = extract_named_blocks(body, "[logic ", "[/logic");
        let memory_blocks = extract_named_blocks(body, "[memory ", "[/memory");
        let meta_blocks = extract_named_blocks(body, "[meta ", "[/meta");
        let ui_blocks = extract_named_blocks(body, "[ui ", "[/ui");
        let rule_blocks = extract_named_blocks(body, "[rule ", "[/rule");
        let grammar_blocks = extract_named_blocks(body, "[grammar ", "[/grammar");

        let mut mnemonic_map = HashMap::new();

        fn extract_mnemonic(lines: &str, map: &mut HashMap<String, String>) {
            for line in lines.lines() {
                if let Some(rest) = line.trim().strip_prefix("> mnemonic.map:") {
                    let mut parts = rest.splitn(2, '=');
                    if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
                        map.insert(k.trim().to_string(), v.trim().to_string());
                    }
                }
            }
        }

        for block in &logic_blocks {
            extract_mnemonic(block, &mut mnemonic_map);
        }
        for block in &grammar_blocks {
            extract_mnemonic(block, &mut mnemonic_map);
        }

        println!("🧠 Capsule loaded: {}", capsule_name);

        capsules.push(Capsule {
            name: capsule_name,
            triggers,
            logic: logic_blocks,
            mnemonic_map,
            memory_blocks,
            meta_blocks,
            ui_blocks,
            rule_blocks,
            grammar_blocks,
            raw: data[start..pos].to_string(),
        });
    }

    capsules
}

fn extract_blocks(source: &str, start_pat: &str, end_pat: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut pos = 0;
    while let Some(start_idx) = source[pos..].find(start_pat) {
        let start = pos + start_idx + start_pat.len();
        let end = source[start..].find(end_pat);
        if let Some(end_offset) = end {
            let content = source[start..start + end_offset].trim().to_string();
            blocks.push(content);
            pos = start + end_offset + end_pat.len();
        } else {
            break;
        }
    }
    blocks
}

fn extract_named_blocks(source: &str, open_prefix: &str, close_prefix: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut pos = 0;
    while let Some(start_idx) = source[pos..].find(open_prefix) {
        let start = pos + start_idx;
        let name_start = start + open_prefix.len();

        let name_end = match source[name_start..].find(']') {
            Some(offset) => name_start + offset,
            None => {
                eprintln!("⚠️ Malformed block missing closing `]` after {}", open_prefix);
                break;
            }
        };
        let block_name = &source[name_start..name_end].trim();

        let body_start = name_end + 1;
        let close_tag = format!("{} {}", close_prefix, block_name);
        let body_end = match source[body_start..].find(&close_tag) {
            Some(offset) => body_start + offset,
            None => {
                eprintln!("⚠️ Malformed block missing closing tag for {}", block_name);
                break;
            }
        };

        blocks.push(source[body_start..body_end].trim().to_string());
        pos = body_end + close_tag.len() + 1; // +1 for trailing ']'
    }
    blocks
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with("[/") && line.ends_with("]") {
            let tag = line[2..line.len() - 1].trim().to_string();
            tokens.push(Token::CloseBlock(tag));
        } else if line.starts_with('[') && line.ends_with(']') {
            let tag = line[1..line.len() - 1].trim().to_string();
            tokens.push(Token::OpenBlock(tag));
        } else if line.starts_with('>') {
            if let Some((cmd, val)) = line[1..].trim().split_once(':') {
                tokens.push(Token::Directive(cmd.trim().to_string(), val.trim().to_string()));
            } else {
                tokens.push(Token::Text(line.to_string()));
            }
        } else if line.contains(':') {
            if let Some((k, v)) = line.split_once(':') {
                tokens.push(Token::KeyValue(k.trim().to_string(), v.trim().to_string()));
            } else {
                tokens.push(Token::Text(line.to_string()));
            }
        } else {
            tokens.push(Token::Text(line.to_string()));
        }
    }
    tokens
}

pub fn parse_tokens(tokens: &[Token]) -> Vec<CapsuleNode> {
    let mut capsules = Vec::new();
    let mut current_capsule = None;
    let mut current_block = None;

    for token in tokens {
        match token {
            Token::OpenBlock(tag) => {
                let mut parts = tag.split_whitespace();
                if let Some(kind) = parts.next() {
                    let name = parts.collect::<Vec<_>>().join(" ");
                    match kind {
                        "capsule" => {
                            current_capsule = Some(CapsuleNode { name, blocks: vec![] });
                        }
                        _ => {
                            current_block = Some(BlockNode {
                                block_type: kind.to_string(),
                                name,
                                lines: vec![],
                            });
                        }
                    }
                }
            }
            Token::CloseBlock(tag) => {
                if tag.starts_with("capsule") {
                    if let Some(capsule) = current_capsule.take() {
                        capsules.push(capsule);
                    }
                } else if let Some(block) = current_block.take() {
                    if let Some(capsule) = current_capsule.as_mut() {
                        capsule.blocks.push(block);
                    }
                }
            }
            Token::Directive(_, line) | Token::Text(line) => {
                if let Some(block) = current_block.as_mut() {
                    block.lines.push(line.to_string());
                }
            }
            _ => {}
        }
    }

    capsules
}
