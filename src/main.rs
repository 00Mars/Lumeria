mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::load_capsules_from_dir;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    println!("🧠 Loading Lumeria system...");

    let capsules = load_capsules_from_dir("./Lumeria");

    println!("✅ Loaded {} capsules:", capsules.len());
    for capsule in &capsules {
        println!("🔗 Capsule: {}", capsule.name);
        for trigger in &capsule.triggers {
            println!("   ↳ Trigger: {}", trigger);
        }
    }

    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("boot.capsuleLoader");
}
