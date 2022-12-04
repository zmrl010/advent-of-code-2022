use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(PartialEq, Eq)]
pub enum Weapon {
    Rock,
    Paper,
    Scissors,
}

impl Weapon {
    pub fn value(&self) -> u16 {
        match self {
            Weapon::Rock => 1,
            Weapon::Paper => 2,
            Weapon::Scissors => 3,
        }
    }
}

impl PartialOrd for Weapon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Weapon {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        use Weapon::*;

        match self {
            Rock => match other {
                Rock => Equal,
                Paper => Less,
                Scissors => Greater,
            },
            Paper => match other {
                Rock => Greater,
                Paper => Equal,
                Scissors => Less,
            },
            Scissors => match other {
                Rock => Less,
                Paper => Greater,
                Scissors => Equal,
            },
        }
    }
}

fn parse_left_col(col: &str) -> Weapon {
    match col {
        "A" => Weapon::Rock,
        "B" => Weapon::Paper,
        "C" => Weapon::Scissors,
        _ => unreachable!(),
    }
}

fn parse_right_col(col: &str) -> Weapon {
    match col {
        "X" => Weapon::Rock,
        "Y" => Weapon::Paper,
        "Z" => Weapon::Scissors,
        _ => unreachable!(),
    }
}

fn parse_line(line: &str) -> (Weapon, Weapon) {
    let mut parts = line.split(" ");

    let (opponent, player) = parts.next().zip(parts.next()).expect("expected two values");

    (parse_left_col(opponent), parse_right_col(player))
}

fn calculate_round_score((opponent, player): (Weapon, Weapon)) -> u16 {
    let player_score: u16 = match player.cmp(&opponent) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };

    player.value() + player_score
}

pub fn calculate_score(input: &str) -> u16 {
    input
        .trim()
        .split("\n")
        .map(parse_line)
        .map(calculate_round_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &str = "A Y\nB X\nC Z";

    #[test]
    fn basic_example_should_total_to_15() {
        let result = calculate_score(BASIC_EXAMPLE);

        assert_eq!(result, 15);
    }

    #[test]
    fn puzzle_input_should_equal_answer() {
        let input = include_str!("../input");

        let result = calculate_score(input);

        assert_eq!(result, 10404);
    }
}
