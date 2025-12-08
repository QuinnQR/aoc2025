use std::num::ParseIntError;

fn main() {
    let points = match parse_input("input") {
        Err(error) => {
            println!("Error occured reading day 6 input: {}", error.to_string());
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
    let edges = get_edges(&points);
    let mut components: Vec<Option<Component>> = Vec::new();
    let mut node_to_component_idx: Vec<usize> = Vec::new();
    node_to_component_idx.resize(points.len(), NO_COMPONENT_IDX);
    add_edges(
        Box::new(edges.iter().take(num_edges_to_wire)),
        &mut components,
        &mut node_to_component_idx,
        points.len(),
    );
    let mut part1_vec = components
        .iter()
        .filter(|x| x.is_some())
        .map(|x| x.as_ref().unwrap().nodes.len())
        .collect::<Vec<_>>();
    part1_vec.sort();
    let part1 = part1_vec.into_iter().rev().take(3).fold(1, |x, y| x * y);
    let (node1_idx, node2_idx) = add_edges(
        Box::new(edges.iter().skip(num_edges_to_wire)),
        &mut components,
        &mut node_to_component_idx,
        points.len(),
    );
    let part2 = points[node1_idx].0.0 * points[node2_idx].0.0;
    (part1 as i64, part2 as i64)
}
fn add_edges<'a>(
    edge_iter: Box<dyn Iterator<Item = &GraphEdge> + 'a>,
    components: &mut Vec<Option<Component>>,
    node_to_component_idx: &mut Vec<usize>,
    num_points: usize,
) -> (usize, usize) {
    for edge in edge_iter {
        let (source_node, dest_node) = edge.get_nodes();
        match (node_to_component_idx[source_node], node_to_component_idx[dest_node]) {
            (NO_COMPONENT_IDX, NO_COMPONENT_IDX) => {
                components.push(Some(Component {
                    nodes: vec![source_node, dest_node],
                }));
                node_to_component_idx[source_node] = components.len() - 1;
                node_to_component_idx[dest_node] = components.len() - 1;
            }
            (NO_COMPONENT_IDX, component_id) => {
                components[component_id].as_mut().unwrap().nodes.push(source_node);
                node_to_component_idx[source_node] = component_id;
                if components[component_id].as_ref().unwrap().nodes.len() == num_points {
                    return (source_node, dest_node);
                }
            }
            (component_id, NO_COMPONENT_IDX) => {
                components[component_id].as_mut().unwrap().nodes.push(dest_node);
                node_to_component_idx[dest_node] = component_id;
                if components[component_id].as_ref().unwrap().nodes.len() == num_points {
                    return (source_node, dest_node);
                }
            }
            (source_id, dest_id) => {
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
    }
    (0, 0)
}
fn get_edges(points: &Vec<Point>) -> Vec<GraphEdge> {
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
    edges.sort();
    edges
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
        .map(str::parse::<i128>)
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    if point_vec.len() != 3 {
        return Err("Incorrect point format in input".into())
    }
    return Ok(Point((point_vec[0], point_vec[1], point_vec[2])));
}
#[derive(PartialEq, Eq, Clone, Copy)]
struct Point((i128, i128, i128));
impl Point {
    fn norm2_sq(&self) -> i128 { self.0.0 * self.0.0 + self.0.1 * self.0.1 + self.0.2 * self.0.2 }
}
impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self { Self((self.0.0 - other.0.0, self.0.1 - other.0.1, self.0.2 - other.0.2)) }
}
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct GraphEdge {
    distance: i128,
    source: usize,
    dest: usize,
}
impl GraphEdge {
    fn get_nodes(&self) -> (usize, usize) { (self.source, self.dest) }
    fn new(dist: i128, src: usize, des: usize) -> Self {
        Self {
            distance: dist,
            source: src,
            dest: des,
        }
    }
}
/*struct GraphEdge(i128);
impl GraphEdge {
    fn new(distance: i128, source: usize, dest: usize) -> Self {
        GraphEdge((distance << 32) + ((source & 0xFF) << 16) as i128 + (dest & 0xFF) as i128)
    }
    fn get_nodes(&self) -> (usize, usize) {
        (
            ((self.0 >> 16) & 0xFF).try_into().unwrap_or(usize::MAX),
            (self.0 & 0xFF).try_into().unwrap_or(usize::MAX),
        )
    }
}
    */
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
        edges.sort();
        let (node1, node2) = edges[0].get_nodes();
        let (node3, node4) = edges[1].get_nodes();
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
