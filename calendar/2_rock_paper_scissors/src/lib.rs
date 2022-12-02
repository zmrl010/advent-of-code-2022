use anyhow::bail;

enum Weapon {
    Rock,
    Paper,
    Scissors,
}

enum Status {
    Win,
    Loss,
    Draw,
}

fn match_first_column(input: &str) {}

fn calculate_score(input: &str) {
    input.split("\n").map(|round| {
        let plays: Vec<&str> = round.split(" ").collect();
        let opponent = match plays[0] {
            "A" => Weapon::Rock,
            "B" => Weapon::Paper,
            "C" => Weapon::Scissors,
            _ => unreachable!(),
        };
        let player = match plays[1] {
            "X" => Weapon::Rock,
            "Y" => Weapon::Paper,
            "Z" => Weapon::Scissors,
            _ => unreachable!(),
        };
    });
}
