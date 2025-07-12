mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::CapsuleLoader;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    let loader = CapsuleLoader::new("core0.lore");
    let mut capsules = loader.load_capsules();
    let loader_extra = CapsuleLoader::new("runtime.orphanTriggers.lore");
    capsules.extend(loader_extra.load_capsules());
    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("boot.capsuleLoader");
}

