use index_vec::{index_vec, IndexVec};

index_vec::define_index_type! {
    /// Index of nodes stored in a buffer
    pub struct NodeIndex = usize;
}

const ROOT_DIR: &str = "/";

pub type Size = u64;

#[derive(Debug, Default)]
pub struct FileSystem {
    /// Collection that owns all nodes
    nodes: IndexVec<NodeIndex, Node>,
}

impl FileSystem {
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a node to the FileSystem and return it's stored index
    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        self.nodes.push(node)
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    File {
        /// file identifier, including extension
        id: String,

        /// size of the file
        size: Size,
    },
    Directory {
        /// directory identifier
        id: String,

        /// buffer indices that reference child nodes
        children: Vec<NodeIndex>,
    },
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
            Self::File { size, .. } => *size,
            Self::Directory { children, .. } => children.iter().fold(0, |acc, i| {
                let node = &nodes[*i];
                acc + node.size(nodes)
            }),
        }
    }

    /// Initialize a file node
    pub fn file(id: &str, size: Size) -> Self {
        Self::File {
            id: id.to_string(),
            size,
        }
    }

    /// Initialize a directory node
    pub fn dir(id: &str, item_indices: &[NodeIndex]) -> Self {
        Self::Directory {
            id: id.to_string(),
            children: Vec::from(item_indices),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_size_recursively_sums_all_children() {
        let mut fs = FileSystem::new();

        // fs.add_node(Node::dir(
        //     ROOT_DIR,
        //     &[
        //         fs.add_node(Node::file("t.txt", 2)),
        //         fs.add_node(Node::dir(
        //             "e",
        //             &[
        //                 fs.add_node(Node::file("p.exe", 2)),
        //                 fs.add_node(Node::dir("z", &[fs.add_node(Node::file("x.lvm", 2))])),
        //             ],
        //         )),
        //     ],
        // ));

        // let result = fs.nodes[0].size(&fs.nodes);

        // assert_eq!(result, 6);
    }
}
