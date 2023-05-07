use petgraph::graph::Graph as PetGraph;
use petgraph::Directed;
use crate::merge_datasets::CombinedDataPoint;
use std::collections::HashMap;


pub type Graph = PetGraph<Vertex, f64, Directed>;

pub struct Vertex {
    pub date: String,
    pub inflation_rate: f64,
    pub interest_rate: f64,
}

fn calculate_distance(vertex_i: &Vertex, vertex_j: &Vertex) -> f64 { //function will be used for testing
    let x_diff = vertex_i.interest_rate - vertex_j.interest_rate;
    let y_diff = vertex_i.inflation_rate - vertex_j.inflation_rate;
    (x_diff * x_diff + y_diff * y_diff).sqrt()
}

pub fn construct_graph(combined_data: Vec<CombinedDataPoint>, threshold: f64) -> Graph {
    let mut graph = Graph::new();

    let mut node_indices = Vec::new();

    for data_point in combined_data {
        let vertex = Vertex {
            date: data_point.date,
            inflation_rate: (data_point.inflation_rate * 100.0).round() / 100.0,
            interest_rate: (data_point.interest_rate * 100.0).round() / 100.0,
        };

        let index = graph.add_node(vertex);
        node_indices.push(index);
    }

    for i in 1..graph.node_count() {
        let vertex_i = &graph[node_indices[i - 1]];
        let vertex_j = &graph[node_indices[i]];

        let distance = calculate_distance(vertex_i, vertex_j);

        if distance <= threshold {  // threshold variable is used for testing only, set to 1000.0 by default
            graph.add_edge(node_indices[i - 1], node_indices[i], distance);
        }
    }

    graph
}

pub fn _print_graph(graph: &Graph) {   // function for testing only
    for node in graph.node_indices() {
        let vertex = &graph[node];
        println!(
            "Node: {:?} (inflation: {}, interest rate: {})",
            vertex.date, vertex.inflation_rate, vertex.interest_rate
        );
    }

    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        let distance = &graph[edge];
        println!(
            "Edge: {:?} -> {:?} (distance: {})",
            graph[source].date, graph[target].date, distance
        );
    }
}

pub fn average_edge_distance(graph: &Graph) -> f64 {
    let total_distance: f64 = graph.edge_indices().map(|edge| graph[edge]).sum();
    let num_edges = graph.edge_count() as f64;
    total_distance / num_edges
}

pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_map: HashMap<usize, usize> = HashMap::new();

    for node in graph.node_indices() {
        let degree = graph.edges_directed(node, petgraph::Direction::Outgoing).count();

        let entry = degree_map.entry(degree).or_insert(0);
        *entry += 1;
    }

    degree_map
}


#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_data() -> Vec<CombinedDataPoint> {
        vec![
            CombinedDataPoint {
                date: String::from("2020-10"),
                inflation_rate: 0.04,
                interest_rate: 0.12,
            },
            CombinedDataPoint {
                date: String::from("2020-11"),
                inflation_rate: -0.06,
                interest_rate: 0.16,
            },
            CombinedDataPoint {
                date: String::from("2020-12"),
                inflation_rate: 0.09,
                interest_rate: 0.17,
            },
        ]
    }

    #[test]
    fn test_calculate_distance() {
        let data = create_sample_data();
        let vertices: Vec<Vertex> = data
            .into_iter()
            .map(|dp| Vertex {
                date: dp.date,
                inflation_rate: dp.inflation_rate,
                interest_rate: dp.interest_rate,
            })
            .collect();

        let distance = calculate_distance(&vertices[0], &vertices[1]);
        assert!((distance - 0.107703).abs() < 1e-6);
    }

    #[test]
    fn test_graph_structure() {
        let data = create_sample_data();
        let threshold = 1.0;
        let graph = construct_graph(data, threshold);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);

        let mut edge_distances: Vec<f64> = graph
            .edge_indices()
            .map(|edge| graph[edge])
            .collect();
        edge_distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert!((edge_distances[0] - 0.107703).abs() < 1e-6);
        assert!((edge_distances[1] - 0.150333).abs() < 1e-6);
    }
    #[test]
    fn test_average_edge_distance() {
        let combined_data = create_sample_data();
        let graph = construct_graph(combined_data, 1.0);
        let avg_distance = average_edge_distance(&graph);
        assert!(avg_distance > 0.0);
    }
    #[test]
    fn test_degree_distribution() {
        let data = create_sample_data();
        let threshold = 1.0;
        let graph = construct_graph(data, threshold);
    
        let degree_dist = degree_distribution(&graph);
        assert_eq!(degree_dist.len(), 2);
        assert_eq!(degree_dist.get(&0), Some(&1));
        assert_eq!(degree_dist.get(&1), Some(&2));
    }
}