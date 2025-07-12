use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Capsule {
    pub name: String,
    pub triggers: Vec<String>,
    pub logic: Vec<String>,
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
        let mut capsules = Vec::new();

        let mut search_start = 0;
        while let Some(cap_start) = data[search_start..].find("[capsule") {
            let start_idx = search_start + cap_start;
            if let Some(end_name_idx) = data[start_idx..].find(']') {
                let name_part = &data[start_idx + 8..start_idx + end_name_idx];
                let cap_name = name_part.trim().to_string();
                let close_tag = format!("[/capsule {}]", cap_name);
                if let Some(close_idx) = data[start_idx + end_name_idx..].find(&close_tag) {
                    let body_start = start_idx + end_name_idx + 1;
                    let body_end = start_idx + end_name_idx + close_idx;
                    let body = &data[body_start..body_end];

                    let mut triggers = Vec::new();
                    let mut logic_blocks = Vec::new();

                    let mut t_start = 0;
                    while let Some(t_idx) = body[t_start..].find("[trigger") {
                        let t_begin = t_start + t_idx + 8;
                        if let Some(t_end) = body[t_begin..].find("/]") {
                            let name = body[t_begin..t_begin + t_end].trim();
                            triggers.push(name.to_string());
                            t_start = t_begin + t_end + 3;
                        } else {
                            break;
                        }
                    }

                    let mut l_start = 0;
                    while let Some(l_idx) = body[l_start..].find("[logic ") {
                        let l_name_begin = l_start + l_idx + 6;
                        if let Some(l_name_end) = body[l_name_begin..].find(']') {
                            let logic_name = body[l_name_begin..l_name_begin + l_name_end].trim();
                            let l_close = format!("[/logic {}]", logic_name);
                            if let Some(l_close_idx) = body[l_name_begin + l_name_end..].find(&l_close) {
                                let content_start = l_name_begin + l_name_end + 1;
                                let content_end = l_name_begin + l_name_end + l_close_idx;
                                logic_blocks.push(body[content_start..content_end].trim().to_string());
                                l_start = l_name_begin + l_name_end + l_close_idx + l_close.len();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    println!("Capsule loaded: {}", cap_name);
                    capsules.push(Capsule { name: cap_name, triggers, logic: logic_blocks });
                    search_start = body_end + close_tag.len();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        capsules
    }
}

