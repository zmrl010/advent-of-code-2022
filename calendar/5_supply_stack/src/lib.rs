mod collections;
mod error;
mod procedure;

use std::iter::Extend;

pub use collections::cratemap::CrateMap;
use error::{Error, ParseError};
pub use procedure::{Instruction, Procedure};

pub type Crate = char;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn collect_message(map: CrateMap<Vec<Crate>>) -> String {
    let mut chars: Vec<(String, char)> = map
        .into_iter()
        .filter_map(|(key, crates)| {
            let cr = crates.last()?;
            Some((key, *cr))
        })
        .collect();

    // keys are only still here so we can sort at the end (rethink this)
    chars.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));
    // iterate again to remove keys
    chars.iter().map(|(_, value)| value).collect()
}

fn move_crates_one_by_one(instruction: Instruction, map: &mut CrateMap<Vec<Crate>>) {
    let Instruction {
        num_crates,
        source,
        target,
    } = instruction;

    let source_values = {
        let source: &mut Vec<Crate> = map.entry(source.clone()).or_default();

        /*
            we need an intermediary vec so we can store the values and drop the mutable
            reference to source in the HashMap before we can get another mutable reference
            to the target in the HashMap. This avoids needing any fancy smart pointers
        */
        let mut values: Vec<Crate> = Vec::with_capacity(num_crates as usize);

        let mut count = num_crates;
        while count > 0 {
            if let Some(value) = source.pop() {
                values.push(value);
            }

            count -= 1;
        }

        values
    };

    map.entry(target.clone()).or_default().extend(source_values);
}

fn move_crates_all(instruction: Instruction, map: &mut CrateMap<Vec<Crate>>) {
    let Instruction {
        num_crates,
        source,
        target,
    } = instruction;
    let num_crates = num_crates as usize;

    let source_values = {
        let source: &mut Vec<Crate> = map.entry(source.clone()).or_default();

        let at = source.len() - num_crates.min(source.len());

        let values = source.split_off(at);

        values
    };

    map.entry(target.clone()).or_default().extend(source_values);
}

pub fn rearrange_crates(input: &str) -> Result<CrateMap<Vec<Crate>>> {
    let (procedure, mut map) = match parse_input(input) {
        Ok(val) => val,
        Err(e) => return Err(Error::from(e)),
    };

    for instruction in procedure {
        move_crates_one_by_one(instruction, &mut map);
    }

    Ok(map)
}

/// Part 2 - using a queue data structure to retain order
pub fn rearrange_crates_part2(input: &str) -> Result<CrateMap<Vec<Crate>>> {
    let (procedure, mut map) = match parse_input(input) {
        Ok(val) => val,
        Err(e) => return Err(Error::from(e)),
    };

    for instruction in procedure {
        move_crates_all(instruction, &mut map)
    }

    Ok(map)
}

/// Input is split into 2 parts, separated by 2 newlines "\n\n".
///
/// * Top half is a map of the crate stacks - [`CrateMap`]
/// * Bottom half is instructions to rearrange the stacks - [`Procedure`]
///
pub fn parse_input(input: &str) -> Result<(Procedure, CrateMap<Vec<Crate>>), ParseError> {
    let input_parts = input
        .split_once("\n\n")
        .expect(r"input should be separated by 2 newlines `\n\n`");

    let map: CrateMap<Vec<Crate>> = input_parts.0.parse()?;
    let procedure: Procedure = input_parts.1.parse()?;

    Ok((procedure, map))
}

#[cfg(test)]
mod tests {
    use super::{collect_message, rearrange_crates, rearrange_crates_part2, Result};
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
    fn part2_basic_example_should_result_in_mcd() -> Result<()> {
        let map = rearrange_crates_part2(BASIC_EXAMPLE)?;
        let message = collect_message(map);

        assert_eq!(message, "MCD");

        Ok(())
    }

    #[test]
    fn part2_input_should_result_in_value() -> Result<()> {
        let map = rearrange_crates_part2(INPUT)?;
        let message = collect_message(map);

        assert_eq!(message, "WDLPFNNNB");

        Ok(())
    }
}
