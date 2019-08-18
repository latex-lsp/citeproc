use std::process::Command;

fn main() {
    Command::new("node")
        .arg("js/build.js")
        .status()
        .unwrap();
}
