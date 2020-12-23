use std::fs;
use std::iter::Peekable;
use std::str::Chars;

fn get_str(input: &mut Peekable<Chars>) -> String {
    let mut key = String::from(input.next().unwrap());
    while let Some(&next) = input.peek() {
        if !next.is_alphanumeric() && next != '#' {
            break
        }
        key.push(input.next().unwrap());
    }
    key
}

fn main() {
    let input_string = fs::read_to_string("src/day4/input.txt").unwrap();
    let mut input = input_string.chars().peekable();
    let mut valid = 0;
    let mut keys = vec![];
    let required_keys = vec![
        "byr".into(), "iyr".into(), "eyr".into(), "hgt".into(), "hcl".into(), "ecl".into(), "pid".into()
    ];
    while input.peek().is_some() {
        let key = get_str(&mut input);
        dbg!(&key);
        assert_eq!(input.next().unwrap(), ':', "{:?}", input);
        let mut value = get_str(&mut input);
        dbg!(&value);
        if match key.as_str() {
            "byr" => {
                let val = value.parse::<i32>().unwrap();
                1920 <= val && val <= 2002
            },
            "iyr" => {
                let val = value.parse::<i32>().unwrap();
                2010 <= val && val <= 2020
            },
            "eyr" => {
                let val = value.parse::<i32>().unwrap();
                2020 <= val && val <= 2030
            },
            "hgt" => {
                let mut unit = String::from(value.pop().unwrap());
                unit.insert(0, value.pop().unwrap());
                if let Ok(val) = value.parse::<i32>() {
                    match unit.as_str() {
                        "cm" => 150 <= val && 193 >= val,
                        "in" => 59 <= val && 76 >= val,
                        _ => false
                    }
                } else {false}
            },
            "hcl" => {
                if value.remove(0) == '#' {
                    let mut res = true;
                    for c in value.chars() {
                        match c {
                            '0'..='9' => {}
                            'a'..='f' => {}
                            _ => {res = false}
                        }
                    }
                    res
                } else {
                    false
                }
            },
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|&x| x.contains(value.as_str())),
            "pid" => value.len() == 9 && !value.chars().any(|x| !x.is_digit(10)),
            _ => true
        } {
            keys.push(key);
        }
        input.next();
        if let Some(&next) = input.peek() {
            if next == '\n' {
                input.next();
                let mut correct = true;
                for key in &required_keys {
                    if !keys.contains(key) {
                        correct = false;
                        println!("Oh no, {:?} is wrong", keys);
                    }
                    // keys.remove(keys.binary_search(key).unwrap());
                }
                if correct { valid += 1 };
                keys = vec![];
            }
        }
    }
    println!("{}", valid);
}