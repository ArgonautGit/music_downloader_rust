use std::process::Command;

fn main() {
    let process = Command::new("echo")
        .arg("TEST")
        .output()
        .expect("Failed to execute process.");

    let output = String::from_utf8_lossy(&process.stdout);
    let output = output.trim();

    println!("{output}");
}
