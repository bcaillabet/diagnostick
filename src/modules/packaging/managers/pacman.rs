use std::process::{Command, Output};

pub fn get_packages() -> Vec<String> {
    let output: Output = Command::new("pacman")
        .arg("-Q")
        .output()
        .expect("Error while trying run pacman -Q");

    if output.status.code() != Some(0) {
        return ["undefined".to_string()].to_vec();
    }

    let output_str: String = String::from_utf8(output.stdout).unwrap();
    output_str.split("\n").map(|p| p.to_string()).collect()
}
