use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug)]
enum Player {
    Player1,
    Player2
}
use Player::*;

static mut GAME_ID: i64 = 0;

fn combat(mut player1: Vec<i64>, mut player2: Vec<i64>) -> (Player, Vec<i64>) {
    let mut states = HashSet::<(Vec<i64>, Vec<i64>)>::new(); // <(Vec<player1's cards>, Vec<player2's cards>)>
    let id;
    unsafe {
        id = GAME_ID;
        GAME_ID += 1;
    }
    // println!("Running game {}", id);
    while !player1.is_empty() && !player2.is_empty() {
        if states.contains(&(player1.clone(), player2.clone())) {
            // println!("Finished game, winner is Player1 with deck {:?} due to recursion", &player1);
            return (Player1, player1);
        }
        states.insert((player1.clone(), player2.clone()));
        // println!("Starting round, player 1 has deck {:?} and player 2 has deck {:?}", &player1, player2);
        let a = player1.remove(0);
        let b = player2.remove(0);
        // println!("Player 1 plays {} and player 2 plays {}", a, b);
        let winner = if a > player1.len() as i64 || b > player2.len() as i64 {
            match a > b {
                true => Player1,
                false => Player2
            }
        } else {
            combat(
                player1.iter().take(a as usize).map(|v| *v).collect(),
                player2.iter().take(b as usize).map(|v| *v).collect()
            ).0
        };
        // println!("{:?} wins this round", &winner);
        match winner {
            Player1 => {
                player1.push(a);
                player1.push(b);
            },
            Player2 => {
                player2.push(b);
                player2.push(a);
            }
        }
    };
    let winner = match player1.is_empty() {
        true => Player2,
        false => Player1
    };
    let winner_deck = match winner {
        Player1 => player1,
        Player2 => player2
    };
    // println!("Finished game, winner is {:?} with deck {:?}", &winner, &winner_deck);
    (winner, winner_deck)
}

fn score(mut winner: Vec<i64>) -> i64 {
    let mut pos = 0;
    winner.reverse();
    winner.iter().fold(0, |a, b| {pos += 1; a + pos*b})
}

fn main() {
    let player1 = read_to_string("src/day22/player1.txt").unwrap().lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap();
    let player2 = read_to_string("src/day22/player2.txt").unwrap().lines().map(str::parse).collect::<Result<Vec<i64>, _>>().unwrap();
   /* while !player1.is_empty() && !player2.is_empty() {
        let a = player1.remove(0);
        let b = player2.remove(0);
        if a > b {
            player1.push(a);
            player1.push(b);
        } else {
            player2.push(b);
            player2.push(a);
        }
    }
    let mut winner = if player1.is_empty() {player2} else {player1};
    println!("Score {}", score);*/
    let winner = combat(player1, player2).1;
    println!("Score {}", score(winner));
}