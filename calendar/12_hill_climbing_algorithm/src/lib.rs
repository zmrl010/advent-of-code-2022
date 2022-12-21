mod graph;

use graph::{Graph, ParseError};

pub fn part1(input: &str) -> Result<usize, ParseError> {
    let graph: Graph = input.parse()?;

    let result = graph::find_shortest_path(graph).expect("no path found");

    Ok(result.0.len() - 1)
}

pub fn part2(input: &str) -> Result<usize, ParseError> {
    let graph: Graph = input.parse()?;

    let result = graph::find_shortest_path(graph).expect("no path found");

    Ok(result.0.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_eq_31() -> Result<(), ParseError> {
        let result = part1(BASIC_INPUT)?;

        assert_eq!(result, 31);

        Ok(())
    }

    #[test]
    fn part1_basic_input_eq_value() -> Result<(), ParseError> {
        let result = part1(INPUT)?;

        assert_eq!(result, 394);

        Ok(())
    }

    #[test]
    fn part2_basic_input_eq_29() -> Result<(), ParseError> {
        let result = part2(BASIC_INPUT)?;

        assert_eq!(result, 29);

        Ok(())
    }

    #[test]
    fn part2_basic_input_eq_value() -> Result<(), ParseError> {
        let result = part2(INPUT)?;

        assert_eq!(result, 394);

        Ok(())
    }
}
