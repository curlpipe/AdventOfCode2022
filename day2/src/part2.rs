use std::fs::read_to_string;

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid format!"),
        }
    }
}

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Invalid format!"),
        }
    }

    pub fn base_value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn round_score(&self, other: Self) -> u32 {
        match (self, other) {
            (Self::Rock, Self::Paper) => 0,
            (Self::Paper, Self::Scissors) => 0,
            (Self::Scissors, Self::Rock) => 0,
            (Self::Rock, Self::Rock) => 3,
            (Self::Paper, Self::Paper) => 3,
            (Self::Scissors, Self::Scissors) => 3,
            (Self::Rock, Self::Scissors) => 6,
            (Self::Paper, Self::Rock) => 6,
            (Self::Scissors, Self::Paper) => 6,
        }
    }

    pub fn from_round(other: &Move, outcome: Outcome) -> Self {
        match (other, outcome) {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Lose) => Self::Scissors,
            (Self::Rock, Outcome::Draw) => Self::Rock,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Lose) => Self::Rock,
            (Self::Paper, Outcome::Draw) => Self::Paper,
            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Lose) => Self::Paper,
            (Self::Scissors, Outcome::Draw) => Self::Scissors,
        }
    }
}

fn main() {
    let data = read_to_string("input.txt").expect("File not found!");
    let result: u32 = data
        .split('\n')
        .filter_map(|line| line.split_once(' '))
        .map(|(them, outcome)| (Move::from_str(them), Outcome::from_str(outcome)))
        .map(determine_score)
        .sum();
    println!("{:?}", result);
}

fn determine_score(round: (Move, Outcome)) -> u32 {
    let (them, outcome) = round;
    let us = Move::from_round(&them, outcome);
    let base = us.base_value();
    let round = us.round_score(them);
    base + round
}
