use std::collections::HashSet;

pub fn count_ranges_that_contain_pair(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let range_a = a.split_once('-').unwrap();
            let range_b = b.split_once('-').unwrap();

            let range_a = (range_a.0.parse().unwrap(), range_a.1.parse().unwrap());
            let range_b = (range_b.0.parse().unwrap(), range_b.1.parse().unwrap());

            let set_a: HashSet<usize> = HashSet::from_iter(range_a.0..=range_a.1);
            let set_b: HashSet<usize> = HashSet::from_iter(range_b.0..=range_b.1);
            (set_a, set_b)
        })
        .filter(|(set_a, set_b)| set_a.is_subset(set_b) || set_a.is_superset(set_b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input");

    const BASIC_EXAMPLE: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn basic_example_should_result_in_2() {
        let result = count_ranges_that_contain_pair(BASIC_EXAMPLE);

        assert_eq!(result, 2)
    }

    #[test]
    fn input_should_result_in_value() {
        let result = count_ranges_that_contain_pair(INPUT);

        assert_eq!(result, 573);
    }
}
