use lumeria::lumeria_loader::CapsuleLoader;
use lumeria::mnemonic_engine::MnemonicEngine;

fn main() {
    println!("🧠 Loading capsules for standalone engine...");
    let mut capsules = Vec::new();
    for file in ["core0.lore", "grammar.lore", "core0.boot.assembly.lore"] {
        let loader = CapsuleLoader::new(file);
        capsules.extend(loader.load_capsules());
    }
    let recursive = CapsuleLoader::load_dir(".");
    capsules.extend(recursive);

    println!("✅ {} capsules loaded", capsules.len());

    let mut engine = MnemonicEngine::new(capsules);
    engine.emit("boot.assembly");
}
