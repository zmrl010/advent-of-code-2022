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

pub fn calculate_player_score(input: &str) -> u16 {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.split(" ");
            let (opponent, player) = parts.next().zip(parts.next()).expect("expected two values");
            let opponent = match opponent {
                "A" => Weapon::Rock,
                "B" => Weapon::Paper,
                "C" => Weapon::Scissors,
                _ => unreachable!(),
            };
            let player = match player {
                "X" => Weapon::Rock,
                "Y" => Weapon::Paper,
                "Z" => Weapon::Scissors,
                _ => unreachable!(),
            };

            let player_score = match player.cmp(&opponent) {
                Ordering::Less => 0u16,
                Ordering::Equal => 3u16,
                Ordering::Greater => 6u16,
            };

            player.value() + player_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    const BASIC_EXAMPLE: &str = "A Y\nB X\nC Z";

    fn read_input() -> anyhow::Result<String> {
        let mut input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        input_path.push("input");

        let input = fs::read_to_string(input_path)?;

        Ok(input)
    }

    #[test]
    fn basic_example_should_total_to_15() {
        let result = calculate_player_score(BASIC_EXAMPLE);

        assert_eq!(result, 15)
    }

    #[test]
    fn puzzle_input_should_equal_answer() -> anyhow::Result<()> {
        let input = read_input()?;

        let result = calculate_player_score(&input);

        assert_eq!(result, 10404);

        Ok(())
    }
}
