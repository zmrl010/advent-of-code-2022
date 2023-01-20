use pathfinding::prelude::Grid;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

/// Calls default sort function and returns the modified array
fn sorted<T: Ord, const N: usize>(mut arr: [T; N]) -> [T; N] {
    arr.sort();
    arr
}

pub fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .flat_map(|line| {
            let mut path = Vec::new();

            let mut points = line.trim().split(" -> ").map(|raw_point| {
                let (x, y) = raw_point
                    .split_once(",")
                    .expect("raw point should contain `,`");

                let x = x.parse().expect("left side should be a valid integer");
                let y = y.parse().expect("right side should be a valid integer");

                Point::from((x, y))
            });

            if let Some(mut prev) = points.next() {
                while let Some(current) = points.next() {
                    if current.x == prev.x {
                        let [min, max] = sorted([prev.y, current.y]);
                        path.extend((min..=max).map(|y| (current.x, y)));
                    } else {
                        let [min, max] = sorted([prev.x, current.x]);
                        path.extend((min..=max).map(|x| (x, current.y)));
                    }
                    prev = current;
                }
            }

            path
        })
        .collect()
}

const START: Point = Point { x: 500, y: 0 };

pub fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut count = 0usize;
    let mut fallen_through = false;

    while !fallen_through {
        let mut current = START;
        let mut at_rest = false;

        while !fallen_through && !at_rest {
            let y = current.y + 1;
            if y > grid.height {
                fallen_through = true;
            }

            if !grid.has_vertex((current.x, y)) {
                current.y = y;
            } else if !grid.has_vertex((current.x - 1, y)) {
                current.x -= 1;
                current.y = y;
            } else if !grid.has_vertex((current.x + 1, y)) {
                current.x += 1;
                current.y = y;
            } else {
                at_rest = true;
            }
        }

        if at_rest {
            let newly_added = grid.add_vertex((current.x, current.y));

            if !newly_added {
                panic!("Failed attempt to add a Grid vertex that already existed.")
            }

            count += 1;
        }
    }

    count
}

/// Include a floor that spans across the entire Grid width
///
/// * Add 2 rows to the grid
/// * Insert vertices across the entire width of the last row
/// * Double width
fn with_floor(mut grid: Grid) -> Grid {
    let width = grid.width * 2;
    grid.resize(width, grid.height + 2);

    let y = grid.height - 1;
    for x in 0..width {
        grid.add_vertex((x, y));
    }

    grid
}

pub fn part2(input: &str) -> usize {
    let mut grid = with_floor(parse_input(input));
    let mut count = 0usize;

    'outer: loop {
        let mut current = START;

        loop {
            if !grid.has_vertex((current.x, current.y + 1)) {
                current = (current.x, current.y + 1).into();
                continue;
            }

            if !grid.has_vertex((current.x - 1, current.y + 1)) {
                current = (current.x - 1, current.y + 1).into();
                continue;
            }

            if !grid.has_vertex((current.x + 1, current.y + 1)) {
                current = (current.x + 1, current.y + 1).into();
                continue;
            }

            grid.add_vertex((current.x, current.y));
            count += 1;
            break;
        }

        if current == START {
            break 'outer;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_result_eq_expected() {
        let result = part1(BASIC_INPUT);

        assert_eq!(result, 24);
    }

    #[test]
    fn part1_input_result_eq_expected() {
        let result = part1(INPUT);

        assert_eq!(result, 817);
    }

    #[test]
    fn part2_basic_input_result_eq_expected() {
        let result = part2(BASIC_INPUT);

        assert_eq!(result, 93)
    }

    #[test]
    fn part2_input_result_eq_expected() {
        let result = part2(INPUT);

        assert_eq!(result, 23416);
    }
}
