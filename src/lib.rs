use std::process::Command;
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
    (3, "hundred"), (4, "thousand"), (7, "million"), (10, "billion"), 
    (13, "trillion"), (16, "quadrillion"), (19, "quintillion"), 
    (22, "sextillion"), (25, "septillion"), (28, "octillion"), 
    (31, "nonillion"), (34, "decillion"), (37, "undecillion"), 
    (40, "duodecillion"), (43, "tredecillion"), (46, "quattuordecillion"), 
    (49, "quindecillion"), (52, "sexdecillion"), (55, "septendecillion"), 
    (58, "octodecillion"), (61, "novemdecillion"), (64, "vigintillion")
].iter().cloned().collect();



pub fn encode(n: u64) -> String {
    match n {
        0..=19 => numbers.get(&n).unwrap().to_string(),
        20..=99 => {
            let tens_digit = tens.get(&(n / 10)).unwrap().to_string();
            let ones_digit = numbers.get(&(n % 10)).unwrap().to_string();
            if n % 10 == 0 {
                tens_digit
            } else {
                format!("{}-{}", tens_digit, ones_digit)
            }
        }
        100..=999 => {
            let hundreds_digit = numbers.get(&(n / 100)).unwrap().to_string();
            let tens_digit = encode(n % 100);
            if n % 100 == 0 {
                format!("{} {}", hundreds_digit, bases.get(&3).unwrap())
            } else {
                format!("{} {} {}", hundreds_digit, bases.get(&3).unwrap(), tens_digit)
            }
        }
        1000..=999_999 => {
            let thousands_digit = encode(n / 1000);
            let hundreds_digit = encode(n % 1000);
            if n % 1000 == 0 {
                format!("{} {}", thousands_digit, bases.get(&4).unwrap())
            } else {
                format!("{} {} {}", thousands_digit, bases.get(&4).unwrap(), hundreds_digit)
            }
        }
        1_000_000..=999_999_999 => {
            let millions_digit = encode(n / 1_000_000);
            let thousands_digit = encode(n % 1_000_000);
            if n % 1_000_000 == 0 {
                format!("{} {}", millions_digit, bases.get(&7).unwrap())
            } else {
                format!("{} {} {}", millions_digit, bases.get(&7).unwrap(), thousands_digit)
            }
        }
        1_000_000_000..=999_999_999_999 => {
            let billions_digit = encode(n / 1_000_000_000);
            let millions_digit = encode(n % 1_000_000_000);
            if n % 1_000_000_000 == 0 {
                format!("{} {}", billions_digit, bases.get(&10).unwrap())
            } else {
                format!("{} {} {}", billions_digit, bases.get(&10).unwrap(), millions_digit)
            }
        }
        _ => panic!("Number too large")
    }

}

pub fn speak(n: &str) -> String {
    let output = Command::new("espeak")
        .arg(n)
        .output()
        .expect("failed to execute process");

    String::from_utf8(output.stdout).unwrap()
}