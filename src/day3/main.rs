use std::fs;

fn slopes(gradient: (usize, usize), grid: &Vec<Vec<bool>>) -> i64 {
    let mut pos = (0, 0);
    let mut trees = 0;
    while pos.1 < grid.len() {
        if pos.0 >= grid[0].len() {
            pos.0 -= grid[0].len();
        }
        if grid[pos.1][pos.0] == true {
            trees += 1;
        }
        pos.0 += gradient.0;
        pos.1 += gradient.1;
    }
    trees
}

fn main() {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown character")
            });
        }
        grid.push(row);
    }
    let mut trees = 1i64;
    for gradient in vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ] {
        trees *= slopes(gradient, &grid)
    }
    println!("Trees: {}", trees)
}