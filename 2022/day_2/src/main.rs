use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");

    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn problem_one(input: &str) -> usize {
    rps_iterator(input)
        .flat_map(|(opponent, your_hand)| {
            let your_hand: Hand = your_hand.parse().ok()?;
            let opponent: Hand = opponent.parse().ok()?;

            Some(your_hand.get_score() + your_hand.get_outcome_for_hand(opponent).get_score())
        })
        .sum()
}

fn problem_two(input: &str) -> usize {
    rps_iterator(input)
        .flat_map(|(opponent, outcome)| {
            let opponent: Hand = opponent.parse().ok()?;
            let outcome: Outcome = outcome.parse().ok()?;

            Some(outcome.get_score() + opponent.get_hand_for_outcome(outcome).get_score())
        })
        .sum()
}

/// Prepare the input for the problems
fn rps_iterator(input: &str) -> impl Iterator<Item = (&str, &str)> + '_ {
    input.split("\n").flat_map(|moves| moves.split_once(" "))
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    /// Returns the outcome of the hand against the opponent's hand.
    fn get_outcome_for_hand(&self, opponent: Hand) -> Outcome {
        match (self, opponent) {
            (hand, opponent) if hand == &opponent.wins_against() => Outcome::Lose,
            (hand, opponent) if hand == &opponent.loses_against() => Outcome::Win,
            _ => Outcome::Draw,
        }
    }

    /// Find the hand that would result in the given outcome
    fn get_hand_for_outcome(&self, outcome: Outcome) -> Hand {
        match (self, outcome) {
            (opponent, Outcome::Win) => opponent.loses_against(),
            (opponent, Outcome::Lose) => opponent.wins_against(),
            (opponent, _) => opponent.clone(),
        }
    }
}

impl Score for Hand {
    /// The score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
    fn get_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Hand {
    fn wins_against(&self) -> Self {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Score for Outcome {
    /// The score for the outcome of the round 0 if you lost, 3 if the round was a draw,
    /// and 6 if you won.
    fn get_score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

trait Score {
    fn get_score(&self) -> usize;
}
