mod lumeria_loader;
mod lumeria_runtime;

use lumeria_loader::CapsuleLoader;
use lumeria_runtime::LumeriaRuntime;

fn main() {
    let capsules = CapsuleLoader::load_dir(".");
    let mut runtime = LumeriaRuntime::new(capsules);
    runtime.emit("boot.assembly");
}

