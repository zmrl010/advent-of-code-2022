use std::{collections::HashSet, ops::Range};

const WINDOW_WIDTH: u8 = 4;

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
pub fn find_sop_marker(input: &str) -> usize {
    let mut window = Window::new(WINDOW_WIDTH);

    while window.rear() < input.len() {
        if let Some(value) = input.get(window.as_range()) {
            let set: HashSet<char> = HashSet::from_iter(value.chars());
            // if set and length are the same, we have no duplicates!
            if set.len() == window.len() {
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

    const INPUT_RESULTS: [(&str, usize); 6] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        (include_str!("../input"), 1804),
    ];
    #[test]
    fn find_sop_marker_should_take_input_and_find_result() {
        for (input, expected_result) in INPUT_RESULTS {
            let result = find_sop_marker(input);
            assert_eq!(result, expected_result)
        }
    }
}
