extern crate graph;
use graph::{EdgeFileReader, Graph};

#[test]
/// this test used to deadlock the sample negatives
/// becasue we computed wrongly the total number of negative edges
/// in undirected graphs.
fn test_load_sorted() {
    let edges_reader = EdgeFileReader::new("tests/data/macaque.tsv".to_string())
        .unwrap()
        .set_separator(Some("\t".to_string()))
        .unwrap()
        .set_verbose(Some(false))
        .set_numeric_node_ids(Some(true))
        .set_header(Some(false));

    let mut g =
        Graph::from_sorted_csv(edges_reader, None, false, 6108, 242, "Graph".to_owned()).unwrap();

    let _ = graph::test_utilities::default_test_suite(&mut g, true).unwrap();
}