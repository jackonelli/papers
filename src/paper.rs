use serde;
use serde_json;
use serde_derive;

use std::collections::HashMap;

use crate::utils::read_file;

struct Paper {
    url: String,
    synced: bool
}

impl Paper {
    pub fn new(data_file: String) -> Paper {
        let json_string = tools::io::read_file(&data_file);
        let tmp_nodes = Paper::papers_from_json(&json_string);
        Graph {
            number_of_nodes: tmp_nodes.len(),
            nodes: tmp_nodes,
            edges_connected: Graph::edges_from_json(&json_string),
        }
    }

    pub fn nodes_from_json(json_string: &String) -> HashMap<usize, Node> {
        let graph_data: GraphData = serde_json::from_str(json_string).
            expect("Could not parse json");
        let nodes = match graph_data.nodes {
            Some(nodes) => nodes,
            None => panic!("No nodes"),
        };
        nodes
    }

}
