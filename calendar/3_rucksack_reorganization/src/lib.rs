use itertools::Itertools;
use std::collections::HashSet;

pub fn calculate_items_sum(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| (a.chars(), b.chars()))
        .map(|(a, b)| -> u32 {
            let a: HashSet<char> = HashSet::from_iter(a);
            let b: HashSet<char> = HashSet::from_iter(b);

            a.iter()
                .filter(|item| b.contains(item))
                .map(get_item_value)
                .sum()
        })
        .sum()
}

/// part 2 entrypoint to calculate common sum of priorities between items shared by the entire group.
/// Group is delimitated by group_size, but the puzzle sets it at 3
///
pub fn calculate_common_items_sum(input: &str, group_size: usize) -> u32 {
    input
        .lines()
        .chunks(group_size)
        .into_iter()
        .map(|group| -> u32 {
            group
                .map(|line| line.trim())
                .map(|line| -> HashSet<char> { HashSet::from_iter(line.chars()) })
                .reduce(
                    |intersection: HashSet<char>, set: HashSet<char>| -> HashSet<char> {
                        intersection.intersection(&set).cloned().collect()
                    },
                )
                .unwrap_or_default()
                .iter()
                .map(get_item_value)
                .sum()
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

    const INPUT: &str = include_str!("../input");

    #[test]
    fn basic_example_should_result_in_157() {
        let result = calculate_items_sum(BASIC_EXAMPLE);

        assert_eq!(result, 157)
    }

    #[test]
    fn part2_basic_example_should_result_in_70() {
        let result = calculate_common_items_sum(BASIC_EXAMPLE, 3);

        assert_eq!(result, 70)
    }

    #[test]
    fn input_should_result_in_value() {
        let result = calculate_items_sum(INPUT);

        assert_eq!(result, 8105);
    }

    #[test]
    fn part2_input_should_result_in_value() {
        let result = calculate_common_items_sum(INPUT, 3);

        assert_eq!(result, 2363);
    }
}
