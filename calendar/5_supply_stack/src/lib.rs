mod collections;
mod cratemap;
mod error;
mod procedure;

pub use collections::{queue::Queue, stack::Stack};
use cratemap::CrateMap;
use error::{Error, ParseError};
pub use procedure::{Instruction, Procedure};

pub type Crate = char;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn collect_message(map: CrateMap) -> String {
    let mut chars: Vec<(String, char)> = Vec::new();

    for (key, mut stack) in map {
        if let Some(cr) = stack.pop() {
            chars.push((key, cr))
        }
    }

    chars.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));

    chars.iter().map(|(_, value)| value).collect()
}

pub fn rearrange_crates(input: &str) -> Result<CrateMap> {
    let (procedure, mut map) = match parse_input(input) {
        Ok(val) => val,
        Err(e) => return Err(Error::from(e)),
    };

    for Instruction {
        num_crates,
        source,
        target,
    } in procedure
    {
        let values = {
            // TODO should we .or_default() here?
            let source_stack = map.entry(source.clone()).or_default();

            let values: Vec<Crate> = (0..num_crates).filter_map(|_| source_stack.pop()).collect();
            values
        };

        let target_stack = map.entry(target.clone()).or_default();

        for value in values {
            target_stack.push(value)
        }
    }

    Ok(map)
}

/// Input is split into 2 parts, separated by 2 newlines "\n\n".
///
/// * Top half is a map of the crate stacks - [`CrateMap`]
/// * Bottom half is instructions to rearrange the stacks - [`Procedure`]
///
pub fn parse_input(input: &str) -> Result<(Procedure, CrateMap), ParseError> {
    let input_parts = input
        .split_once("\n\n")
        .expect(r"input should be separated by 2 newlines `\n\n`");

    let map: CrateMap = input_parts.0.parse()?;
    let procedure: Procedure = input_parts.1.parse()?;

    Ok((procedure, map))
}

#[cfg(test)]
mod tests {
    use super::{collect_message, rearrange_crates, Result};
    const INPUT: &str = include_str!("../input");

    const BASIC_EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn basic_example_should_result_in_cmz() -> Result<()> {
        let map = rearrange_crates(BASIC_EXAMPLE)?;
        let message = collect_message(map);

        assert_eq!(message, "CMZ");

        Ok(())
    }

    #[test]
    fn input_should_result_in_value() -> Result<()> {
        let map = rearrange_crates(INPUT)?;
        let message = collect_message(map);

        assert_eq!(message, "ZBDRNPMVH");

        Ok(())
    }

    #[test]
    fn part2_basic_example_should_result_in_cmz() -> Result<()> {
        let map = rearrange_crates(BASIC_EXAMPLE)?;
        let message = collect_message(map);

        assert_eq!(message, "MCD");

        Ok(())
    }

    #[test]
    fn part2_input_should_result_in_value() -> Result<()> {
        let map = rearrange_crates(INPUT)?;
        let message = collect_message(map);

        assert_eq!(message, "ZBDRNPMVH");

        Ok(())
    }
}
