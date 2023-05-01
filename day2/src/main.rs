#![feature(slice_take)]

use std::env;
use crate::Outcome::{Draw, ILose, IWin};
use crate::Play::{Paper, Rock, Scissors};

#[derive(Copy, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    IWin,
    ILose,
    Draw,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", a(input));

    let input = std::fs::read_to_string(filename).unwrap();
    println!("{}", b(input));
}

fn get_play_a(play_code: &str) -> Play {
    // could use TryFrom trait on the Play struct
    match play_code {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => panic!("Invalid play code")
    }
}

fn get_plays_b(opponents_play_code: &str, my_outcome_code: &str) -> (Play, Play) {
    let opponents_play = match opponents_play_code {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        _ => panic!("Invalid opponent's play code")
    };

    let my_outcome = match my_outcome_code {
        "X" => ILose,
        "Y" => Draw,
        "Z" => IWin,
        _ => panic!("Invalid outcome code")
    };

    match (opponents_play, my_outcome) {
        // could factor out into what beats what with 3 items
        (Rock, ILose) => (Rock, Scissors),
        (Rock, Draw) => (Rock, Rock),
        (Rock, IWin) => (Rock, Paper),
        (Paper, ILose) => (Paper, Rock),
        (Paper, Draw) => (Paper, Paper),
        (Paper, IWin) => (Paper, Scissors),
        (Scissors, ILose) => (Scissors, Paper),
        (Scissors, Draw) => (Scissors, Scissors),
        (Scissors, IWin) => (Scissors, Rock),
    }
}

fn get_match_outcome(opponents_play: Play, my_play: Play) -> Outcome {
    match (opponents_play, my_play) {
        (Rock, Paper) => IWin,
        (Rock, Scissors) => ILose,
        (Paper, Rock) => ILose,
        (Paper, Scissors) => IWin,
        (Scissors, Rock) => IWin,
        (Scissors, Paper) => ILose,
        _ => Draw,
    }
}

fn a(input: String) -> usize {
    let mut my_score = 0;

    for line in input.split("\n") {
        let mut plays: Vec<_> = line.split(' ').map(get_play_a).collect();

        assert!(plays.len() >= 2);
        // could use iter::next rather than Vec::remove
        let opponents_play = plays.remove(0);
        let my_play = plays.remove(0);

        let outcome = get_match_outcome(opponents_play, my_play);
        my_score += match outcome {
            IWin => 6,
            ILose => 0,
            Draw => 3,
        };

        // could put bonus points on the Move struct itself
        my_score += match my_play {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    my_score
}

fn b(input: String) -> usize {
    let mut my_score = 0;

    for line in input.split("\n") {
        let mut codes: Vec<_> = line.split(' ').collect();

        assert!(codes.len() >= 2);
        let opponents_play_code = codes.remove(0);
        let my_outcome_code = codes.remove(0);
        let (opponents_play, my_play) = get_plays_b(&opponents_play_code, &my_outcome_code);

        let outcome = get_match_outcome(opponents_play, my_play);
        my_score += match outcome {
            IWin => 6,
            ILose => 0,
            Draw => 3,
        };
        my_score += match my_play {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    my_score
}

#[test]
fn test_example_a() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(a(input), 15);
}

#[test]
fn test_example_b() {
    let input = std::fs::read_to_string("src/example_data.txt").unwrap();
    assert_eq!(b(input), 12);
}