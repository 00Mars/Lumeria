use crate::lumeria_loader::Capsule;
use std::collections::HashMap;

const MAX_RECURSION_DEPTH: usize = 64;
const MAX_SIGNAL_RECURSION: usize = 8;

pub struct LumeriaRuntime {
    pub capsules: Vec<Capsule>,
    pub mnemonic_map: HashMap<String, String>,
    pub trigger_map: HashMap<String, Vec<usize>>,
    pub memory: HashMap<String, i64>,
    call_stack: Vec<String>,
    recursion_counts: HashMap<String, usize>,
}

impl LumeriaRuntime {
    pub fn new(capsules: Vec<Capsule>) -> Self {
        let mut mnemonic_map = HashMap::new();
        let mut trigger_map: HashMap<String, Vec<usize>> = HashMap::new();

        for (i, cap) in capsules.iter().enumerate() {
            for (k, v) in &cap.mnemonic_map {
                mnemonic_map.insert(k.clone(), v.clone());
            }
            for trig in &cap.triggers {
                trigger_map.entry(trig.clone()).or_default().push(i);
            }
        }

        Self {
            capsules,
            mnemonic_map,
            trigger_map,
            memory: HashMap::new(),
            call_stack: Vec::new(),
            recursion_counts: HashMap::new(),
        }
    }

    pub fn emit(&mut self, signal: &str) {
    if self.call_stack.len() >= MAX_RECURSION_DEPTH {
        println!("⚠️ Maximum recursion depth exceeded at: {}", signal);
        return;
    }

    if self.call_stack.contains(&signal.to_string()) {
        println!("🔁 Infinite loop detected: {}", signal);
        return;
    }

    let count = self.recursion_counts.entry(signal.to_string()).or_insert(0);
    if *count >= MAX_SIGNAL_RECURSION {
        println!("🛑 Recursion guard blocked: {} (count = {})", signal, count);
        return;
    }

    *count += 1;
    self.call_stack.push(signal.to_string());
    println!("\n🚨 Emit: {}", signal);

    let Some(indices) = self.trigger_map.get(signal).cloned() else {
        println!("⚠️ No capsule responds to: {}", signal);
        self.call_stack.pop();
        if let Some(c) = self.recursion_counts.get_mut(signal) {
            *c = c.saturating_sub(1);
        }
        return;
    };

    for i in indices {
        let (name, logic_blocks) = {
            let cap = &self.capsules[i];
            (cap.name.clone(), cap.logic.clone())
        };

        println!("📦 Executing: {}", name);
        for logic in logic_blocks {
            self.execute_logic(&logic);
        }
    }

    self.call_stack.pop();
    if let Some(c) = self.recursion_counts.get_mut(signal) {
        *c = c.saturating_sub(1);
    }
}

    pub fn mnemonic_keys(&self) -> Vec<String> {
        self.mnemonic_map.keys().cloned().collect()
    }

    fn execute_logic(&mut self, logic: &str) {
        let mut lines = logic.lines().peekable();

        while let Some(raw) = lines.next() {
            let line = raw.trim();

            if let Some(rest) = line.strip_prefix("> log:") {
                let msg = rest.trim().trim_matches('"');
                println!("{}", msg);
            } else if let Some(rest) = line.strip_prefix("> emit:") {
                self.emit(rest.trim());
            } else if let Some(rest) = line.strip_prefix("> default:") {
                if let Some((k, v)) = self.parse_assignment(rest) {
                    self.memory.entry(k).or_insert(v);
                }
            } else if let Some(rest) = line.strip_prefix("> memory.set:") {
                if let Some((k, v)) = self.parse_assignment(rest) {
                    self.memory.insert(k, v);
                }
            } else if let Some(rest) = line.strip_prefix("> mnemonic.map:") {
                let mut parts = rest.trim().splitn(2, '=');
                if let (Some(key), Some(val)) = (parts.next(), parts.next()) {
                    self.mnemonic_map
                        .insert(key.trim().to_string(), val.trim().to_string());
                }
            } else if line.starts_with("[condition") {
                if let Some(end_idx) = line.find(']') {
                    let name = line[10..end_idx].trim();
                    let check_line = lines.next().unwrap_or("");
                    let cond = check_line
                        .trim()
                        .strip_prefix("check:")
                        .unwrap_or("")
                        .trim();
                    let then_line = lines.next().unwrap_or("");
                    if !then_line.trim().starts_with("then:") {
                        continue;
                    }

                    let mut block = String::new();
                    while let Some(next) = lines.next() {
                        if next.trim() == format!("[/condition {}]", name) {
                            break;
                        }
                        block.push_str(next);
                        block.push('\n');
                    }

                    if self.evaluate_condition(cond) {
                        self.execute_logic(&block);
                    }
                }
            }
        }
    }

    fn parse_assignment(&mut self, raw: &str) -> Option<(String, i64)> {
        let mut parts = raw.trim().splitn(2, '=');
        let key = parts.next()?.trim().to_string();
        let val_expr = parts.next()?.trim();
        let value = self.parse_value(val_expr);
        Some((key, value))
    }

    fn parse_value(&self, expr: &str) -> i64 {
    let expr = expr.trim();
    if expr.starts_with("{{") && expr.ends_with("}}") {
        let inner = expr
            .trim_start_matches("{{")
            .trim_end_matches("}}")
            .trim();

        let mut tokens = inner.split_whitespace();
        if let (Some(var), Some(op), Some(num)) = (
            tokens.next(),
            tokens.next(),
            tokens.next(),
        ) {
            let val: i64 = num.parse().unwrap_or(0);
            let base = *self.memory.get(var).unwrap_or(&0);
            return match op {
                "+" => base + val,
                "-" => base - val,
                _ => base,
            };
        } else {
            return 0;
        }
    } else {
        expr.parse().unwrap_or(0)
    }
}


    fn evaluate_condition(&self, cond: &str) -> bool {
        let mut tokens = cond.split_whitespace();
        if let (Some(var), Some(op), Some(num_str)) = (tokens.next(), tokens.next(), tokens.next()) {
            let left = *self.memory.get(var).unwrap_or(&0);
            let right: i64 = num_str.parse().unwrap_or(0);
            match op {
                "<" => left < right,
                "<=" => left <= right,
                ">" => left > right,
                ">=" => left >= right,
                "==" => left == right,
                "!=" => left != right,
                _ => false,
            }
        } else {
            false
        }
    }
}

