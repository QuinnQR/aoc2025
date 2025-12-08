use std::{cmp::Reverse, collections::BinaryHeap, num::ParseIntError};

fn main() {
    let points = match parse_input("input") {
        Err(error) => {
            println!("Error occured reading day 8 input: {}", error.to_string());
            return;
        }
        Ok(input_data) => input_data,
    };
    assert_eq!(points.len(), 1000);
    let (part1, part2) = calculate_answers(points, 1000);
    println!("\tDay 8\nPart 1: {}\nPart 2: {}", part1, part2);
}
const NO_COMPONENT_IDX: usize = usize::MAX;
fn calculate_answers(points: Vec<Point>, num_edges_to_wire: usize) -> (i64, i64) {
    let mut edge_heap = get_edges(&points);
    let mut components: Vec<Option<Component>> = Vec::new();
    components.reserve(points.len());
    let mut node_to_component_idx: Vec<usize> = Vec::new();
    node_to_component_idx.resize(points.len(), NO_COMPONENT_IDX);

    // Wire components for part 1 (returns early, when num_edges_to_wire has been wired)
    add_edges_to_graph(
        &mut edge_heap,
        &mut components,
        &mut node_to_component_idx,
        points.len(),
        num_edges_to_wire,
    );
    let part1 = get_part_one(&components);

    // Wire components for part 2 (returns when the whole graph is a component, return indices of last 2 points wired)
    let (last_node1, last_node2) = add_edges_to_graph(
        &mut edge_heap,
        &mut components,
        &mut node_to_component_idx,
        points.len(),
        usize::MAX,
    );
    let part2 = points[last_node1].0.0 * points[last_node2].0.0;
    (part1 as i64, part2 as i64)
}
fn get_part_one(components: &Vec<Option<Component>>) -> i64 {
    let mut part1_component_sizes = components
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.as_ref().unwrap().nodes.len())
        .collect::<Vec<_>>();
    part1_component_sizes.sort();
    part1_component_sizes.into_iter().rev().take(3).fold(1, |x, y| x * y) as i64
}
fn add_edges_to_graph<'a>(
    edge_heap: &mut std::collections::BinaryHeap<GraphEdge>,
    components: &mut Vec<Option<Component>>,
    node_to_component_idx: &mut Vec<usize>,
    num_points: usize,
    max_edges: usize,
) -> (usize, usize) {
    // Assumes that part1 finishes before part2 (technically not guaranteed)
    // The return value are the ids of the final nodes connected, used for part2
    // Once a component has a size of num_points, the function returns as part 2 is finished.
    let mut edge = edge_heap.pop();
    let mut remaining_edges = max_edges;
    while edge.is_some() {
        let (source_node, dest_node) = edge.unwrap().get_nodes();
        match (node_to_component_idx[source_node], node_to_component_idx[dest_node]) {
            (NO_COMPONENT_IDX, NO_COMPONENT_IDX) => {
                // Neither node is currently in a component, so make a new one.
                components.push(Some(Component {
                    nodes: vec![source_node, dest_node],
                }));
                node_to_component_idx[source_node] = components.len() - 1;
                node_to_component_idx[dest_node] = components.len() - 1;
            }
            (NO_COMPONENT_IDX, component_id) => {
                // One node is in a component, so add the other node to the component as well
                components[component_id].as_mut().unwrap().nodes.push(source_node);
                node_to_component_idx[source_node] = component_id;
                if components[component_id].as_ref().unwrap().nodes.len() == num_points {
                    return (source_node, dest_node);
                }
            }
            (component_id, NO_COMPONENT_IDX) => {
                // Mirror of above case, same idea
                components[component_id].as_mut().unwrap().nodes.push(dest_node);
                node_to_component_idx[dest_node] = component_id;
                if components[component_id].as_ref().unwrap().nodes.len() == num_points {
                    return (source_node, dest_node);
                }
            }
            (source_id, dest_id) => {
                // Both nodes are in a component, so merge them and deactivate one (in this case,
                // deactivate the component that source_id belongs to)
                if source_id != dest_id {
                    let old_source = components[source_id].take().unwrap();
                    for node in old_source.nodes.iter() {
                        node_to_component_idx[*node] = dest_id;
                    }
                    components[dest_id]
                        .as_mut()
                        .unwrap()
                        .nodes
                        .extend(old_source.nodes.into_iter());
                    if components[dest_id].as_ref().unwrap().nodes.len() == num_points {
                        return (source_node, dest_node);
                    }
                }
            }
        }
        remaining_edges -= 1;
        if remaining_edges == 0 {
            // Used for part 1 to end the iteration early.
            // as part 1 doesn't use the RV, can return anything here.
            return (0, 0)
        };
        edge = edge_heap.pop();
    }
    // Shouldn't reach here as either part 1 should finish (remaining edges == 0) or
    // part 2 should finish (largest component is of size num_points)
    (0, 0)
}
fn get_edges(points: &Vec<Point>) -> BinaryHeap<GraphEdge> {
    let mut edges: Vec<GraphEdge> = Vec::new();
    edges.reserve(points.len() * (points.len() - 1) / 2);
    for (p1_id, point1) in points.iter().enumerate() {
        for (p2_id, point2) in points.iter().enumerate() {
            if p1_id == p2_id {
                break;
            }
            edges.push(GraphEdge::new((*point2 - *point1).norm2_sq(), p1_id, p2_id));
        }
    }
    // This should be O(n^2) (instead of O(n^2 log n^2) for a sort) (where n is the number of nodes)
    BinaryHeap::from(edges)
}
fn parse_input<P>(filename: P) -> Result<Vec<Point>, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_to_string(filename)?
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Point>, Box<dyn std::error::Error>>>()
}
fn parse_line(point_line: &str) -> Result<Point, Box<dyn std::error::Error>> {
    let point_vec = point_line
        .split(',')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    if point_vec.len() != 3 {
        return Err("Incorrect point format in input".into())
    }
    Ok(Point((point_vec[0], point_vec[1], point_vec[2])))
}
#[derive(PartialEq, Eq, Clone, Copy)]
struct Point((i64, i64, i64));
impl Point {
    fn norm2_sq(&self) -> i64 { self.0.0 * self.0.0 + self.0.1 * self.0.1 + self.0.2 * self.0.2 }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self((self.0.0 - other.0.0, self.0.1 - other.0.1, self.0.2 - other.0.2)) }
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct GraphEdge {
    distance: i64,
    source: usize,
    destination: usize,
}
impl GraphEdge {
    fn get_nodes(&self) -> (usize, usize) { (self.source, self.destination) }
    fn new(edge_distance: i64, edge_source: usize, edge_dest: usize) -> Self {
        Self {
            distance: edge_distance,
            source: edge_source,
            destination: edge_dest,
        }
    }
}
impl PartialOrd for GraphEdge {
    // Technically I think this isn't valid as two non equal GraphEdges could return equal, but it compiles and works for this problem
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(Reverse(self.distance).cmp(&Reverse(other.distance)))
    }
}
impl Ord for GraphEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { Reverse(self.distance).cmp(&Reverse(other.distance)) }
}

