#![feature(trace_macros)]

mod alt;

use std::fs;

fn read_int(s: &mut String) -> i32 {
    let mut i = String::from(s.remove(0));
    let next = s.chars().next().unwrap();
    if next.is_digit(10) {
        i.push(s.remove(0));
    }
    i.parse::<i32>().unwrap()
}

fn main() {
    let input = fs::read_to_string("src/day2/input.txt").unwrap();
    /*let mut valid = 0;
    for line in input.lines() {
        let mut password = String::from(line);
        let range_min = read_int(&mut password) as usize;
        password.remove(0);
        let range_max = read_int(&mut password) as usize;
        password.remove(0);
        let c = password.remove(0);
        password.remove(0);
        password.remove(0);
        let first = password.chars().nth(range_min-1).unwrap();
        let second = password.chars().nth(range_max-1).unwrap();
        dbg!(first, second);
        if (password.chars().nth(range_min-1).unwrap() == c) ^ (c == password.chars().nth(range_max-1).unwrap()) {
            println!("{} is valid", line);
            valid += 1;
        } else {
            println!("{} is not valid", line);
        }
    }*/
    println!("{}", alt::alt(input.lines()));
}