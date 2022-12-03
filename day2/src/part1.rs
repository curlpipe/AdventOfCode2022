use std::fs::read_to_string;

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
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
}

fn main() {
    let data = read_to_string("input.txt").expect("File not found!");
    let result: u32 = data
        .split('\n')
        .filter_map(|line| line.split_once(' '))
        .map(|(them, us)| (Move::from_str(them), Move::from_str(us)))
        .map(|round| determine_score(round))
        .sum();
    println!("{:?}", result);
}

fn determine_score(round: (Move, Move)) -> u32 {
    let (them, us) = round;
    let base = us.base_value();
    let round = us.round_score(them);
    base + round
}
