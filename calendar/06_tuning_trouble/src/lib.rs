use std::{collections::HashSet, ops::Range};

/// Dynamic sliding window
pub struct Window {
    /// index of the first char in the window
    front_index: usize,
    /// width of the window - determine rear index with `front_index + width`
    width: u8,
}

impl Window {
    const fn new(width: u8) -> Self {
        Self {
            front_index: 0,
            width,
        }
    }

    /// get the front index
    fn front(&self) -> usize {
        self.front_index
    }

    /// get the rear index
    fn rear(&self) -> usize {
        self.front_index + self.len()
    }

    fn len(&self) -> usize {
        self.width as usize
    }

    /// get `front..rear` as a [`Range`]
    fn as_range(&self) -> Range<usize> {
        self.front()..self.rear()
    }

    /// Increment the index by one
    fn slide(&mut self) {
        self.front_index += 1
    }
}

/// Get the index of the start-of-packet marker
pub fn find_sop_marker(input: &str, window_width: u8) -> usize {
    let mut window = Window::new(window_width);

    while window.rear() < input.len() {
        if let Some(value) = input.get(window.as_range()) {
            let set: HashSet<char> = HashSet::from_iter(value.chars());
            // if set and length are the same, we have no duplicates!
            if set.len() == (window_width as usize) {
                return window.rear();
            }
        }

        window.slide()
    }

    input.len()
}

#[cfg(test)]
mod tests {
    use crate::find_sop_marker;

    const WINDOW_WIDTH: u8 = 4;
    const WINDOW_WIDTH_PART2: u8 = 14;

    const INPUTS: [&str; 6] = [
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        include_str!("../input"),
    ];

    const RESULTS: [usize; 6] = [7, 5, 6, 10, 11, 1804];

    const RESULTS_PART2: [usize; 6] = [19, 23, 23, 29, 26, 2508];

    #[test]
    fn find_sop_marker_should_take_input_and_find_result() {
        for (input, expected_result) in INPUTS.iter().zip(RESULTS.iter()) {
            let result = find_sop_marker(input, WINDOW_WIDTH);
            assert_eq!(result, *expected_result)
        }
    }

    #[test]
    fn part2_find_sop_marker_should_take_input_and_find_result() {
        for (input, expected_result) in INPUTS.iter().zip(RESULTS_PART2.iter()) {
            let result = find_sop_marker(input, WINDOW_WIDTH_PART2);
            assert_eq!(result, *expected_result)
        }
    }
}
