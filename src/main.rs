mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::load_capsules_from_dir;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    println!("🧠 Loading Lumeria system...");

    // Load primary boot capsule
    let loader_primary = CapsuleLoader::new("core0.lore");
    let mut capsules = loader_primary.load_capsules();

    // Load supplemental boot capsules
    let loader_boot = CapsuleLoader::new("core0.boot.assembly.lore");
    capsules.extend(loader_boot.load_capsules());

    // Fallback: dynamically load any extra capsules from directory
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

