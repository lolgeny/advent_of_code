use std::collections::HashMap;

fn next(numbers: &mut HashMap<i64, i64>) {
    // dbg!(&numbers);
    // shift up
    for (d, x) in numbers.clone() {
        numbers.insert(d + 1, x);
        // numbers.remove(&d);
    }
    // then do
    let prev = numbers[&1];
    // dbg!(i, &numbers, prev);
    let mut found = false;
    for j in (2..numbers.len() as i64).rev() {
        if numbers[&j] == prev {
            numbers.insert(0, j);
            found = true;
            break;
        }
    };
    if !found {
        numbers.insert(0, 0);
    }
}

fn main() {
    const TARGET: usize = 30000000;
    let input = std::fs::read_to_string("src/day15/input.txt").unwrap();
    let mut numbers: HashMap<i64, i64> = HashMap::new();
    let mut s = 0;
    for n in input.split(',')
        .map(|x| i64::from_str_radix(x, 10)) {
        numbers.insert(0, n.unwrap());
        // dbg!(&numbers);
        for (d, x) in numbers.clone() {
            numbers.insert(d + 1, x);
            // numbers.remove(&d);
        }
        // dbg!(&numbers);
        s += 1;
    }
    for _ in s..TARGET {
        next(&mut numbers);
    }
    dbg!(&numbers);
    println!("{}", numbers[&0])
}