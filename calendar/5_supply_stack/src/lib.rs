mod cratemap;
mod procedure;
mod stack;
mod supply_crate;

pub use stack::CrateStack;
pub use supply_crate::Crate;

pub fn rearrange_crates(input: &str) -> Crate {
    Crate::new('C')
}

/// Input is split into 2 parts, separated by 2 newlines "\n\n".
///
/// * Top half is the starting arrangement of the crate stacks - [`parse_stacks`]
/// * Bottom half is instructions to rearrange the stacks - [`parse_procedure`]
///
pub fn parse_input(input: &str) {
    let input_parts = input
        .split_once("\n\n")
        .expect(r"input should be separated by 2 newlines `\n\n`");
}

#[cfg(test)]
mod tests {
    use super::*;

    // const INPUT: &str = include_str!("../input");

    const BASIC_EXAMPLE: &str = "    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    #[test]
    fn basic_example_should_result_in_cmz() {
        let result = rearrange_crates(BASIC_EXAMPLE);

        assert_eq!(result, Crate::new('C'))
    }
}
