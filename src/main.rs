mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::load_capsules_from_dir;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    println!("🧠 Loading Lumeria system...");

    // Load explicitly defined core files
    let mut capsules = Vec::new();
    for file in ["core0.lore", "grammar.lore", "core0.boot.assembly.lore"] {
        let loader = CapsuleLoader::new(file);
        capsules.extend(loader.load_capsules());
    }

    // Fallback: dynamically load any extra capsules from ./Lumeria
    let extra_capsules = load_capsules_from_dir("./Lumeria");
    capsules.extend(extra_capsules);

    println!("✅ Loaded {} capsules:", capsules.len());
    for capsule in &capsules {
        println!("🔗 Capsule: {}", capsule.name);
        for trigger in &capsule.triggers {
            println!("   ↳ Trigger: {}", trigger);
        }
    }

    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("go");
}

