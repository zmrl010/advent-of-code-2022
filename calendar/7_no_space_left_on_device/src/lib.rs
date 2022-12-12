mod filesystem;

use filesystem::{Node, NodeTable, NodeIndex};

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

                    if target_dir == ".." {

                        if let Some(node_index) = node.parent() {

                            current_dir_index = node_index

                        }

                    } else {

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
            val if val.chars().all(char::is_numeric) => {
                let size: u64 = val.parse()?;

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

fn part1(input: &str) -> anyhow::Result<u64> {
    let node_table = parse_input(input)?;

    let mut total: u64 = 0;

    let root_node = &node_table[NodeIndex::new(0)];

    if let Node::Directory(dir) = root_node {
        for node_index in dir.iter() {
            let node = &node_table[*node_index];

            let size = node.size(&node_table);

            if size < 100_000 {
                total += size
            }
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests { 
    use crate::part1;

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

    const RESULTS: [u64; 2] = [95437, 190290];

    #[test]
    fn should_take_input_and_find_result() -> anyhow::Result<()> {
        let mut i = 0;
        for input in INPUTS {
            let total = part1(input)?;
            
            assert_eq!(total, RESULTS[i]);

            i += 1;
        }

        assert!(true);

        Ok(())
    }

    #[test]
    fn part2_should_take_input_and_find_result() {}
}
