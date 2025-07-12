mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::CapsuleLoader;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    let mut capsules = Vec::new();
    for file in ["core0.lore", "grammar.lore"] {
        let loader = CapsuleLoader::new(file);
        capsules.extend(loader.load_capsules());
    }
    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("boot.capsuleLoader");
}

