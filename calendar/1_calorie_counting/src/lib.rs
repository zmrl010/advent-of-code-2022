//! # Advent of Code 2022 Day 1
//! Page: https://adventofcode.com/2022/day/1
//! Input: https://adventofcode.com/2022/day/1/input

type Integer = u32;

pub fn find_largest_calorie_cluster(input: &str) -> Option<Integer> {
    input.split("\n\n").map(calculate_cluster_sum).max()
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
    use std::{env, fs, path::PathBuf};

    use anyhow::{anyhow, Context};

    use crate::find_largest_calorie_cluster;

    const BASIC_EXAMPLE: &str =
        "1000 \n2000 \n3000 \n\n4000 \n\n5000 \n6000 \n\n7000 \n8000 \n9000 \n\n10000 \n";

    #[test]
    fn should_find_largest_cluster_is_24000_in_basic_example() {
        let result = find_largest_calorie_cluster(BASIC_EXAMPLE);

        assert_eq!(result, Some(24000));
    }

    #[test]
    fn should_compute_input() -> anyhow::Result<()> {
        let input_path = {
            let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            dir.push("input");
            dir
        };

        let input = fs::read_to_string(&input_path)
            .with_context(|| anyhow!("failed to read file at `{}`", input_path.display()))?;
        let result = find_largest_calorie_cluster(input.as_str());

        assert_eq!(result, Some(72602));

        Ok(())
    }
}
