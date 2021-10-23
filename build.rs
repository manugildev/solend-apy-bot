use std::process::Command;

fn main() {
    // Use npm to build the web project
    Command::new("npm").current_dir("web/").args(&["run", "build"]).spawn().unwrap();
}