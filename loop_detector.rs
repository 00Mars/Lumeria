use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct SignalSpot {
    signal: String,
    kind: String, // "emit", "trigger", "when"
    file: PathBuf,
    line_number: usize,
}

fn main() {
    let mut signal_map: HashMap<String, Vec<SignalSpot>> = HashMap::new();
    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();

    let root = Path::new(".");
    let lore_files = find_lore_files(root);

    for file in lore_files {
        if let Ok(contents) = fs::read_to_string(&file) {
            for (i, line) in contents.lines().enumerate() {
                let l = line.trim();

                if let Some(signal) = l.strip_prefix("[trigger ") {
                    let sig = signal.trim_end_matches(" /]").to_string();
                    record(&mut signal_map, "trigger", &sig, &file, i + 1);
                } else if let Some(signal) = l.strip_prefix("[when ") {
                    let sig = signal.trim_end_matches("]").trim().to_string();
                    record(&mut signal_map, "when", &sig, &file, i + 1);
                } else if let Some(signal) = l.strip_prefix("> emit:") {
                    let sig = signal.trim().to_string();
                    record(&mut signal_map, "emit", &sig, &file, i + 1);
                    edges.entry(sig.clone()).or_default();
                }
            }
        }
    }

    // Build edges from emit -> trigger/when
    for (signal, spots) in &signal_map {
        for spot in spots {
            if spot.kind == "emit" {
                if let Some(targets) = signal_map.get(signal) {
                    for target in targets {
                        if target.kind == "trigger" || target.kind == "when" {
                            edges.entry(signal.clone())
                                 .or_default()
                                 .insert(signal.clone()); // self edge
                        }
                    }
                }
            }
        }
    }

    // Detect loops
    println!("🔍 Detecting unguarded recursion loops...");
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    for signal in edges.keys() {
        detect_loops(signal, &edges, &mut visited, &mut stack, &signal_map);
    }
}

fn find_lore_files(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if root.is_dir() {
        for entry in fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                result.extend(find_lore_files(&path));
            } else if path.extension().map(|s| s == "lore").unwrap_or(false) {
                result.push(path);
            }
        }
    }
    result
}

fn record(
    map: &mut HashMap<String, Vec<SignalSpot>>,
    kind: &str,
    signal: &str,
    file: &Path,
    line: usize,
) {
    map.entry(signal.to_string())
        .or_default()
        .push(SignalSpot {
            signal: signal.to_string(),
            kind: kind.to_string(),
            file: file.to_path_buf(),
            line_number: line,
        });
}

fn detect_loops<'a>(
    signal: &'a str,
    edges: &'a HashMap<String, HashSet<String>>,
    visited: &mut HashSet<&'a str>,
    stack: &mut Vec<&'a str>,
    signal_map: &HashMap<String, Vec<SignalSpot>>,
) {
    if stack.contains(&signal) {
        // Found a loop
        println!("\n🚨 Infinite loop detected on signal: '{}'", signal);
        for sig in stack.iter().chain(std::iter::once(&signal)) {
            if let Some(spots) = signal_map.get(*sig) {
                for spot in spots {
                    println!(
                        "  → [{}:{}] {}: {}",
                        spot.file.display(),
                        spot.line_number,
                        spot.kind,
                        spot.signal
                    );
                }
            }
        }
        return;
    }

    if visited.contains(signal) {
        return;
    }

    visited.insert(signal);
    stack.push(signal);

    if let Some(neighbors) = edges.get(signal) {
        for next in neighbors {
            detect_loops(next, edges, visited, stack, signal_map);
        }
    }

    stack.pop();
}