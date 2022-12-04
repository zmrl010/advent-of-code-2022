use std::collections::HashSet;

pub fn calculate_items_sum(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| -> u32 {
            let a: HashSet<char> = HashSet::from_iter(a.chars());
            let b: HashSet<char> = HashSet::from_iter(b.chars());

            let items = a.iter().filter(|item| b.contains(item));

            items.map(get_item_value).sum()
        })
        .sum()
}

const LOWERCASE_COEFFICIENT: u32 = 96;
const UPPERCASE_COEFFICIENT: u32 = 38;

fn get_item_value(char: &char) -> u32 {
    let val = *char as u32;
    let val = match char {
        'a'..='z' => val - LOWERCASE_COEFFICIENT,
        'A'..='Z' => val - UPPERCASE_COEFFICIENT,
        _ => 0,
    };

    val
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn basic_example_should_result_in_157() {
        let result = calculate_items_sum(BASIC_EXAMPLE);

        assert_eq!(result, 157)
    }

    #[test]
    fn input_should_result_in_value() {
        let input = include_str!("../input");

        let result = calculate_items_sum(input);

        assert_eq!(result, 8105);
    }
}
