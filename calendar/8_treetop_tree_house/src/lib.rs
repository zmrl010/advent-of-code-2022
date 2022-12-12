mod grid;

#[derive(Debug)]
pub struct Tree {
    value: u8,
}

impl Tree {
    pub fn new(value: u8) -> Self {
        Self { value }
    }
}

fn count_visible_trees() {}

#[cfg(test)]
mod tests {

    const BASIC_INPUT: &str = "30373
25512
65332
33549
35390";

    const INPUT: &str = include_str!("../input");

    #[test]
    fn test_lib() {}
}
