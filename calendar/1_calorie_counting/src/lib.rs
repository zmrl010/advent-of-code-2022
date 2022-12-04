//! # Advent of Code 2022 Day 1
//! Page: https://adventofcode.com/2022/day/1
//! Input: https://adventofcode.com/2022/day/1/input

type Integer = u32;

/// Calculate the sum of each calorie cluster and find the largest one
pub fn find_max_calorie_cluster(input: &str) -> Option<Integer> {
    input.split("\n\n").map(calculate_cluster_sum).max()
}

/// Calculate the sum of each calorie cluster and return the sum of the top n from those
pub fn sum_n_max_calorie_clusters(input: &str, n: usize) -> Integer {
    let mut sums = input
        .split("\n\n")
        .map(calculate_cluster_sum)
        .collect::<Vec<Integer>>();
    sums.sort_unstable_by(|a, b| b.cmp(a));
    sums.iter().take(n).sum()
}

/// Take a line separated list of integers and calculate the sum of the entire cluster
///
/// # Arguments
///
/// * `input` - Line separated list of integers
///
/// # Returns
///
/// Sum of each line in the cluster
///
pub fn calculate_cluster_sum(input: &str) -> Integer {
    input
        .split("\n")
        .filter_map(|value| -> Option<Integer> { value.trim().parse().ok() })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &str =
        "1000 \n2000 \n3000 \n\n4000 \n\n5000 \n6000 \n\n7000 \n8000 \n9000 \n\n10000 \n";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn find_max_calorie_cluster_should_be_24000_in_basic_example() {
        let result = find_max_calorie_cluster(BASIC_EXAMPLE);

        assert_eq!(result, Some(24000));
    }

    #[test]
    fn find_3_max_calorie_clusters_sum_should_be_45000_in_basic_example() {
        let result = sum_n_max_calorie_clusters(BASIC_EXAMPLE, 3);

        assert_eq!(result, 45000);
    }

    #[test]
    fn should_find_max_calorie_cluster_input() -> anyhow::Result<()> {
        let result = find_max_calorie_cluster(INPUT);

        assert_eq!(result, Some(72602));

        Ok(())
    }

    #[test]
    fn should_find_3_max_calorie_clusters_sum_using_input() -> anyhow::Result<()> {
        let result = sum_n_max_calorie_clusters(INPUT, 3);

        assert_eq!(result, 207410);

        Ok(())
    }
}
