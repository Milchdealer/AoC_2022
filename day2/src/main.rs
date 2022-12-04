use std::convert::From;
use std::fs;

#[derive(PartialEq, Clone)]
enum Moves {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Outcome {
    LOSE,
    DRAW,
    WIN,
}

#[derive(Clone)]
struct Move {
    pub _move: Moves,
    pub points: u32,
}

impl Move {
    fn rock() -> Self {
        Move {
            _move: Moves::ROCK,
            points: 1,
        }
    }

    fn paper() -> Self {
        Move {
            _move: Moves::PAPER,
            points: 2,
        }
    }

    fn scissors() -> Self {
        Move {
            _move: Moves::SCISSORS,
            points: 3,
        }
    }
}

impl From<Moves> for Move {
    fn from(item: Moves) -> Self {
        match item {
            Moves::ROCK => Move::rock(),
            Moves::PAPER => Move::paper(),
            Moves::SCISSORS => Move::scissors(),
        }
    }
}

fn wins_against(_move: Moves) -> Moves {
    match _move {
        Moves::ROCK => Moves::SCISSORS,
        Moves::PAPER => Moves::ROCK,
        Moves::SCISSORS => Moves::PAPER,
    }
}

fn loses_against(_move: Moves) -> Moves {
    match _move {
        Moves::ROCK => Moves::PAPER,
        Moves::PAPER => Moves::SCISSORS,
        Moves::SCISSORS => Moves::ROCK,
    }
}

fn translate_move(input_move: &str) -> Move {
    match input_move {
        "A" | "X" => Move::rock(),
        "B" | "Y" => Move::paper(),
        "C" | "Z" => Move::scissors(),
        &_ => todo!(),
    }
}

fn translate_outcome(input_outcome: &str) -> Outcome {
    match input_outcome {
        "X" => Outcome::LOSE,
        "Y" => Outcome::DRAW,
        "Z" => Outcome::WIN,
        &_ => todo!(),
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");

    let result = content
        .split("\n")
        .map(|line| {
            (
                line.get(0..1).unwrap_or_default(),
                line.get(2..3).unwrap_or_default(),
            )
        })
        .map(|moves| (translate_move(moves.0), translate_move(moves.1)))
        .map(|(opponent, me)| {
            if wins_against(me.clone()._move) == opponent._move {
                me.points + 6
            } else if me._move == opponent._move {
                me.points + 3
            } else {
                me.points
            }
        })
        .reduce(|acc, item| acc + item);

    println!("{:?}", result);

    let result = content
        .split("\n")
        .map(|line| {
            (
                line.get(0..1).unwrap_or_default(),
                line.get(2..3).unwrap_or_default(),
            )
        })
        .map(|moves| (translate_move(moves.0), translate_outcome(moves.1)))
        .map(|(opponent, outcome)| match outcome {
            Outcome::LOSE => Move::from(wins_against(opponent._move)).points,
            Outcome::DRAW => opponent.points + 3,
            Outcome::WIN => Move::from(loses_against(opponent._move)).points + 6,
        })
        .reduce(|acc, item| acc + item);

    println!("{:?}", result);
}
