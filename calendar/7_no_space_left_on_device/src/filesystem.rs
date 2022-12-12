use std::{
    ops::{Index, IndexMut},
    slice::Iter,
};

use index_vec::IndexVec;

index_vec::define_index_type! {
    /// Index of nodes stored in a buffer
    pub struct NodeIndex = usize;
}

pub type Size = u64;

/// List that stores nodes to be referenced by their index
#[derive(Debug, Default)]
pub struct NodeTable {
    /// Collection that owns all nodes
    nodes: IndexVec<NodeIndex, Node>,
}

impl NodeTable {
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a node to the FileSystem and return it's stored index
    pub fn add(&mut self, node: Node) -> NodeIndex {
        self.nodes.push(node)
    }

    /// Create a file node and attach it to the nodes collection, returning its index
    pub fn add_file(&mut self, name: &str, size: Size) -> NodeIndex {
        let node = Node::file(name, size);
        self.add(node)
    }

    /// Create a directory node and attach it to the nodes collection, returning its index
    pub fn add_dir(&mut self, name: &str) -> NodeIndex {
        let node = Node::dir(name);
        self.add(node)
    }

    /// Get a node at index
    pub fn get(&self, index: NodeIndex) -> Option<&Node> {
        self.nodes.get(index)
    }
}

impl Index<NodeIndex> for NodeTable {
    type Output = Node;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<NodeIndex> for NodeTable {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

#[derive(Debug, Clone, Default)]
pub struct File {
    /// file name, including extension
    pub name: String,

    /// size of the file
    size: Size,

    /// option containing the index of the file's parent node if there is one
    pub parent: Option<NodeIndex>,
}

impl File {
    pub fn new(name: &str, size: Size) -> Self {
        Self {
            name: name.to_string(),
            size,
            parent: None,
        }
    }

    /// get the current size of the file
    fn size(&self) -> Size {
        self.size
    }
}

#[derive(Debug, Clone, Default)]
pub struct Directory {
    /// directory name
    pub name: String,

    /// collection of indices that reference child nodes
    pub children: Vec<NodeIndex>,

    /// option containing the index of the directory's parent node if there is one
    pub parent: Option<NodeIndex>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Add a node index to the directory
    pub fn push(&mut self, node_index: NodeIndex) {
        self.children.push(node_index)
    }

    /// Returns an Iterator over child node indices
    pub fn iter(&self) -> Iter<'_, NodeIndex> {
        self.children.iter()
    }

    /// Recursively sum the size of all sub directories and files
    fn size(&self, nodes: &IndexVec<NodeIndex, Node>) -> Size {
        self.children.iter().fold(0, |acc, i| {
            let node = &nodes[*i];
            acc + node.size(nodes)
        })
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    File(File),
    Directory(Directory),
}

// pub fn walk_nodes(root: Node) -> impl Iterator<Item = Node> {
//     match root {
//         Node::File { .. } => [root].iter(),
//         Node::Directory { items, .. } => items.iter().map(|node| walk_nodes(*node)).flatten(),
//     }
// }

impl Node {
    /// Calculate the total size of the node and any
    /// subnodes if the node is a directory
    pub fn size(&self, nodes: &IndexVec<NodeIndex, Node>) -> Size {
        match self {
            Self::File(file) => file.size(),
            Self::Directory(dir) => dir.children.iter().fold(0, |acc, i| {
                let node = &nodes[*i];
                acc + node.size(nodes)
            }),
        }
    }

    /// return the nodes name
    pub fn name(&self) -> String {
        match self {
            Node::File(file) => file.name.clone(),
            Node::Directory(dir) => dir.name.clone(),
        }
    }

    /// Initialize a file node
    pub fn file(name: &str, size: Size) -> Self {
        Self::File(File::new(name, size))
    }

    /// Initialize a directory node
    pub fn dir(name: &str) -> Self {
        Self::Directory(Directory::new(name))
    }

    /// Get the node's parent index if there is one
    pub fn parent(&self) -> Option<NodeIndex> {
        match self {
            Node::File(f) => f.parent,
            Node::Directory(d) => d.parent,
        }
    }

    /// Set the nodes parent index
    pub fn set_parent(&mut self, index: NodeIndex) {
        match self {
            Node::File(f) => f.parent = Some(index),
            Node::Directory(d) => d.parent = Some(index),
        }
    }

    /// Get the nodes parent directory by cross referencing a node vector with
    pub fn parent_dir(&self, nodes: &IndexVec<NodeIndex, Node>) -> Option<Directory> {
        let parent_index = self.parent()?;

        let parent_node = nodes.get(parent_index)?;

        match parent_node {
            Node::File(_) => unreachable!("parent should not be a file"),
            Node::Directory(dir) => Some(dir.clone()),
        }
    }
}
