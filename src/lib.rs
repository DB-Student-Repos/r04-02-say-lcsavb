use std::process::Command;
use std::collections::HashMap;

let base_numbers: HashMap<u64, String> = HashMap::new();

let base_numbers = [(0, "zero"), (1, "one", 1), (2, "two", 2), (3, "three", 3), (4, "four", 4), 
                (5, "five", 5), (6, "six", 6), (7, "seven", 7), (8, "eight", 8), 
                (9, "nine", 9), (10, "ten", 10), (11, "eleven", 11), (12, "twelve", 12), 
                (13, "thirteen", 13), (14, "fourteen", 14), (15, "fifteen", 15), 
                (16, "sixteen", 16), (17, "seventeen", 17), (18, "eighteen", 18), (19, "nineteen", 19)];

let tens: HashMap<u64, String> = HashMap::new();

let tens = [(20, "twenty", 20), (30, "thirty", 30), (40, "forty", 40), (50, "fifty", 50), 
            (60, "sixty", 60), (70, "seventy", 70), (80, "eighty", 80), (90, "ninety", 90)];



pub fn encode(n: u64) -> String {
    match n.len() {
        1 | 2 => {
            return numbers.get(&n).unwrap().to_string()
        
    }
}


pub fn speak(n: &str) -> String {
    let output = Command::new("espeak")
        .arg(n)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}