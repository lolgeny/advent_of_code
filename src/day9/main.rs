#[allow(dead_code)]
fn part1(input: Vec<i64>) {
    let mut preamble = Vec::from(&input[..25]);
    let mut pointer = 25;
    loop {
        let mut found = false;
        'search:for a in 0..preamble.len() {
            for b in a+1..preamble.len() {
                if preamble[a] + preamble[b] == input[pointer] {
                    found = true;
                    break 'search;
                }
            }
        }
        if found {
            preamble.remove(0);
            preamble.push(input[pointer]);
            pointer += 1;
        } else {
            println!("{} is invalid", input[pointer]);
            break;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("src/day9/input.txt").unwrap()
        .lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap();
    for a in 0..input.len() {
        for b in a+1..input.len() {
            if input[a..b].iter().sum::<i64>() == 731031916 {
                println!(
                    "Product {}",
                    input[a..b].iter().min().unwrap() + input[a..b].iter().max().unwrap()
                );
            }
        }
    }
}