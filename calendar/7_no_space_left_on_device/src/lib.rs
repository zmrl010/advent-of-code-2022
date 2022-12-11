mod filesystem;

use filesystem::{Directory, Node, NodeTable};
// fn sum_dirs() {}

fn parse_input(input: &str) -> anyhow::Result<NodeTable> {
    let mut nodes = NodeTable::new();

    let mut current_dir_index = nodes.add(Node::Directory(Directory::new("/")));

    /*
        stack that stores the node indices of directories we travel
        so we can move forward and back with .push() and .pop()
    */
    let mut cursor = vec![current_dir_index];

    for line in input.trim().lines() {
        let args: Vec<&str> = line.split(' ').collect();
        match args[0] {
            "$" => match args[1] {
                "cd" => {
                    let target_dir = args[2];
                    if target_dir == ".." {
                        if let Some(node_index) = cursor.pop() {
                            current_dir_index = node_index
                        }
                    } else {
                        cursor.push(current_dir_index);
                        let node = &nodes[current_dir_index];
                        //
                        if node.name() == target_dir {}

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
                let dir = Directory::new(args[1]);
                nodes.add(Node::Directory(dir));
            }
            val if val.chars().all(char::is_numeric) => {
                // add file node
            }
            _ => continue,
        }
    }

    Ok(nodes)
}

fn inspect_file_system_dirs(input: &str) {}

#[cfg(test)]
mod tests {

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

    const RESULTS: [u32; 2] = [95437, 95437];

    #[test]
    fn should_take_input_and_find_result() {}

    #[test]
    fn part2_should_take_input_and_find_result() {}
}
