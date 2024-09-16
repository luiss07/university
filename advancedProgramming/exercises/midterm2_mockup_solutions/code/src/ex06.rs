// Write the necessary structs to represent an oriented graph generic over `T`, where `T`
// implements `Hash`, `PartialEq` and `Eq`.
// - `Node`, with a value of type `T` and a vector of adjacent nodes
// - `Graph`, with a vector of nodes
//
// Then, implement the following methods for `Node`:
// - `new`, which creates a new `Node` with the given value and the given vector of adjacents
// - `get_value`, which returns a reference to the value of the node
//
// Implement `Debug` for `Node`, so that it prints the value of the node and the values of its
// adjacents.
// For example, if the node has value `1` and its adjacents are `2` and `3`, it should print:
// >[value: 1, adjacents: [2, 3]]
//
// Then, implement the following methods for `Graph`:
// - `new`, which creates a `Graph` from a vector of nodes, with the respective adjacents set
// - `dfs`, which performs a depth-first search on the graph, starting from the given node. It
// returns a vector of nodes, in the order in which they were visited.
//

use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

type NodeRef<T> = Rc<Node<T>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node<T: Hash + PartialEq + Eq> {
    value: T,
    adjacents: Vec<NodeRef<T>>,
}

struct Graph<T: Hash + PartialEq + Eq> {
    nodes: Vec<NodeRef<T>>,
}

impl<T: Hash + PartialEq + Eq> Node<T> {
    pub fn new(value: T, adjacents: Vec<NodeRef<T>>) -> Self {
        Self { value, adjacents }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

impl<T> Debug for Node<T>
where
    T: Hash + PartialEq + Eq + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adjacents = self
            .adjacents
            .iter()
            .map(|x| format!("{:?}", x.get_value()))
            .collect::<Vec<_>>();
        let adj_str = format!("[{}]", adjacents.join(", "));

        write!(f, "[value: {:?}, adjacents: {:?}]", self.value, adj_str)
    }
}

impl<T> Graph<T>
where
    T: Hash + PartialEq + Eq + Debug,
{
    pub fn new(nodes: Vec<NodeRef<T>>) -> Self {
        Self { nodes }
    }

    pub fn dfs(&self, root: NodeRef<T>) -> Vec<NodeRef<T>> {
        let mut visited = HashSet::<NodeRef<T>>::new();
        let mut history = Vec::<NodeRef<T>>::new();
        let mut queue = VecDeque::<NodeRef<T>>::new();
        queue.push_back(root);

        while let Some(current_node) = queue.pop_front() {
            if visited.insert(current_node.clone()) {
                history.push(current_node.clone());
                for neighbor in current_node.adjacents.iter().rev() {
                    queue.push_front(neighbor.clone());
                }
            }
        }

        history
    }
}

// ---

#[test]
fn dfs_test() {
    let n1 = Rc::new(Node::new(1, vec![]));
    let n2 = Rc::new(Node::new(2, vec![n1.clone()]));
    let n3 = Rc::new(Node::new(3, vec![]));
    let n4 = Rc::new(Node::new(4, vec![n1.clone(), n3.clone()]));
    let n5 = Rc::new(Node::new(5, vec![n2.clone(), n4.clone()]));
    let n6 = Rc::new(Node::new(6, vec![n5.clone(), n4.clone()]));
    let n7 = Rc::new(Node::new(7, vec![n2.clone(), n4.clone()]));

    let graph = Graph::new(vec![
        n1.clone(),
        n2.clone(),
        n3.clone(),
        n4.clone(),
        n5.clone(),
        n6.clone(),
        n7.clone(),
    ]);

    let mut paths: Vec<Vec<NodeRef<i32>>> = vec![];
    for n in graph.nodes.iter() {
        paths.push(graph.dfs(n.clone()))
    }

    paths.iter().for_each(|path| {
        println!("{:?}", path);
    });
}
/*
[[value: 1, adjacents: "[]"]]
[[value: 2, adjacents: "[1]"], [value: 1, adjacents: "[]"]]
[[value: 3, adjacents: "[]"]]
[[value: 4, adjacents: "[1, 3]"], [value: 1, adjacents: "[]"], [value: 3, adjacents: "[]"]]
[[value: 5, adjacents: "[2, 4]"], [value: 2, adjacents: "[1]"], [value: 1, adjacents: "[]"], [value: 4, adjacents: "[1, 3]"], [value: 3, adjacents: "[]"]]
[[value: 6, adjacents: "[5, 4]"], [value: 5, adjacents: "[2, 4]"], [value: 2, adjacents: "[1]"], [value: 1, adjacents: "[]"], [value: 4, adjacents: "[1, 3]"], [value: 3, adjacents: "[]"]]
[[value: 7, adjacents: "[2, 4]"], [value: 2, adjacents: "[1]"], [value: 1, adjacents: "[]"], [value: 4, adjacents: "[1, 3]"], [value: 3, adjacents: "[]"]]
*/
