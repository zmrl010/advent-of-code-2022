pub fn part1(_input: &str) -> usize {
    todo!()
}

pub fn part2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_result_eq_expected() {
        let result = part1(BASIC_INPUT);

        assert_eq!(result, 0);
    }

    #[test]
    fn part1_input_result_eq_expected() {
        let result = part1(INPUT);

        assert_eq!(result, 0);
    }

    #[test]
    fn part2_basic_input_result_eq_expected() {
        let result = part2(BASIC_INPUT);

        assert_eq!(result, 0)
    }

    #[test]
    fn part2_input_result_eq_expected() {
        let result = part2(INPUT);

        assert_eq!(result, 0);
    }
}
