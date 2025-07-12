mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::load_capsules_from_dir;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    println!("🧠 Loading Lumeria system...");

    // Load explicitly defined core and grammar capsules
    let mut capsules = Vec::new();
    for file in ["core0.lore", "grammar.lore", "core0.boot.assembly.lore"] {
        let loader = CapsuleLoader::new(file);
        capsules.extend(loader.load_capsules());
    }

    // Dynamically load additional `.lore`, `.loot`, or `.arena` capsules from subdirectories
    let recursive_capsules = CapsuleLoader::load_dir(".");
    capsules.extend(recursive_capsules);

    println!("✅ Loaded {} capsules:", capsules.len());
    for capsule in &capsules {
        println!("🔗 Capsule: {}", capsule.name);
        for trigger in &capsule.triggers {
            println!("   ↳ Trigger: {}", trigger);
        }
    }

    // Start execution
    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("boot.assembly");
}

