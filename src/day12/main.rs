use aoc::parse;
use std::f64::consts::PI;

fn rotate(direction: (i64, i64), anticlockwise: f64) -> (i64, i64) {
    // dbg!(direction, anticlockwise);
    let direction = (direction.0 as f64, direction.1 as f64);
    (
        (direction.0 * anticlockwise.cos() - direction.1 * anticlockwise.sin()).round() as i64,
        (direction.0 * anticlockwise.sin() + direction.1 * anticlockwise.cos()).round() as i64
    )
}

fn main() {
    assert_eq!(rotate((10, 4), 270f64.to_radians()), (4, -10));
    let input = std::fs::read_to_string("src/day12/input.txt").unwrap();
    let parser = parse! {
        word instruction nospace
        int magnitude
    };
    let mut position = (0, 0);
    let mut waypoint = (10, 1);
    for line in input.lines() {
        let parse_result = parser.parse(line.into()).unwrap();
        let instruction = parse_result["instruction"].as_str();
        let magnitude = parse_result["magnitude"].as_int();
        dbg!(&instruction, &magnitude);
        match instruction.as_str() {
            "N" => waypoint.1 += magnitude,
            "S" => waypoint.1 += -magnitude,
            "E" => waypoint.0 += magnitude,
            "W" => waypoint.0 += -magnitude,
            "F" => position = (position.0 + magnitude*waypoint.0, position.1 + magnitude*waypoint.1),
            "L" => waypoint = rotate(waypoint, (magnitude as f64).to_radians()),
            "R" => waypoint = rotate(waypoint, (360.0 - magnitude as f64).to_radians()),
            _ => panic!("Unknown instruction")
        }
        dbg!(&position, &waypoint);
    }
    println!("At {}, {}, so total of {}", position.0, position.1, position.0.abs() + position.1.abs())
}