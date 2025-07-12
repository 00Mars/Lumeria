mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::CapsuleLoader;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    let loader_primary = CapsuleLoader::new("core0.lore");
    let mut capsules = loader_primary.load_capsules();

    // Load additional boot assembly capsules
    let loader_boot = CapsuleLoader::new("core0.boot.assembly.lore");
    capsules.extend(loader_boot.load_capsules());

    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("boot.assembly");
}

