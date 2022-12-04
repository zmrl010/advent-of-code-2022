use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

#[derive(PartialEq, Eq, Clone)]
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

    /// Return a copy of the weapon
    pub fn get_equal(&self) -> Self {
        self.clone()
    }

    /// Return weapon that is weak to self
    pub fn get_lesser(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    /// Return weapon that is strong against self
    pub fn get_greater(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
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

/// Decrypt left column
/// * A for Rock
/// * B for Paper
/// * C for Scissors
fn decrypt_left(col: &str) -> Weapon {
    match col {
        "A" => Weapon::Rock,
        "B" => Weapon::Paper,
        "C" => Weapon::Scissors,
        _ => unreachable!(),
    }
}

/// Decrypt right column (part 1)
/// * X for Rock
/// * Y for Paper
/// * Z for Scissors
fn decrypt_right(col: &str) -> Weapon {
    match col {
        "X" => Weapon::Rock,
        "Y" => Weapon::Paper,
        "Z" => Weapon::Scissors,
        _ => unreachable!(),
    }
}

/// Decrypt right column (part 2)
/// * X for Less
/// * Y for Equal
/// * Z for Greater
fn decrypt_right_ord(col: &str) -> Ordering {
    match col {
        "X" => Ordering::Less,
        "Y" => Ordering::Equal,
        "Z" => Ordering::Greater,
        _ => unreachable!(),
    }
}

fn split_line_parts(line: &str) -> (&str, &str) {
    let mut parts = line.split(" ");

    parts.next().zip(parts.next()).expect("expected two values")
}

fn parse_line(line: &str) -> (Weapon, Weapon) {
    let (opponent, player) = split_line_parts(line);

    (decrypt_left(opponent), decrypt_right(player))
}

/// altered line parser after we are told that the 2nd column is actually
/// whether you should win, lose, or draw
///
/// see: [`decrypt_right_ord`]
fn parse_line_altered(line: &str) -> (Weapon, Weapon) {
    let (opponent, player) = split_line_parts(line);

    let (opponent, player) = (decrypt_left(opponent), decrypt_right_ord(player));

    let player = match player {
        Ordering::Equal => opponent.get_equal(),
        Ordering::Less => opponent.get_lesser(),
        Ordering::Greater => opponent.get_greater(),
    };

    (opponent, player)
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

pub fn calculate_score_part2(input: &str) -> u16 {
    input
        .trim()
        .split("\n")
        .map(parse_line_altered)
        .map(calculate_round_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &str = "A Y\nB X\nC Z";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_example_should_total_to_15() {
        let result = calculate_score(BASIC_EXAMPLE);

        assert_eq!(result, 15);
    }

    #[test]
    fn puzzle_input_should_equal_answer() {
        let result = calculate_score(INPUT);

        assert_eq!(result, 10404);
    }

    #[test]
    fn part2_basic_example_should_total_to_12() {
        let result = calculate_score_part2(BASIC_EXAMPLE);

        assert_eq!(result, 12);
    }

    #[test]
    fn part2_puzzle_input_should_equal_answer() {
        let result = calculate_score_part2(INPUT);

        assert_eq!(result, 10334);
    }
}
