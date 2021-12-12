use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;


const START_NODE: &str = "start";
const END_NODE: &str = "end";

#[derive(Debug)]
struct Node {
    large: bool,
    name: String,
    neighbors: HashSet<String>,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>,
}

fn get_paths(graph: &Graph) -> Vec<Vec<usize>> {
    Vec::new()
}

fn graph_from_data(data: &String) -> Graph {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for row in data.split("\n") {
        let start_end: Vec<String> = row.split("-").map(|x| x.to_string()).collect();
        let start_str = &start_end[0];
        let end_str = &start_end[1];

        for name in &start_end {
            if ! nodes.contains_key(name) {
                let is_large: bool = &name.to_ascii_uppercase() == name;
                let new_node = Node{large: is_large, name: name.to_string(), neighbors: HashSet::new()};
                nodes.insert(name.to_string(), new_node);
            }
        }

        // The key here is using 'get_mut' so we can actually modify the node that we are accessing.
        nodes.get_mut(end_str).unwrap().neighbors.insert(start_str.to_string());
        nodes.get_mut(start_str).unwrap().neighbors.insert(end_str.to_string());
    }

    if ! nodes.contains_key(START_NODE) {
        panic!("Graph must contain a start node!");
    }
    if ! nodes.contains_key(END_NODE) {
        panic!("Graph must contain an end node!");
    }

    Graph{nodes: nodes}

}


fn day_12_passage() {
    let data = fs::read_to_string("input/12-demo-01.txt").expect("Unable to read file.");
    // let data = fs::read_to_string("input/12.txt").expect("Unable to read file.");

    let caves = graph_from_data(&data);
    println!("Graph: {:?}", caves);
    let paths = get_paths(&caves);
    println!("Part 1 result: {}", paths.len());
}

fn main() {
    day_12_passage();
}
