/// 2021 AoC Day 12
///
/// Not going to lie, I am both proud and surprised I was able to code this path enumeration without looking up any
/// undergraduate graph theory.
///
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

fn can_visit_part_1(graph: &Graph, node: &Node, cur_path: &Vec<String>) -> bool {
    node.large || !cur_path.contains(&node.name)
}

/// Applies the rules from Part 2 to determine if we can visit 'node' given the current path 'cur_path'.
///
/// This function is very slow because of the hashing. We can probably just store a graph node list as a Vec and use
/// a Vec to count stuff and it will be an order of magnitude faster.
fn can_visit_part_2(graph: &Graph, node: &Node, cur_path: &Vec<String>) -> bool {
    if node.large {
        return true
    }
    if node.name == START_NODE {
        return false
    }

    // allow a small node to be visited once **or twice**
    let mut stats: HashMap<&String, usize> = HashMap::new();
    for el in cur_path {
        if ! graph.nodes[el].large {
            *stats.entry(el).or_insert(0usize) += 1;
        }
    }
    *stats.entry(&node.name).or_insert(0usize) += 1;
    if stats[&node.name] > 2 {
        return false;
    }

    let mut double_visits = 0usize;
    for (nn, count) in &stats {
        if count > &1usize {
            double_visits += 1usize;
        }
    }

    // If there are 0 or 1 double visits in the path including this node, we are good.
    double_visits <= 1
}

fn get_paths_from_base(graph: &Graph, cur_path: &Vec<String>) -> Vec<Vec<String>> {
    let latest = &cur_path[cur_path.len() - 1];
    let mut paths_reached_end = Vec::new();

    let part_1 = true;
    let can_visit = if part_1 { can_visit_part_1 } else { can_visit_part_2 };

    for neigh in &graph.nodes[latest].neighbors {
        if neigh == END_NODE {
            // Found a new way to the exit
            let mut new_path = cur_path.to_vec();
            new_path.push(neigh.to_string());
            paths_reached_end.push(new_path);
        }
        else if can_visit(&graph, &graph.nodes[neigh], cur_path) {
            // todo add recursive call paths to paths_reached_end
            let mut new_path = cur_path.to_vec();
            new_path.push(neigh.to_string());

            let mut rec_paths = get_paths_from_base(graph, &new_path);
            paths_reached_end.append(&mut rec_paths);
        }
        else {
            // nothing to do - can't visit neighbor, and neighbor is not an end either
        }
    }

    paths_reached_end
}

fn get_paths(graph: &Graph) -> Vec<Vec<String>> {
    get_paths_from_base(graph, &vec![(&START_NODE).to_string()])
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
    // let data = fs::read_to_string("input/12-demo-01.txt").expect("Unable to read file.");
    // let data = fs::read_to_string("input/12-demo-02.txt").expect("Unable to read file.");
    // v0, Release mode:         70ms
    // v0, Debug mode:          700ms
    //
    // let data = fs::read_to_string("input/12-demo-03.txt").expect("Unable to read file.");
    //
    // v0.0, Release mode:     1760ms
    let data = fs::read_to_string("input/12.txt").expect("Unable to read file.");

    let caves = graph_from_data(&data);
    println!("Graph: {:?}", caves);
    let paths = get_paths(&caves);
    println!("Part 1 paths:");
    // for path in &paths {
    //     println!("\t- {:?}", path);
    // }
    println!("Part 1 result: {}", paths.len());
}

fn main() {
    day_12_passage();
}
