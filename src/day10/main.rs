use std::collections::HashMap;
use std::cell::RefCell;
use cache::cache;

#[cache(current)]
fn arrangements(current: i64, rest: &[i64]) -> i64 {
    if rest.len() == 0 {
        1
    } else {
        let mut total = 0;
        for possible in 0..3.min(rest.len()) {
            if rest[possible] - current <= 3 {
                total += arrangements(rest[possible], &rest[possible+1..]/*, cache*/);
            }
        }
        total
    }
}

fn main() {
    let input_str = std::fs::read_to_string("src/day10/input.txt").unwrap();
    let mut input = input_str.lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap();
    input.sort();
    // let mut j1 = 0;
    // let mut j3 = 0;
    // for i in 1..input.len() {
    //     match input[i] - input[i-1] {
    //         1 => j1 += 1,
    //         3 => j3 += 1,
    //         _ => {}
    //     }
    // }
    // j1 += 1; j3 += 1;
    // println!("1v: {}, 3v: {}, product: {}", j1, j3, j1*j3);
    println!("combinations: {}", arrangements(0, &input/*, &RefCell::new(HashMap::new())*/))
}