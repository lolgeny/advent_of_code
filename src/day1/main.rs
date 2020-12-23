use std::fs;

fn part1(input: Vec<i32>) {
    let mut start = 0usize;
    let mut end = input.len() - 1;
    while start < end {
        if input[start] + input[end] == 2020 {
            println!("{}", input[start]*input[end]);
            break;
        }
        if 2020 - input[start] < input[end] {
            end -= 1
        } else {
            start += 1
        }
    }
}

fn part2(input: Vec<i32>) {
    let mut a = 0;
    let mut b = 1;
    let mut c = input.len() - 1;
    while 2020 - input[c] < input[a] + input[b] {
        c -= 1;
    }
    c += 1;
    while 2020 - input[b] > input[a] + input[b] {
        b += 1;
    }
    b -= 1;
    while 2020 - input[a] != input[b] + input[c] {
        a += 1;
    }
    println!("{}", input[a]*input[b]*input[c]);
}

fn main() {
    let mut input = fs::read_to_string("src/day1/input.txt").unwrap().lines()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    input.sort();
    part2(input);
}