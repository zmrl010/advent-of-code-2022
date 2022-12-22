use std::{
    cmp::Ordering,
    error::Error,
    fmt::{self, Display, Formatter},
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ParseError {
    Packet(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Packet(string) => {
                write!(f, "ParseError(Packet): Error parsing packet:\n\n`{string}`")
            }
        }
    }
}

impl Error for ParseError {}

impl From<serde_json::Error> for ParseError {
    fn from(s: serde_json::Error) -> Self {
        Self::Packet(s.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (List(a), List(b)) => a.cmp(b),
            (List(a), Integer(b)) => {
                let b_list = vec![Integer(b.clone())];
                a.cmp(&b_list)
            }
            (Integer(a), List(b)) => {
                let a_list = vec![Integer(a.clone())];
                a_list.cmp(&b)
            }
            (Integer(a), Integer(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type PacketPair = (Packet, Packet);

fn parse_input(input: &str) -> Result<Vec<PacketPair>, ParseError> {
    input
        .trim()
        .split("\n\n")
        .map(|line| {
            let raw_packets = line
                .split_once("\n")
                .ok_or_else(|| ParseError::Packet(format!("unable to split {line} by \\n")))?;
            Ok((
                serde_json::from_str(raw_packets.0)?,
                serde_json::from_str(raw_packets.1)?,
            ))
        })
        .collect()
}

pub fn part1(input: &str) -> Result<usize, ParseError> {
    let packet_pairs = parse_input(input)?;

    let sum = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            if pair.0 < pair.1 {
                // first index is 1; so add one to zero-based index
                return Some(index + 1);
            }

            None
        })
        .sum();

    Ok(sum)
}

pub fn part2(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .trim()
        .lines()
        .filter_map(|line| {
            let line = line.trim();

            if line.is_empty() {
                return None;
            }

            serde_json::from_str(line).ok()
        })
        .collect();

    let divider_packets = (
        Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]),
    );

    packets.push(divider_packets.0.clone());
    packets.push(divider_packets.1.clone());

    packets.sort();

    let decoder = packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| -> Option<usize> {
            if *packet == divider_packets.0 || *packet == divider_packets.1 {
                return Some(i + 1);
            }

            None
        });

    decoder.product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = include_str!("../basic_input");
    const INPUT: &str = include_str!("../input");

    #[test]
    fn part1_basic_input_eq_13() -> Result<(), ParseError> {
        let result = part1(BASIC_INPUT)?;

        assert_eq!(result, 13);

        Ok(())
    }

    #[test]
    fn part1_basic_input_eq_value() -> Result<(), ParseError> {
        let result = part1(INPUT)?;

        assert_eq!(result, 5905);

        Ok(())
    }

    #[test]
    fn part2_basic_input_eq_140() {
        let result = part2(BASIC_INPUT);

        assert_eq!(result, 140)
    }

    #[test]
    fn part2_basic_input_eq_value() {
        let result = part2(INPUT);

        assert_eq!(result, 21691);
    }
}
