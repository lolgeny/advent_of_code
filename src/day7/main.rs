use std::collections::HashMap;
use aoc::cache::cache;

#[allow(dead_code)]
fn can_contain_shiny(rule: String, rules: &HashMap<String, Vec<(u32, String)>>) -> bool {
    for content in rules.get(rule.as_str()).unwrap() {
        if content.1 == "shiny gold" {
            return true;
        } else {
            if can_contain_shiny(content.1.clone(), rules) {return true}
        }
    }
    return false;
}

#[cache(rule)]
fn bags_inside(rule: String, rules: &HashMap<String, Vec<(u32, String)>>) -> i64 {
    let mut total = 0;
    for content in rules.get(rule.as_str()).unwrap() {
        total += content.0 as i64 * (bags_inside(content.1.clone(), rules) + 1);
    }
    total
}

fn main() {
    let input = std::fs::read_to_string("src/day7/input.txt").unwrap();
    let mut rules = HashMap::new();
    let start = std::time::Instant::now();
    for line in input.lines() {
        let mut c = line.chars().peekable();
        let mut name = String::new();
        while *c.peek().unwrap() != ' ' {name.push(c.next().unwrap());}
        name.push(c.next().unwrap());
        while *c.peek().unwrap() != ' ' {name.push(c.next().unwrap());}
        c.next();
        while *c.peek().unwrap() != ' ' {c.next();}
        c.next();
        while *c.peek().unwrap() != ' ' {c.next();}
        c.next();
        let mut contents = vec![];
        if *c.peek().unwrap() != 'n' {
            while c.peek().is_some() {
                let number = c.next().unwrap();
                c.next();
                let mut color = String::new();
                while *c.peek().unwrap() != ' ' {color.push(c.next().unwrap());}
                color.push(c.next().unwrap());
                while *c.peek().unwrap() != ' ' {color.push(c.next().unwrap());}
                c.next();
                while *c.peek().unwrap_or(&' ') != ' ' {c.next();}
                c.next();
                contents.push((number.to_digit(10).unwrap(), color));
            }
        }
        rules.insert(name, contents);
    }
    /*let mut total = 0;
    for rule in rules.keys() {
        if can_contain_shiny(rule.clone(), &rules) {total += 1}
    }*/
    println!("Total {}", bags_inside("shiny gold".into(), &rules));
    let duration = std::time::Instant::now().duration_since(start);
    println!("Total time {:?}", duration)
}