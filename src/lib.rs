use std::process::Command;
use std::collections::HashMap;
use std::collections::HashMap;

let numbers: HashMap<u64, &str> = [
    (0, "zero"), (1, "one"), (2, "two"), (3, "three"), (4, "four"), 
    (5, "five"), (6, "six"), (7, "seven"), (8, "eight"), (9, "nine"), 
    (10, "ten"), (11, "eleven"), (12, "twelve"), (13, "thirteen"), 
    (14, "fourteen"), (15, "fifteen"), (16, "sixteen"), (17, "seventeen"), 
    (18, "eighteen"), (19, "nineteen")
].iter().cloned().collect();

let tens: HashMap<u64, &str> = [
    (2, "twenty"), (3, "thirty"), (4, "forty"), (5, "fifty"), 
    (6, "sixty"), (7, "seventy"), (8, "eighty"), (9, "ninety")
].iter().cloned().collect();

let bases: HashMap<u64, &str> = [
    (2, "hundred"), (3, "thousand"), 
    (6, "million"), (9, "billion"), 
    (12, "trillion")
].iter().cloned().collect();



pub fn encode(n: u64) -> String {
    
}

pub fn speak(n: &str) -> String {
    let output = Command::new("espeak")
        .arg(n)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}