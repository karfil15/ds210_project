use std::error::Error;
mod data_processing;
use data_processing::{read_inflation_data, read_interest_rate_data};
mod merge_datasets;
use merge_datasets::*;
mod graph;
use graph::*;

fn main() -> Result<(), Box<dyn Error>> {
    let inflation_data = read_inflation_data("inflation.csv").unwrap();
    let interest_rate_data = read_interest_rate_data("interest_rates.csv").unwrap();
    let combined_data = combine_datasets(inflation_data, interest_rate_data);

    let graph = construct_graph(combined_data, 1000.0);

    let avg_edge_distance = average_edge_distance(&graph);
    println!("Average edge distance: {:.4}", avg_edge_distance);

    let degree_dist = degree_distribution(&graph);
    println!("Degree distribution: {:?}", degree_dist);



    Ok(())
}




   
