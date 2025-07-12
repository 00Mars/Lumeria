use crate::lumeria_loader::Capsule;
use std::collections::HashMap;

pub struct LumeriaRuntime {
    capsules: Vec<Capsule>,
    memory: HashMap<String, i64>,
}

impl LumeriaRuntime {
    pub fn new(capsules: Vec<Capsule>) -> Self {
        Self { capsules, memory: HashMap::new() }
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
        let mut lines = logic.lines().peekable();
        while let Some(raw) = lines.next() {
            let line = raw.trim();
            if let Some(rest) = line.strip_prefix("> log:") {
                let msg = rest.trim().trim_matches('"');
                println!("{}", msg);
            } else if let Some(rest) = line.strip_prefix("> emit:") {
                let sig = rest.trim();
                self.emit(sig);
            } else if let Some(rest) = line.strip_prefix("> default:") {
                if let Some((k,v)) = self.parse_assignment(rest) {
                    self.memory.entry(k).or_insert(v);
                }
            } else if let Some(rest) = line.strip_prefix("> memory.set:") {
                if let Some((k,v)) = self.parse_assignment(rest) {
                    self.memory.insert(k, v);
                }
            } else if line.starts_with("[condition") {
                if let Some(end_idx) = line.find(']') {
                    let name = line[10..end_idx].trim();
                    let check_line = lines.next().unwrap_or("");
                    let cond = check_line.trim().strip_prefix("check:").unwrap_or("").trim();
                    let then_line = lines.next().unwrap_or("");
                    if !then_line.trim().starts_with("then:") { continue; }
                    let mut block = String::new();
                    while let Some(next) = lines.next() {
                        if next.trim() == format!("[/condition {}]", name) { break; }
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

    fn parse_assignment(&mut self, raw: &str) -> Option<(String,i64)> {
        let mut parts = raw.trim().splitn(2,'=');
        let key = parts.next()?.trim().to_string();
        let val_expr = parts.next()?.trim();
        let value = self.parse_value(val_expr);
        Some((key,value))
    }

    fn parse_value(&self, expr: &str) -> i64 {
        let expr = expr.trim();
        if expr.starts_with("{{") && expr.ends_with("}}"){ 
            let inner = expr.trim_start_matches("{{").trim_end_matches("}}").trim();
            let mut tokens = inner.split_whitespace();
            if let (Some(var), Some(op), Some(num)) = (tokens.next(), tokens.next(), tokens.next()) {
                let val: i64 = num.parse().unwrap_or(0);
                let base = *self.memory.get(var).unwrap_or(&0);
                return match op {
                    "+" => base + val,
                    "-" => base - val,
                    _ => base,
                };
            }
            0
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

