use std::collections::HashSet;

fn main() {
    let input_str = std::fs::read_to_string("src/day6/input.txt").unwrap();
    let mut input = input_str.chars().peekable();
    let mut total = 0;
    while input.peek().is_some() {
        let mut yes = HashSet::new();
        let mut me = HashSet::new();
        let mut first = true;
        while let Some(answer) = input.next() {
            dbg!(answer);
            // dbg!(answer);
            if answer == '\n' {
                println!("finished person");
                if first {
                    yes = me;
                    first = false;
                } else {
                    println!("Appending me to yes");
                    dbg!(&me, &yes);
                    yes = me.iter().filter_map(|v| yes.take(v)).collect();
                    println!("got yes");
                    dbg!(&yes);
                }
                me = HashSet::new();
                if let Some(&next) = input.peek() {
                    if next =='\n' {
                        input.next();
                        break
                    }
                }
            } else {
                me.insert(answer);
            }
        }
        total += yes.len() as i64;
        println!("Got group of {:?}", yes);
    }
    println!("total: {}", total)
}