mod filesystem;

use filesystem::FileSystem;
// fn sum_dirs() {}

enum Command {
    CD(String),
    LS,
}

fn parse_input(input: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let values = input.lines().map(|line| match line {
        val if val.starts_with("$") => {
            let mut args = val.split(' ');
            let command = args.nth(1).expect("expected command to be 2rd element");
            let command_args = args
                .nth(0)
                .expect("expected command args to be 3rd element");
        }
        val if val.starts_with("dir") => {
            let args = val.split(' ');
        }
        _ => unreachable!(),
    });

    fs
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
