fn update((x, y): (usize, usize), lookup: &Vec<Vec<i32>>, grid: &mut Vec<Vec<i32>>) -> bool {
    if lookup[x][y] == 0 {return false}
    let mut total_neighbours = 0;
    for neighbour in &[
        (-1, 0),
        (1, 0),
        (-1, 1),
        (1, 1),
        (0, 1),
        (0, -1),
        (-1, -1),
        (1, -1)
    ] {
        let mut nx = x as isize + neighbour.0;
        let mut ny = y as isize + neighbour.1;
        while nx >= 0 && nx < lookup.len() as isize && ny >= 0 && ny < lookup[0].len() as isize && lookup[nx as usize][ny as usize] == 0 {
            nx += neighbour.0;
            ny += neighbour.1;
        }
        if nx >= 0 && nx < lookup.len() as isize && ny >= 0 && ny < lookup[0].len() as isize && lookup[nx as usize][ny as usize] == 2 {total_neighbours += 1}
    }
    let mut changed = false;
    if total_neighbours == 0 {
        changed |= grid[x][y] == 1;
        grid[x][y] = 2;
    } else if total_neighbours >= 5 {
        changed |= grid[x][y] == 2;
        grid[x][y] = 1;
    }
    return changed;
}

fn main() {
    let input = std::fs::read_to_string("src/day11/input.txt").unwrap();
    let mut grid = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            row.push(match c {
                '.' => 0,
                'L' => 1,
                _ => panic!("Unknown seat")
            });
        }
        grid.push(row);
    }
    let mut old_grid = grid.clone();
    // let mut first = true;
    loop {
        let mut changed = false;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                changed |= update((row, col), &old_grid, &mut grid);
            }
        }
        old_grid = grid.clone();
        if !changed {break}
        // println!("Passed an iteration");
        // for row in grid.iter() {
        //     for seat in row {
        //         print!("{}", match seat {
        //             0 => '.',
        //             1 => 'L',
        //             2 => '#',
        //             _ => unreachable!()
        //         });
        //     }
        //     println!();
        // }
        // if !first {break};
        // first = false;
    }
    let mut total_seats = 0;
    for row in grid {
        for seat in row {
            if seat == 2 {total_seats += 1}
        }
    }
    println!("Total {}", total_seats)
}