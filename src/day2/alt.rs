use std::str::Lines;
use aoc::parse;

pub (super) fn alt(input: Lines) -> i64 {
    let parser = parse! {
        int a nospace
        "-" nospace
        int b
        word c nospace
        ":"
        word password
    };
    let mut valid = 0;
    for line in input {
        let parse_result = parser.parse(line.into()).unwrap();
        let a = parse_result["a"].as_int() as usize;
        let b = parse_result["b"].as_int() as usize;
        let c = parse_result["c"].as_str().chars().next().unwrap();
        let password = parse_result["password"].as_str();
        if (password.chars().nth(a-1).unwrap() == c) ^ (password.chars().nth(b-1).unwrap() == c) {
            valid += 1;
        }
    };
    valid
}