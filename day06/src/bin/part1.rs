use anyhow::Result;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

#[allow(dead_code)]
const INPUT_FILE: &str = "input.txt";
#[allow(dead_code)]
const EXAMPLE_FILE: &str = "example.txt";

/*
https://adventofcode.com/2019/day/06
*/

pub fn read_input(file_path: &str) -> Result<String> {
    let mut file = File::open(file_path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    Ok(contents)
}

#[derive(Debug, Clone)]
struct Node {
    value: String,
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<Node>>;

fn build_tree(orbits: &str) -> TreeNodeRef {
    let mut nodes: HashMap<String, TreeNodeRef> = HashMap::new();
    let mut parent_map: HashMap<String, String> = HashMap::new();
    let mut children_set: HashSet<String> = HashSet::new();

    // First pass: create all nodes and store them in the HashMap
    for line in orbits.lines() {
        let parts: Vec<&str> = line.split(')').collect();
        let parent = parts[0].to_string();
        let child = parts[1].to_string();

        parent_map.insert(child.clone(), parent.clone());
        children_set.insert(child.clone());

        nodes.entry(parent.clone()).or_insert_with(|| {
            Rc::new(RefCell::new(Node {
                value: parent.clone(),
                left: None,
                right: None,
            }))
        });
        nodes.entry(child.clone()).or_insert_with(|| {
            Rc::new(RefCell::new(Node {
                value: child.clone(),
                left: None,
                right: None,
            }))
        });
    }

    // Second pass: link child nodes to their respective parents
    for (child, parent) in &parent_map {
        if let Some(parent_node) = nodes.get(parent) {
            if let Some(child_node) = nodes.get(child) {
                if parent_node.borrow().left.is_none() {
                    parent_node.borrow_mut().left = Some(Rc::clone(child_node));
                } else {
                    parent_node.borrow_mut().right = Some(Rc::clone(child_node));
                }
            }
        }
    }

    // Find the root node (the node that is not a child of any other node)
    let root_value = nodes
        .keys()
        .find(|&k| !children_set.contains(k))
        .unwrap()
        .clone();
    nodes.remove(&root_value).unwrap()
}

fn calculate_distances(node: &TreeNodeRef, depth: usize, distances: &mut HashMap<String, usize>) {
    let node_borrow = node.borrow();
    distances.insert(node_borrow.value.clone(), depth);

    if let Some(ref left) = node_borrow.left {
        calculate_distances(left, depth + 1, distances);
    }
    if let Some(ref right) = node_borrow.right {
        calculate_distances(right, depth + 1, distances);
    }
}

fn main() {
    #[allow(unused_variables)]
    let contents: String = read_input(INPUT_FILE).unwrap_or_else(|err| panic!("{}", err));
    let tree = build_tree(&contents);
    let mut distances = HashMap::new();
    calculate_distances(&tree, 0, &mut distances);

    let total_distance: usize = distances.values().sum();
    println!("Total distance: {}", total_distance);
}
