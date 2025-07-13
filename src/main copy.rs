use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct SignalNode {
    file: String,
    line: usize,
    emits: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).map(String::as_str).unwrap_or(".");

    let mut graph: HashMap<String, Vec<SignalNode>> = HashMap::new();
    let mut triggers: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "lore").unwrap_or(false))
    {
        let path = entry.path();
        let file_content = fs::read_to_string(path).unwrap_or_default();
        let file = path.to_string_lossy().into_owned();
        let mut current_trigger = None;

        for (i, line) in file_content.lines().enumerate() {
            let line_num = i + 1;
            if let Some(stripped) = line.trim().strip_prefix("[trigger ") {
                if let Some(sig) = stripped.strip_suffix(" /]") {
                    triggers
                        .entry(sig.to_string())
                        .or_default()
                        .push((file.clone(), line_num));
                    current_trigger = Some(sig.trim().to_string());
                }
            }

            if line.contains("> emit:") {
                if let Some(start) = line.find("> emit:") {
                    let emitted = line[start + 7..].trim();
                    if let Some(trigger) = &current_trigger {
                        graph
                            .entry(trigger.clone())
                            .or_default()
                            .push(SignalNode {
                                file: file.clone(),
                                line: line_num,
                                emits: vec![emitted.to_string()],
                            });
                    }
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut stack = vec![];

    for sig in graph.keys() {
        detect_cycle(sig, &graph, &mut visited, &mut stack, &mut vec![]);
    }
}

fn detect_cycle(
    signal: &str,
    graph: &HashMap<String, Vec<SignalNode>>,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>,
    path: &mut Vec<(String, usize, String)>,
) {
    if stack.contains(&signal.to_string()) {
        let cycle_start = stack.iter().position(|s| s == signal).unwrap();
        let cycle: Vec<_> = path[cycle_start..]
            .iter()
            .map(|(file, line, sig)| format!("{} @ {} → {}", sig, file, line))
            .collect();

        println!("🔁 Infinite loop path:");
        for step in cycle {
            println!("  {}", step);
        }
        println!();
        return;
    }

    if let Some(nodes) = graph.get(signal) {
        stack.push(signal.to_string());
        for node in nodes {
            for emit in &node.emits {
                path.push((node.file.clone(), node.line, signal.to_string()));
                detect_cycle(emit, graph, visited, stack, path);
                path.pop();
            }
        }
        stack.pop();
    }
}