use std::process::Command;

fn main() {
    // Tell Cargo that if the /web/src folder changes, rerun this build script.
    println!("cargo:rerun-if-changed=web/src/");
    // Use npm to build the web project
    Command::new("npm").current_dir("web/").args(&["run", "build"]).spawn().unwrap();
}