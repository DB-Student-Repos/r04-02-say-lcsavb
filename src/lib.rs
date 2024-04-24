use std::process::Command;

pub fn encode(n: u64) -> String {
    match n.len() {
        2 => 
    }
}


pub fn speak(n: &str) -> String {
    let output = Command::new("espeak")
        .arg(n)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}