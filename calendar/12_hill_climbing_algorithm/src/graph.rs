use std::{
    error,
    fmt::{self, Display, Formatter},
    slice::Iter,
    str::FromStr,
};

use pathfinding::prelude::dijkstra;

pub type Index = usize;

#[derive(Debug)]
pub enum ParseError {
    Graph(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Graph(value) => {
                writeln!(
                    f,
                    "ParseError(Graph): error parsing tile graph from `{value}`"
                )
            }
        }
    }
}

impl error::Error for ParseError {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tile {
    Start,
    End,
    Tower(u32),
}

impl Tile {
    /// Get the char value of the tile
    pub fn char(&self) -> char {
        match self {
            Tile::Start => 'a',
            Tile::End => 'z',
            Tile::Tower(value) => {
                char::from_u32(*value).expect("Tile value should have been validated here")
            }
        }
    }

    /// Get the Tile's elevation
    ///
    /// The elevation is found differently depending on the variant
    /// * [`Tile::Tower`] - this will be its .0 property, which
    /// is an integer converted from the raw char
    /// * [`Tile::Start`] - same elevation as 'a' character
    /// * [`Tile::End`] - same elevation as 'z' character
    fn elevation(&self) -> u32 {
        match self {
            Tile::Start => 'a' as u32,
            Tile::End => 'z' as u32,
            Tile::Tower(value) => *value,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => Tile::Tower(c as u32),
        }
    }
}

impl FromStr for Graph {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<Pos> = None;
        let mut end: Option<Pos> = None;

        let mut tiles = vec![];

        for (y, line) in s.trim().lines().enumerate() {
            tiles.push(vec![]);

            for (x, char) in line.trim().chars().enumerate() {
                let tile = Tile::from(char);

                match tile {
                    Tile::Start => start = Some(Pos(x, y)),
                    Tile::End => end = Some(Pos(x, y)),
                    Tile::Tower(_) => {}
                };

                tiles[y].push(tile);
            }
        }

        Ok(Self {
            start: start.ok_or_else(|| ParseError::Graph(s.to_string()))?,
            end: end.ok_or_else(|| ParseError::Graph(s.to_string()))?,
            tiles,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    start: Pos,
    end: Pos,
    tiles: Vec<Vec<Tile>>,
}

impl Graph {
    fn height(&self) -> usize {
        self.tiles.len()
    }

    fn width(&self) -> usize {
        self.tiles.get(0).map(|row| row.len()).unwrap_or(0)
    }

    fn get(&self, pos: &Pos) -> Option<&Tile> {
        self.tiles.get(pos.1)?.get(pos.0)
    }

    /// Return an iterator over each row of the graph
    pub fn iter(&self) -> Iter<'_, Vec<Tile>> {
        self.tiles.iter()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub(crate) Index, pub(crate) Index);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(x, y) = self;
        write!(f, "Pos({x}, {y})")
    }
}

impl From<(Index, Index)> for Pos {
    fn from((x, y): (Index, Index)) -> Self {
        Self(x, y)
    }
}

impl Pos {
    pub fn successors(&self, graph: &Graph) -> Vec<(Pos, u32)> {
        let &Self(x, y) = self;

        let mut positions = vec![];

        let validate_pos = |x: Index, y: Index| -> Option<Pos> {
            let successor_pos = Pos(x, y);

            let current_tile = graph.get(self).unwrap();
            let successor_tile = graph.get(&successor_pos).unwrap();

            if successor_tile.elevation() <= current_tile.elevation() + 1 {
                return Some(successor_pos);
            }

            None
        };

        let mut add_pos = |x: Index, y: Index| {
            if let Some(successor_pos) = validate_pos(x, y) {
                positions.push((successor_pos, 1));
            }
        };

        if x > 0 {
            add_pos(x - 1, y)
        }

        if y > 0 {
            add_pos(x, y - 1)
        }

        if y < graph.height() - 1 {
            add_pos(x, y + 1)
        }

        if x < graph.width() - 1 {
            add_pos(x + 1, y)
        }

        positions
    }
}

/// Find shortest path using custom starting position
pub fn find_shortest_path(start: &Pos, graph: Graph) -> Option<(Vec<Pos>, u32)> {
    dijkstra(start, |p| p.successors(&graph), |p| *p == graph.end)
}

/// Find shortest path using graph's `start`ing position
pub fn find_shortest_path_from_start(graph: Graph) -> Option<(Vec<Pos>, u32)> {
    find_shortest_path(&graph.start.clone(), graph)
}
