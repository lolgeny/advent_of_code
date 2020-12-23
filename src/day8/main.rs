use std::collections::HashSet;

fn get_parts(input: &String) -> (String, i64) {
    let mut input = input.chars().peekable();
    let mut instruction = String::new();
    loop {
        instruction.push(input.next().unwrap());
        if *input.peek().unwrap() == ' ' {break}
    }
    input.next();
    let num = input.collect::<String>();
    (instruction, num.parse::<i64>().unwrap())
}

#[derive(Clone)]
enum Instruction {
    Nop(usize),
    Acc(i64),
    Jmp(usize)
}

fn main() {
    let input_str = std::fs::read_to_string("src/day8/input.txt").unwrap();
    let input: Vec<String> = input_str.lines().map(Into::into).collect();
    let mut instructions = vec![];
    for line in input.iter() {
        let (instruction, num) = get_parts(line);
        instructions.push(match instruction.as_str() {
            "nop" => Instruction::Nop(num as usize),
            "jmp" => Instruction::Jmp(num as usize),
            "acc" => Instruction::Acc(num),
            _ => panic!("Unknown instruction")
        });
    }
    'instruction_test:for special in 0..instructions.len() {
        let mut pointer = 0;
        let mut acc = 0;
        let mut visited = HashSet::new();
        while pointer < input.len() {
            if visited.contains(&pointer) {continue 'instruction_test}
            visited.insert(pointer);
            let mut instruction = instructions[pointer].clone();
            if pointer == special {
                instruction = match instruction {
                    Instruction::Nop(num) => Instruction::Jmp(num),
                    Instruction::Jmp(num) => Instruction::Nop(num),
                    x => x
                };
            }
            pointer += match instruction {
                Instruction::Nop(_) => 1,
                Instruction::Jmp(num) => num,
                Instruction::Acc(num) => {
                    acc += num;
                    1
                }
            }
        }
        println!("acc {}", acc)
    }
}