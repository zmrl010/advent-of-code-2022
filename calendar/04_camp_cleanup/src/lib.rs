mod id_range;

pub use id_range::{IdRange, Index};
use std::collections::HashSet;

pub type IdSet = HashSet<Index>;

/// Part 1
pub fn count_ranges(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .map(parse_id_sets)
        .filter(subset_or_superset)
        .count()
}

/// Part 2
pub fn count_ranges_intersect(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.trim())
        .map(parse_id_sets)
        .filter(sets_intersect)
        .count()
}

fn sets_intersect((a, b): &(IdSet, IdSet)) -> bool {
    !a.is_disjoint(b)
}

fn subset_or_superset((a, b): &(IdSet, IdSet)) -> bool {
    a.is_subset(b) || a.is_superset(b)
}

fn parse_id_sets(line: &str) -> (IdSet, IdSet) {
    let (a, b) = line.split_once(',').unwrap();
    let range_a: IdRange = a.parse().unwrap();
    let range_b: IdRange = b.parse().unwrap();

    let set_a: HashSet<Index> = HashSet::from_iter(range_a);
    let set_b: HashSet<Index> = HashSet::from_iter(range_b);
    (set_a, set_b)
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
        let result = count_ranges(BASIC_EXAMPLE);

        assert_eq!(result, 2)
    }

    #[test]
    fn part2_basic_example_should_result_in_4() {
        let result = count_ranges_intersect(BASIC_EXAMPLE);

        assert_eq!(result, 4)
    }

    #[test]
    fn input_should_result_in_value() {
        let result = count_ranges(INPUT);

        assert_eq!(result, 573);
    }

    #[test]
    fn part2_input_should_result_in_value() {
        let result = count_ranges_intersect(INPUT);

        assert_eq!(result, 867);
    }
}
