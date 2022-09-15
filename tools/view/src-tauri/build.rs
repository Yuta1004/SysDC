use std::process::Command;

fn main() {
    Command::new("npm")
        .args(&["run", "build"])
        .current_dir("../")
        .status()
        .unwrap();

    tauri_build::build();

    println!("cargo:rerun-if-changed=../src");
    println!("cargo:rerun-if-changed=../node_modules");
    println!("cargo:rerun-if-changed=../package.json");
}
