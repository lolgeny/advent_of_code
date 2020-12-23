
fn main() {
    let input = std::fs::read_to_string("src/day5/input.txt").unwrap();
    let mut seats = vec![];
    for line in input.lines() {
        let mut row = String::new();
        for i in 0..7 {
            row.push(match line.chars().nth(i).unwrap() {
                'F' => '0',
                'B' => '1',
                _ => panic!("Unknown row specifier")
            });
        }
        let row = i64::from_str_radix(row.as_str(), 2).unwrap();
        let mut column = String::new();
        for i in 7..10 {
            column.push(match line.chars().nth(i).unwrap() {
                'R' => '1',
                'L' => '0',
                _ => panic!("Unknown column specifier")
            });
        }
        let column = i64::from_str_radix(column.as_str(), 2).unwrap();
        let seat_id = row*8 + column;
        seats.push(seat_id);
    }
    seats.sort();
    let mut empty_seats = vec![];
    for seat in 0..seats.len() - 1 {
        if seats[seat + 1] > seats[seat] + 1 {
            empty_seats.push(seats[seat] + 1);
        }
    }
    println!("Empty: {:?}", empty_seats)
}