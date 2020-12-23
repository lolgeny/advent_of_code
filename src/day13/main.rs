use rayon::prelude::*;
use std::time::Instant;

static mut REQUIREMENTS: Vec<i128> = Vec::new();
static mut TIME: Option<Instant> = None;
unsafe fn verify(time: i128) -> bool {
    if time % 100000000 == 0 {
        println!("{} - {:?}", time, Instant::now().duration_since(TIME.unwrap()));
    }
    let first_leave = 23 * (time/23);
    // if first_leave != time {return false}
    for r in 1..REQUIREMENTS.len() {
        if REQUIREMENTS[r] == -1 {continue}
        let expected = first_leave + r as i128;
        if expected % REQUIREMENTS[r] != 0 {return false}
    };
    true
}

fn main() {
    let input_str = std::fs::read_to_string("src/day13/input.txt").unwrap();
    let mut input = input_str.lines();
    input.next();
    unsafe {
        TIME = Some(Instant::now());
        for id in input.next().unwrap().split(',') {
            REQUIREMENTS.push(match id {
                "x" => -1,
                n => n.parse().unwrap()
            });
        }
    }
    // 782000000000 - 100188000000000 - 100431000000000
    let times = 100238600000000..;
    times.step_by(23).par_bridge().into_par_iter().for_each(|x| unsafe {if verify(x) {
        println!("{} is success", x);
        std::process::exit(0);
    }});
}