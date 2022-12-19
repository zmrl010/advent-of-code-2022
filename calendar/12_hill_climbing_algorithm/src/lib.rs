pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_eq_31() {}

    #[test]
    fn part1_basic_input_eq_value() {}

    #[test]
    fn part2_basic_input_eq_31() {}

    #[test]
    fn part2_basic_input_eq_value() {}

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
