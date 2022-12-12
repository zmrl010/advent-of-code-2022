mod filesystem;

use filesystem::{Node, NodeIndex, NodeTable};

fn parse_input(input: &str) -> anyhow::Result<NodeTable> {
    let mut nodes = NodeTable::new();

    let mut current_dir_index = nodes.add_dir("/");

    for line in input.trim().lines() {
        let mut parts = line.split(' ');

        match parts.nth(0).unwrap() {
            "$" => match parts.nth(0).unwrap() {
                "cd" => {
                    let node = &nodes[current_dir_index];

                    let target_dir = parts.nth(0).unwrap();

                    match target_dir {
                        ".." => {
                            if let Some(node_index) = node.parent() {
                                current_dir_index = node_index
                            }
                        }
                        "/" => {}
                        _ => {
                            if let Node::Directory(ref cwd) = node {
                                let child =
                                    cwd.iter().find(|&child| nodes[*child].name() == target_dir);

                                if let Some(target) = child {
                                    current_dir_index = *target
                                } else {
                                    eprintln!("directory {target_dir} was not found.")
                                }
                            }
                        }
                    }
                }
                // "ls" => {} we can ignore `ls` because a shift in contexts
                // would complicate things and our logic already handles the requirements
                _ => continue,
            },
            "dir" => {
                let target_dir = parts.nth(0).unwrap();

                let index = nodes.add_dir(target_dir);

                nodes[index].set_parent(current_dir_index);

                if let Node::Directory(cwd) = &mut nodes[current_dir_index] {
                    cwd.push(index)
                }
            }
            value if value.chars().all(char::is_numeric) => {
                let size: u64 = value.parse()?;

                let name = parts.nth(0).unwrap();

                let index = nodes.add_file(name, size);

                nodes[index].set_parent(current_dir_index);

                if let Node::Directory(cwd) = &mut nodes[current_dir_index] {
                    cwd.push(index)
                }
            }
            _ => continue,
        }
    }

    Ok(nodes)
}

const SIZE_LIMIT_PART1: u64 = 100_100;

pub fn calculate_dir_sums_under_limit(input: &str) -> anyhow::Result<u64> {
    let node_table = parse_input(input)?;

    let total: u64 = node_table
        .iter()
        .filter(|&node| matches!(node, Node::Directory(_)))
        .map(|dir| dir.size(&node_table))
        .filter(|size| *size < SIZE_LIMIT_PART1)
        .sum();

    Ok(total)
}

const TOTAL_AVAILABLE_SPACE: u64 = 70_000_000;
const MINIMUM_SPACE_NEEDED: u64 = 30_000_000;
const MAXIMUM_SPACE_FOR_USE: u64 = TOTAL_AVAILABLE_SPACE - MINIMUM_SPACE_NEEDED;

/// **Part 2** - Find a directory to delete; One that lets us reach our
/// goal with the minimal possible
pub fn find_smallest_viable_dir_size(input: &str) -> anyhow::Result<u64> {
    let node_table = parse_input(input)?;

    let total_space_used = node_table[0.into()].size(&node_table);

    let mut dir_size: u64 = 0;
    let mut max_potential_new_size = 0;

    for node in node_table
        .iter()
        .filter(|&node| matches!(node, Node::Directory(_)))
    {
        let node_size = node.size(&node_table);
        let potential_new_size = total_space_used - node_size;
        if potential_new_size <= MAXIMUM_SPACE_FOR_USE
            && potential_new_size > max_potential_new_size
        {
            max_potential_new_size = potential_new_size;
            dir_size = node_size
        }
    }

    Ok(dir_size)
}

#[cfg(test)]
mod tests {
    use crate::{calculate_dir_sums_under_limit, find_smallest_viable_dir_size};

    const BASIC_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    const INPUTS: [&str; 2] = [BASIC_INPUT, include_str!("../input")];

    const RESULTS: [u64; 2] = [95437, 1642503];

    #[test]
    fn should_take_input_and_find_result() -> anyhow::Result<()> {
        let mut i = 0;
        for input in INPUTS {
            let total = calculate_dir_sums_under_limit(input)?;

            assert_eq!(total, RESULTS[i]);

            i += 1;
        }

        Ok(())
    }

    const RESULTS_PART2: [u64; 2] = [24933642, 6999588];

    #[test]
    fn part2_should_take_input_and_find_result() -> anyhow::Result<()> {
        let mut i = 0;
        for input in INPUTS {
            let size = find_smallest_viable_dir_size(input)?;

            assert_eq!(size, RESULTS_PART2[i]);

            i += 1;
        }

        Ok(())
    }
}
