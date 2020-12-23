use aoc::parse;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum MaskMode {
    Keep,
    Set,
    Float
}

fn main() {
    let input = std::fs::read_to_string("src/day14/input.txt").unwrap();
    let mem_parser = parse! {
        "mem[" nospace
        int address nospace
        "] ="
        int value
    };
    let mask_parser = parse! {
        "mask ="
        custom mask (char::is_alphanumeric)
    };
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut mask = vec![];
    for line in input.lines() {
        dbg!(line);
        if let Some(mem) = mem_parser.parse(line.into()) {
            println!("mem");
            let value = mem["value"].as_int();
            let address = mem["address"].as_int();
            let mut result_address = vec![address];
            // dbg!(mask.len(), &result_address);
            for index in 0..mask.len() {
                // dbg!(index, &mask[index], result_address.len());
                match mask[index] {
                    MaskMode::Keep => {},
                    MaskMode::Set => for a in 0..result_address.len() {
                        result_address[a] |= 1 << index;
                    },
                    MaskMode::Float => for a in 0..result_address.len() {
                        result_address.push(result_address[a] | (1 << index));
                        result_address[a] &= !(1 << index)
                    }
                }
                // dbg!(index, &mask[index], &result_address);
            }
            for a in result_address {
                memory.insert(a, value);
            }
        } else {
            let mask_result = mask_parser.parse(line.into()).unwrap();
            println!("mask");
            mask = vec![];
            for c in mask_result["mask"].as_str().chars() {
                mask.insert(0, match c {
                    '1' => MaskMode::Set,
                    '0' => MaskMode::Keep,
                    'X' => MaskMode::Float,
                    _ => panic!()
                });
            }
            dbg!(&mask);
        }
    }
    let mut sum = 0i128;
    for n in memory.values() {
        sum = sum.checked_add(*n as i128).unwrap();
    }
    println!("{}", sum)
}