struct Component {
    nodes: Vec<usize>,
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_file_parse() {
        let points = match parse_input("test") {
            Err(error) => {
                println!("Error parsing test input: {}", error.to_string());
                panic!();
            }
            Ok(value) => value,
        };
        assert_eq!(points.len(), 20);
        assert_eq!(points[0].0, (162, 817, 812));
        assert_eq!(points[9].0, (52, 470, 668));
        assert_eq!(points[19].0, (425, 690, 689));
    }
    #[test]
    fn test_edges() {
        let mut edges = get_edges(&parse_input("test").expect("Test data should be stored at \"test\""));
        let (node1, node2) = edges.pop().unwrap().get_nodes();
        let (node3, node4) = edges.pop().unwrap().get_nodes();
        assert_eq!(node1, 19);
        assert_eq!(node2, 0);
        assert_eq!(node3, 7);
        assert_eq!(node4, 0);
    }
    #[test]
    fn test_edge_type() {
        let source = 1;
        let dest = 2;
        let dist = 5;
        let edge = GraphEdge::new(dist, source, dest);
        let (new_source, new_dest) = edge.get_nodes();
        assert_eq!(new_source, source);
        assert_eq!(new_dest, dest);
    }
    #[test]
    fn test_part_one() {
        let (part1, _part2) =
            calculate_answers(parse_input("test").expect("Test data should be stored at \"test\""), 10);
        assert_eq!(part1, 40);
    }
    #[test]
    fn test_part_two() {
        let (_part1, part2) =
            calculate_answers(parse_input("test").expect("Test data should be stored at \"test\""), 10);
        assert_eq!(part2, 25272);
    }
}
