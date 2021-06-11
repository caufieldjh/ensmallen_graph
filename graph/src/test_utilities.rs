//! Test functions used both for testing and fuzzing.
use super::*;
use itertools::Itertools;
use log::warn;
use num_traits::Zero;
use rand::Rng;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

// where to save the test files
#[cfg(target_os = "macos")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "linux")]
static DEFAULT_PATH: &str = "/tmp/";
#[cfg(target_os = "windows")]
static DEFAULT_PATH: &str = "";

const NONEXISTENT: &str = "Cthulhu is a fictional cosmic entity created by writer H. P. Lovecraft and first introduced in the short story The Call of Cthulhu,[2] published in the American pulp magazine Weird Tales in 1928. Considered a Great Old One within the pantheon of Lovecraftian cosmic entities, the creature has since been featured in numerous popular culture references. Lovecraft depicts it as a gigantic entity worshipped by cultists, in shape like an octopus, a dragon, and a caricature of human form. Its name was given to the Lovecraft-inspired universe where it and its fellow entities existed, the Cthulhu Mythos.";

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Computes a random string of the chosen length
pub fn random_string(len: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Computes a random path.
pub fn random_path(path: Option<&str>) -> String {
    Path::new(path.unwrap_or(DEFAULT_PATH))
        .join(random_string(64))
        .to_str()
        .unwrap()
        .to_string()
}

#[allow(clippy::redundant_clone)]
/// Load the Strings Protein Protein Interaction graph with given parametrization.
/// This is our default graph we use on tests.
pub fn load_ppi(
    load_nodes: bool,
    load_edge_types: bool,
    load_weights: bool,
    directed: bool,
    verbose: bool,
    skip_selfloops: bool,
) -> Graph {
    let graph_name = "STRING PPI".to_owned();
    let nodes_reader = if load_nodes {
        Some(
            NodeFileReader::new("tests/data/ppi/nodes.tsv".to_string())
                .unwrap()
                .set_verbose(Some(false))
                .set_node_types_column_number(Some(1))
                .set_nodes_column_number(Some(0))
                .set_node_types_column(Some("category".to_string()))
                .unwrap()
                .set_default_node_type(Some("default".to_string()))
                .set_nodes_column(Some("id".to_string()))
                .unwrap()
                .set_ignore_duplicates(Some(true))
                .set_separator(Some("\t"))
                .unwrap()
                .set_header(Some(true))
                .set_max_rows_number(Some(100000))
                .set_rows_to_skip(Some(0))
                .clone(),
        )
    } else {
        None
    };
    let edges_reader = EdgeFileReader::new("tests/data/ppi/edges.tsv".to_string())
        .unwrap()
        .set_verbose(Some(verbose))
        .set_ignore_duplicates(Some(true))
        .set_separator(Some("\t"))
        .unwrap()
        .set_header(Some(true))
        .set_rows_to_skip(Some(0))
        .set_sources_column(Some("subject".to_string()))
        .unwrap()
        .set_destinations_column(Some("object".to_string()))
        .unwrap()
        .set_weights_column(if load_weights {
            Some("weight".to_string())
        } else {
            None
        })
        .unwrap()
        .set_edge_types_column(if load_edge_types {
            Some("edge_label".to_string())
        } else {
            None
        })
        .unwrap()
        .set_default_edge_type(if load_edge_types {
            Some("Kebab".to_string())
        } else {
            None
        })
        .set_max_rows_number(Some(100000))
        .set_default_weight(if load_weights { Some(5.0) } else { None })
        .set_skip_selfloops(Some(skip_selfloops))
        .clone();

    let ppi = Graph::from_unsorted_csv(
        edges_reader,
        nodes_reader,
        directed,
        false,
        graph_name.clone(),
    )
    .unwrap();
    assert_eq!(ppi.has_node_types(), load_nodes);
    assert_eq!(ppi.has_edge_types(), load_edge_types,);
    assert_eq!(ppi.has_edge_weights(), load_weights);
    assert_eq!(
        ppi.has_selfloops(),
        !skip_selfloops,
        concat!(
            "I was expecting the graph self-loops status to be {} ",
            "since we have given parameter skip_selfloops equal to {}, ",
            "but actually is {}.\n",
            "The graph report is: \n {:?}"
        ),
        !skip_selfloops,
        skip_selfloops,
        ppi.has_selfloops(),
        ppi.textual_report()
    );
    ppi
}

/// Load an empty graph instance
pub fn load_empty_graph(directed: bool) -> Graph {
    Graph::from_integer_sorted(
        std::iter::empty(),
        0,
        Vocabulary::default(),
        None,
        None,
        directed,
        false,
        "Empty graph",
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    )
    .unwrap()
}

#[allow(clippy::redundant_clone)]
/// This is our default graph we use on tests with node types.
pub fn load_cora() -> Graph {
    let graph_name = "Cora".to_owned();
    let edges_reader = EdgeFileReader::new("tests/data/cora/edges.tsv")
        .unwrap()
        .set_separator(Some("\t"))
        .unwrap()
        .set_verbose(Some(false))
        .set_sources_column(Some("subject"))
        .unwrap()
        .set_destinations_column(Some("object"))
        .unwrap()
        .set_edge_types_column(Some("edge_type"))
        .unwrap();
    let nodes_reader = Some(
        NodeFileReader::new("tests/data/cora/nodes.tsv")
            .unwrap()
            .set_separator(Some("\t"))
            .unwrap()
            .set_nodes_column(Some("id"))
            .unwrap()
            .set_verbose(Some(false))
            .set_node_types_column(Some("node_type"))
            .unwrap(),
    );
    Graph::from_unsorted_csv(edges_reader, nodes_reader, false, false, graph_name.clone()).unwrap()
}

/// Return WalksParameters to execute a first order walk.
pub fn first_order_walker(graph: &Graph) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(8)?
        .set_iterations(Some(1))?
        .set_random_state(Some(43))
        .set_dense_node_mapping(Some(graph.get_dense_nodes_mapping())))
}

/// Return WalksParameters to execute a second order walk.
pub fn second_order_walker(
    graph: &Graph,
    return_weight: WeightT,
    explore_weight: WeightT,
) -> Result<WalksParameters, String> {
    Ok(WalksParameters::new(8)?
        .set_iterations(Some(1))?
        .set_return_weight(Some(return_weight))?
        .set_explore_weight(Some(explore_weight))?
        .set_max_neighbours(Some(3))?
        .set_change_edge_type_weight(Some(2.0))?
        .set_change_node_type_weight(Some(2.0))?
        .set_dense_node_mapping(Some(graph.get_dense_nodes_mapping()))
        .set_random_state(Some(43)))
}

fn validate_vocabularies(graph: &Graph) {
    if let Some(ets) = &graph.edge_types {
        assert_eq!(!ets.ids.is_empty(), graph.has_edge_types(),
            "We expected that if the graph has edge types then it cannot be empty. The report of the graph is:\n{:?}",
            graph.textual_report()
        );
    }

    if let Some(nts) = &graph.node_types {
        assert_eq!(!nts.ids.is_empty(), graph.has_node_types());
    }

    if let Some(ws) = &graph.weights {
        assert_eq!(
            !ws.is_empty(), graph.has_edge_weights(),
            concat!(
                "We expect the edge weights vector to NOT be empty if the graph says it has weights.\n",
                "The graph report is:\n{:?}"
            ),
            graph.textual_report()
        );
    }
}

/// Executes the default test suite for holdouts.
pub fn default_holdout_test_suite(
    graph: &Graph,
    train: &Graph,
    test: &Graph,
) -> Result<(), String> {
    for g in &[graph, train, test] {
        validate_vocabularies(g);
    }
    assert!(
        !train.overlaps(&test)?,
        "Training graph overlaps with test graph!"
    );
    assert!(
        !test.overlaps(&train)?,
        "Test graph overlaps with training graph!"
    );
    assert!(graph.contains(&train)?, "Graph does not training graph.");
    assert!(graph.contains(&test)?, "Graph does not contain test graph.");
    let summed = (train | test)?;
    validate_vocabularies(&summed);
    assert!(
        summed.contains(&graph)?,
        "Composed train and test graph do not contained original graph."
    );
    let subtracted = (graph - test)?;
    validate_vocabularies(&subtracted);
    assert!(
        subtracted.contains(&train)?,
        "Main graph subtracted test does not contain training graph."
    );
    assert!(
        !subtracted.overlaps(&test)?,
        "Main graph subtracted train does not contain test graph."
    );
    let xorred = (graph ^ test)?;
    validate_vocabularies(&xorred);
    assert!(
        xorred.contains(&train)?,
        "Main graph xorred test does not contain training graph."
    );
    assert!(
        !xorred.overlaps(&test)?,
        "Main graph xorred train does not contain testing graph."
    );
    let anded = (graph & test)?;
    validate_vocabularies(&anded);
    assert!(
        anded.contains(&test)?,
        "Main graph anded test does not contain training graph."
    );

    Ok(())
}

/// Test that the spanning arborescence algorithm from bader is working correctly.
pub fn test_spanning_arborescence_bader(graph: &Graph, verbose: Option<bool>) {
    let kruskal_tree = graph.spanning_arborescence_kruskal(verbose).0;
    let random_kruskal_tree = graph
        .random_spanning_arborescence_kruskal(Some(42), None, verbose)
        .0;
    if !graph.directed {
        let spanning_arborescence_bader: Vec<(NodeT, NodeT)> =
            graph.spanning_arborescence(verbose).unwrap().1.collect();
        assert_eq!(
            spanning_arborescence_bader.len(), kruskal_tree.len(),
            "The number of extracted edges forming the spanning arborescence computed by the bader's algorithm does not match the one computed by kruskal. The graph report is:\n{:?}\nThe bader's tree is:\n{:?}\nThe kruskal's tree is:\n{:?}",
            graph.textual_report(), spanning_arborescence_bader, kruskal_tree,
        );
    } else {
        assert!(graph.spanning_arborescence(verbose).is_err());
    }
    assert_eq!(random_kruskal_tree.len() as usize, kruskal_tree.len());
}

pub fn test_graph_properties(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    // Testing that vocabularies are properly loaded
    validate_vocabularies(graph);

    // If the graph is undirected, all the edges must have their symmetrical one
    if !graph.is_directed() {
        graph
            .par_iter_edge_node_ids(true)
            .for_each(|(_, src_node_id, dst_node_id)| {
                assert!(
                    graph.has_edge_from_node_ids(dst_node_id, src_node_id),
                    concat!(
                        "In an undirected graph, for every edge there must ",
                        "have its own symmetric edge.\n",
                        "In the provided graph instance, for the edge from ",
                        "the source node ID {} to the destination node ID {} ",
                        "the symmetric edge does not exist.\n",
                        "This error is likely caused by some mis-parametrization ",
                        "in a method that is expected to produce a simmetric graph."
                    ),
                    src_node_id,
                    dst_node_id
                );
            });
    }

    assert_eq!(
        graph.iter_unweighted_node_degrees().is_sorted(),
        graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
        concat!(
            "The cached value for the method ",
            "has_nodes_sorted_by_increasing_outbound_node_degree ",
            "does not match the computed method.\n",
            "The degrees of this graph are:\n {:?}\n",
            "The reported order was: {}"
        ),
        graph.get_unweighted_node_degrees(),
        graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
    );
    let mut degrees = graph.get_unweighted_node_degrees();
    degrees.reverse();
    assert_eq!(
        degrees.is_sorted(),
        graph.has_nodes_sorted_by_decreasing_outbound_node_degree(),
        concat!(
            "The cached value for the method ",
            "has_nodes_sorted_by_decreasing_outbound_node_degree ",
            "does not match the computed method."
        )
    );

    if let (Ok(min_degree), Ok(max_degree)) = (
        graph.get_unweighted_minimum_node_degree(),
        graph.get_unweighted_maximum_node_degree(),
    ) {
        assert_eq!(
            graph.has_nodes_sorted_by_decreasing_outbound_node_degree()
                && graph.has_nodes_sorted_by_increasing_outbound_node_degree(),
            min_degree == max_degree
        );
    }

    // Test that the weights do not contain zeros.
    if graph.has_edge_weights() {
        graph.iter_edge_weights().unwrap().for_each(|w| {
            assert!(!w.is_zero(), "The graph cannot contain a zero weight. ");
            assert!(
                !w.is_infinite(),
                "The graph cannot contain an infinite weight. "
            );
            assert!(!w.is_nan(), "The graph cannot contain a nan weight. ");
        });
        // If the graph is undirected, the edge weights must be symmetrical
        if !graph.is_directed() {
            graph
                .iter_edge_node_ids(false)
                .for_each(|(_, src_node_id, dst_node_id)| unsafe {
                    assert!(
                        (graph.get_unchecked_edge_weight_from_node_ids(src_node_id, dst_node_id)
                            - graph
                                .get_unchecked_edge_weight_from_node_ids(dst_node_id, src_node_id))
                        .abs()
                            < WeightT::EPSILON,
                        concat!(
                            "In an undirected graph, we expect for the edge weights to be symmetrical ",
                            "but in the provided graph there has been found a case where the edge ",
                            "from {} to {} has weight {}, while the edge from {} to {} has ",
                            "weight {}, creating an asymetrical case."
                        ),
                        src_node_id,
                        dst_node_id,
                        graph.get_unchecked_edge_weight_from_node_ids(src_node_id, dst_node_id),
                        dst_node_id,
                        src_node_id,
                        graph.get_unchecked_edge_weight_from_node_ids(dst_node_id, src_node_id),

                    );
                });
        }
    }

    // Testing that the degrees computation is correct
    assert_eq!(
        graph.get_unweighted_maximum_node_degree()?,
        graph.iter_unweighted_node_degrees().max().unwrap(),
        "The cached maximum degree does not match the one computed from the node degrees."
    );

    if graph.has_singleton_nodes() {
        assert!(graph.get_unweighted_minimum_node_degree()? == 0);
        assert!(graph.iter_unweighted_node_degrees().min().unwrap() == 0);
    }

    if !graph.is_directed() && !graph.has_singleton_nodes() {
        assert!(graph.get_unweighted_minimum_node_degree()? > 0);
        assert!(graph.iter_unweighted_node_degrees().min().unwrap() > 0);
    }

    if !graph.is_directed() && graph.get_unweighted_minimum_node_degree()? == 0 {
        assert!(graph.has_singleton_nodes());
    }

    if graph.has_singleton_nodes() {
        assert!(graph.unique_sources.is_some());
    }

    if !graph.has_disconnected_nodes() && !graph.has_trap_nodes() {
        assert!(graph.get_unweighted_minimum_node_degree()? > 0);
        assert!(
            graph.iter_unweighted_node_degrees().min().unwrap() > 0,
            concat!(
                "Since the graph does not contain disconnected nodes nor it ",
                "contains trap nodes, the minimum outbound node degree must be ",
                "greater than zero.\n\n",
                "The graph edges are: {:?}"
            ),
            graph.get_edge_node_ids(true)
        );
    }

    assert_eq!(
        graph.singleton_nodes_with_selfloops.is_some(),
        graph.has_singleton_nodes_with_selfloops(),
    );

    assert_eq!(
        graph.get_unweighted_minimum_node_degree()?,
        graph.iter_unweighted_node_degrees().min().unwrap(),
        concat!(
            "The cached minimum degree does not match the one computed from the node degrees.\n",
            "The outbound node degrees are: {:?}"
        ),
        graph.get_unweighted_node_degrees()
    );

    if graph.has_edge_weights() {
        assert!(
            (graph.get_weighted_maximum_node_degree()?
                - graph
                    .iter_weighted_node_degrees()?
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap())
            .abs()
                < f64::EPSILON,
            concat!(
                "The cached weighted maximum degree ({}) ",
                "does not match the one computed from the node degrees ({}), ",
                "where the node degrees list is {:?}.\n",
                "Additionally the number of weighted singleton nodes is {:?}."
            ),
            graph.get_weighted_maximum_node_degree()?,
            graph
                .iter_weighted_node_degrees()?
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            graph.get_weighted_node_degrees(),
            graph.get_weighted_singleton_nodes_number()
        );
        assert!(
            (graph.get_weighted_mininum_node_degree()?
                - graph
                    .iter_weighted_node_degrees()?
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap())
            .abs()
                < f64::EPSILON,
            "The cached weighted minimum degree ({:?}) does not match the one computed from the node degrees ({:?}).",
            graph.get_weighted_mininum_node_degree()?,
            graph
                    .iter_weighted_node_degrees()?
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
        );
    }

    assert_eq!(
        graph
            .singleton_nodes_with_selfloops
            .as_ref()
            .map_or(0, |x| x.len() as NodeT),
        graph.get_singleton_nodes_with_selfloops_number(),
    );

    for singleton_node_id in graph.iter_singleton_node_ids() {
        assert!(
            unsafe { graph.get_unchecked_unweighted_node_degree_from_node_id(singleton_node_id) }
                == 0
        );
        assert!(unsafe { graph.is_unchecked_singleton_from_node_id(singleton_node_id) });
    }

    if !graph.is_directed() {
        for node_id in graph.iter_node_ids() {
            unsafe {
                assert_eq!(
                    graph.is_unchecked_singleton_from_node_id(node_id),
                    graph.get_unchecked_unweighted_node_degree_from_node_id(node_id) == 0
                )
            };
        }
    }

    // Test get_edge_id_from_node_names_and_edge_type_name()
    assert!(
        graph
            .get_edge_id_from_node_names_and_edge_type_name(NONEXISTENT, NONEXISTENT, None)
            .is_err(),
        "Graph contains non-existing edge."
    );

    // Test has_node_from_name
    assert!(
        !(graph.has_node_name_and_node_type_name(NONEXISTENT, None)),
        "The graph seems to have a non-existing node."
    );
    assert!(
        !(graph.has_node_name(NONEXISTENT)),
        "The graph seems to have a non-existing node."
    );

    // Singletons with selfloops can only exist if the graph has nodes AND selfloops
    if graph.has_singleton_nodes() {
        assert!(graph.has_nodes());
    }
    if graph.has_singleton_nodes_with_selfloops() {
        assert!(graph.has_nodes());
        assert!(graph.has_selfloops());
    }

    // Test translate_edge|node_types()
    assert!(
        graph
            .get_edge_type_ids_from_edge_type_names(vec![Some(NONEXISTENT.to_string())])
            .is_err(),
        "The graph seems to have a non-existing edge type."
    );

    assert!(
        graph
            .get_node_type_ids_from_node_type_names(vec![Some(NONEXISTENT.to_string())])
            .is_err(),
        "The graph seems to have a non-existing node type."
    );

    assert_eq!(
        graph.get_connected_nodes_number()
            + graph.get_singleton_nodes_number()
            + graph.get_singleton_nodes_with_selfloops_number(),
        graph.get_nodes_number(),
        "Sum of singleton and non singleton nodes number does not match."
    );

    assert_eq!(
        graph.get_disconnected_nodes_number(), graph.get_singleton_nodes_number() + graph.get_singleton_nodes_with_selfloops_number(),
        "Sum of singleton and singleton with selfloops does not match the number of disconnected nodes."
    );

    warn!("Running connected components tests.");
    let (_components_number, smallest, biggest) = graph.get_connected_components_number(None);
    assert!(
        biggest >= smallest,
        "smallest: {} biggest: {}",
        smallest,
        biggest
    );

    if smallest == 1 {
        assert!(
            graph.has_singleton_nodes() || graph.has_singleton_nodes_with_selfloops(),
            "When the smallest component is one the graph must have singletons! Graph report: \n{:?}",
            graph.textual_report()
        );
    }

    assert_eq!(
        !graph.has_nodes(),
        smallest == 0,
        "When the smallest component is zero the graph must be empty! Graph report: \n{:?}",
        graph.textual_report()
    );

    // Get one edge from the graph if there are any presents
    if let Some(edge) = graph.iter_unique_edge_node_ids(true).next() {
        let src_string = unsafe { graph.get_unchecked_node_name_from_node_id(edge.0) };
        let dst_string = unsafe { graph.get_unchecked_node_name_from_node_id(edge.1) };
        let edge_id = graph.get_edge_id_from_node_names(&src_string, &dst_string)?;
        if graph.has_edge_types() {
            let edge_type = graph.get_edge_type_name_from_edge_id(edge_id)?;
            let clone_edge_type = edge_type.clone();
            assert!(
                graph.has_edge_from_node_names_and_edge_type_name(&src_string, &dst_string, clone_edge_type.as_deref()),
                "I was expecting for the edge ({}, {}, {:?}) to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                edge_type,
                graph.textual_report()
            );
        } else {
            assert!(
                graph.has_edge_from_node_names(&src_string, &dst_string),
                "I was expecting for the edge ({}, {}) without type to exist, but it seems to not exist in graph {:?}",
                src_string,
                dst_string,
                graph.textual_report()
            );
        }
        assert!(graph.has_node_name(&src_string) && graph.has_node_name(&dst_string));
        if graph.has_node_types() {
            assert!(
                graph.has_node_name_and_node_type_name(
                    &src_string,
                    graph.get_node_type_names_from_node_name(&src_string)?
                ) && graph.has_node_name_and_node_type_name(
                    &dst_string,
                    graph.get_node_type_names_from_node_name(&dst_string)?
                ),
                concat!(
                    "The nodes {:?} and {:?} with node types are not present in the graph.\n",
                    "The node types are {:?} and {:?}.\n",
                    "The first node existance is {}\n",
                    "The second node existance is {}\n",
                    "The graph report is {:?}"
                ),
                src_string,
                dst_string,
                graph.get_node_type_names_from_node_name(&src_string),
                graph.get_node_type_names_from_node_name(&dst_string),
                graph.has_node_name_and_node_type_name(
                    &src_string,
                    graph.get_node_type_names_from_node_name(&src_string)?
                ),
                graph.has_node_name_and_node_type_name(
                    &dst_string,
                    graph.get_node_type_names_from_node_name(&dst_string)?
                ),
                graph.textual_report()
            );
        }
        assert_eq!(
            graph.get_edge_id_from_node_names(&src_string, &dst_string)?,
            graph.get_edge_id_from_node_ids(edge.0, edge.1).unwrap(),
            "Check of given edge ID does not match."
        );
    }

    // Test the generation of the textual report, this includes the connected components algorithm.
    graph.report();
    graph.textual_report();
    graph.overlap_textual_report(&graph, verbose)?;

    // Compute degrees metrics
    for src in 0..5 {
        for dst in 0..5 {
            let _ = graph.get_unweighted_preferential_attachment_from_node_ids(src, dst, true);
            let _ = graph.get_unweighted_preferential_attachment_from_node_ids(src, dst, false);
            let _ = graph.get_jaccard_coefficient_from_node_ids(src, dst);
            let _ = graph.get_adamic_adar_index_from_node_ids(src, dst);
            let _ = graph.get_unweighted_resource_allocation_index_from_node_ids(src, dst);
            if graph.has_edge_weights() {
                let _ = graph.get_weighted_preferential_attachment_from_node_ids(src, dst, true);
                let _ = graph.get_weighted_preferential_attachment_from_node_ids(src, dst, false);
                let _ = graph.get_weighted_resource_allocation_index_from_node_ids(src, dst);
            }
        }
    }

    assert_eq!(
        graph.has_node_types(),
        graph.get_node_type_id_from_node_id(0).is_ok()
    );

    assert!(
        graph.get_node_type_id_from_node_id(graph.get_nodes_number() + 1).is_err(),
        "Given graph does not raise an exception when a node's node type greater than the number of available nodes is requested."
    );

    assert_eq!(
        graph.has_edge_types(),
        graph.get_edge_type_id_from_edge_id(0).is_ok()
    );

    assert!(
        graph.get_edge_type_id_from_edge_id(graph.get_directed_edges_number() + 1).is_err(),
        "Given graph does not raise an exception when a edge's edge type greater than the number of available edges is requested."
    );

    // Evaluate get_node_type
    assert_eq!(
        graph.get_node_type_id_from_node_id(0).is_ok(),
        graph.has_node_types()
    );

    // Evaluate get_edge_type
    assert_eq!(
        graph.get_edge_type_id_from_edge_id(0).is_ok(),
        graph.has_edge_types()
    );

    // Evaluate get_edge_type_counts
    assert_eq!(
        graph.get_edge_type_id_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    // Evaluate get_edge_type_id_counts_hashmap
    assert_eq!(
        graph.get_edge_type_id_counts_hashmap().is_ok(),
        graph.has_edge_types()
    );

    graph.set_name(graph.get_name());
    graph.strongly_connected_components();

    // Checking that the connected components are a dense range.
    let (_, connected_components, total_connected_components, _, _) =
        graph.random_spanning_arborescence_kruskal(Some(42), None, verbose);
    let actual_components_number = connected_components.iter().unique().count() as NodeT;
    assert_eq!(
        actual_components_number,
        total_connected_components,
        "The measured number of connected components ({}) does not match the computed number of connected components ({}).",
        actual_components_number,
        total_connected_components
    );
    let max_component_id = connected_components.iter().max();
    if let Some(mci) = max_component_id {
        assert_eq!(
            *mci as usize,
            total_connected_components as usize - 1,
            "We expected the connected components to be a dense set.\n The obtained components are: \n{:?}\n The graph report is:\n{:?}",
            connected_components,
            graph.textual_report()
        );
    }
    if !graph.is_directed() {
        // Checking that the connected components are a dense range.
        let (connected_components, total_connected_components, _, _) =
            graph.connected_components(verbose)?;
        let actual_components_number = connected_components.iter().unique().count() as NodeT;
        assert_eq!(
            actual_components_number,
            total_connected_components,
            "The measured number of connected components ({}) does not match the computed number of connected components ({}).",
            actual_components_number,
            total_connected_components
        );
        let max_component_id = connected_components.iter().max();
        if let Some(mci) = max_component_id {
            assert_eq!(
                *mci as usize,
                total_connected_components as usize - 1,
                "We expected the connected components to be a dense set.\n The obtained components are: \n{:?}\n The graph report is:\n{:?}",
                connected_components,
                graph.textual_report()
            );
        }
    }
    Ok(())
}

pub fn test_node_centralities(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    let node_degree_centralities = graph.get_unweighted_degree_centrality()?;

    assert_eq!(
        node_degree_centralities.len(),
        graph.get_nodes_number() as usize
    );

    assert!(
        node_degree_centralities
            .iter()
            .cloned()
            .all(|value| value <= 1.0 && value >= 0.0),
        "All node degrees centralities are expected to be within 0 and 1, but are {:?}.",
        node_degree_centralities
    );

    if graph.has_edge_weights() {
        let node_degree_centralities = graph.get_weighted_degree_centrality()?;

        assert_eq!(
            node_degree_centralities.len(),
            graph.get_nodes_number() as usize
        );

        assert!(
            node_degree_centralities
                .iter()
                .cloned()
                .all(|value| value <= 1.0 && value >= 0.0),
            concat!(
                "All weighted node degrees centralities ",
                "are expected to be within 0 and 1, ",
                "but are {:?} and the node degrees are {:?}, with the ",
                "minimum weighted node degree being {} and ",
                "maximum weighted node degree being {}.",
            ),
            node_degree_centralities,
            graph.get_weighted_node_degrees(),
            graph.get_weighted_mininum_node_degree()?,
            graph.get_weighted_maximum_node_degree()?,
        );
    }

    let node_betweenness_centralities = graph.get_betweenness_centrality(None, verbose);
    assert_eq!(
        node_betweenness_centralities.len(),
        graph.get_nodes_number() as usize
    );
    node_betweenness_centralities
        .into_iter()
        .enumerate()
        .for_each(|(node_id, value)| {
            if unsafe { graph.is_unchecked_singleton_from_node_id(node_id as NodeT) } {
                assert!(value.abs() < f64::EPSILON);
            }
        });
    Ok(())
}

pub fn test_vertex_cover(graph: &mut Graph, _verbose: Option<bool>) -> Result<(), String> {
    let vertex_cover = graph.approximated_vertex_cover_set();
    graph
        .par_iter_edge_node_ids(true)
        .for_each(|(_, src_node_id, dst_node_id)| {
            assert!(
                vertex_cover.contains(&src_node_id) || vertex_cover.contains(&dst_node_id),
                concat!(
                    "We expected for either the node {} or {} to be in the vertex cover.\n",
                    "The vertex cover is {:?}"
                ),
                src_node_id,
                dst_node_id,
                vertex_cover
            );
        });
    Ok(())
}

pub fn test_bfs(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    // We avoid running this test on too big graphs so to avoid slowing down the test suite
    if graph.get_nodes_number() > 100 {
        return Ok(());
    }

    // If the graph is empty the other tests on BFS make little sense
    if !graph.has_nodes() {
        assert!(graph
            .get_breath_first_search_from_node_ids(0, None, None, None, None, None, None)
            .is_err());
        return Ok(());
    }

    // BFS on an unweighted graph gives simmetric results.
    if !graph.is_directed() {
        let components_ids = graph.get_node_connected_component_ids(verbose);
        for maximal_depth in [None, Some(1), Some(2), Some(3)] {
            graph.iter_node_ids().for_each(|src_node_id| {
                graph.iter_node_ids().for_each(|dst_node_id| unsafe {
                    // Check that the obtained results are simmetric
                    let src_to_dst = graph
                        .get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(
                            src_node_id,
                            dst_node_id,
                            maximal_depth,
                        );
                    let dst_to_src = graph
                        .get_unchecked_unweighted_minimum_path_node_ids_from_node_ids(
                            dst_node_id,
                            src_node_id,
                            maximal_depth,
                        );
                    if src_node_id == dst_node_id {
                        assert!(src_to_dst.is_err());
                        assert!(dst_to_src.is_err());
                        return;
                    }
                    if components_ids[src_node_id as usize] != components_ids[dst_node_id as usize]
                    {
                        assert!(src_to_dst.is_err());
                        assert!(dst_to_src.is_err());
                        return;
                    }
                    if let (Ok(src_to_dst), Ok(dst_to_src)) = (src_to_dst, dst_to_src) {
                        // Check that the two paths have the same length
                        assert_eq!(src_to_dst.len(), dst_to_src.len());
                        assert_eq!(src_to_dst, dst_to_src.into_iter().rev().collect::<Vec<_>>());
                        // Test that the k-paths return a compatible result
                        let kpaths = graph
                            .get_unchecked_unweighted_k_shortest_path_node_ids_from_node_ids(
                                src_node_id,
                                dst_node_id,
                                5,
                            );
                        let min_length = kpaths.into_iter().map(|path| path.len()).min().unwrap();
                        assert_eq!(min_length, src_to_dst.len());
                    }
                });
            });
        }
    }
    Ok(())
}

pub fn test_dijkstra(graph: &mut Graph, _verbose: Option<bool>) -> Result<(), String> {
    // We avoid running this test on too big graphs so to avoid slowing down the test suite
    if graph.get_nodes_number() > 100 {
        return Ok(());
    }
    // If the graph is empty the other tests on Dijkstra make little sense
    if !graph.has_nodes() {
        assert!(graph
            .get_dijkstra_from_node_ids(0, None, None, None, None, None)
            .is_err());
        return Ok(());
    }
    // Dijkstra on unweighted graphs does not make sense
    // Dijkstra on weighted graphs with negative weights does not make sense
    if !graph.has_edge_weights() || graph.has_negative_edge_weights().unwrap() {
        assert!(graph
            .get_dijkstra_from_node_names(
                unsafe { graph.get_unchecked_node_name_from_node_id(0).as_ref() },
                None,
                None,
                None,
                None,
                None
            )
            .is_err());
        return Ok(());
    }
    // Dijkstra on an unweighted graph gives simmetric results.
    if !graph.is_directed() {
        for use_edge_weights_as_probabilities in [true, false] {
            if use_edge_weights_as_probabilities
                && !graph.has_edge_weights_representing_probabilities().unwrap()
            {
                continue;
            }
            graph.iter_node_ids().for_each(|src_node_id| {
                graph.iter_node_ids().for_each(|dst_node_id| unsafe {
                    // Check that the obtained results are simmetric
                    let (src_to_dst_distance, src_to_dst) = graph
                        .get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                            src_node_id,
                            dst_node_id,
                            Some(use_edge_weights_as_probabilities),
                            None,
                        );
                    let (dst_to_src_distance, dst_to_src) = graph
                        .get_unchecked_weighted_minimum_path_node_ids_from_node_ids(
                            dst_node_id,
                            src_node_id,
                            Some(use_edge_weights_as_probabilities),
                            None,
                        );
                    let src_to_dst_distance = src_to_dst_distance as WeightT;
                    let dst_to_src_distance = dst_to_src_distance as WeightT;
                    assert!(
                        // We need both checks because both distances
                        // my be infinite, and therefore the epsilon check
                        // may not be enough.
                        src_to_dst_distance.is_infinite() && dst_to_src_distance.is_infinite()
                            || (src_to_dst_distance - dst_to_src_distance).abs() < WeightT::EPSILON,
                        concat!(
                            "The path from source to destination has distance {} ",
                            "while the distance from destination to source has ",
                            "destination {}. The path from source to destination ",
                            "is {:?}, while the path from destination to source ",
                            "is {:?}. The two paths should be symmetric and with ",
                            "the same distance.\nThe graph report is:\n{:?}"
                        ),
                        src_to_dst_distance,
                        dst_to_src_distance,
                        src_to_dst,
                        dst_to_src,
                        graph.textual_report()
                    );
                });
            });
        }
    }
    Ok(())
}

pub fn test_polygons(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    assert_eq!(
        graph
            .get_unweighted_number_of_triangles_per_node(Some(false), None, verbose)
            .into_iter()
            .map(|triangles_number| triangles_number as EdgeT)
            .sum::<EdgeT>(),
        graph.get_unweighted_number_of_triangles(Some(false), None, verbose)
    );
    Ok(())
}

pub fn test_transitivity(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    // We skip this test of graph with more than 1000 nodes to avoid slowing down
    // too much the test suite.
    if graph.get_nodes_number() > 1000 {
        return Ok(());
    }
    if !graph.has_edge_weights() && !graph.has_edge_types() {
        // We define the 0-th iteration of transitive closure as the graph itself
        assert_eq!(
            graph.clone(),
            graph.get_transitive_closure(Some(0), verbose)
        );
        // We define the first iteration of transitive closure as the graph itself
        let graph_with_selfloops = graph.add_selfloops(None, None, verbose).unwrap();
        assert_eq!(
            graph_with_selfloops,
            graph_with_selfloops.get_transitive_closure(Some(1), verbose),
            concat!(
                "We expected the original graph to equal to the graph obtained after ",
                "a single iteration of transitive closure, but they are different.\n",
                "The to_dot of the first graph is: \n {}\n",
                "The to_dot of the second graph is: \n {}\n",
            ),
            graph_with_selfloops.clone().to_dot(Some(false)),
            graph_with_selfloops
                .get_transitive_closure(Some(1), verbose)
                .to_dot(Some(false)),
        );
        // Doing multiple iterations should be equal to doing the same iteration multiple times
        let three_iterations = graph_with_selfloops.get_transitive_closure(Some(3), verbose);
        let two_times_two = graph_with_selfloops
            .get_transitive_closure(Some(2), verbose)
            .get_transitive_closure(Some(2), verbose);
        assert_eq!(
            three_iterations,
            two_times_two,
            concat!(
                "We expected the graph after 3 transitive closures to be ",
                "equal to the graph after two times two transitive closures.\n",
                "The to_dot of the first graph is: \n {}\n",
                "The to_dot of the second graph is: \n {}\n",
            ),
            three_iterations.to_dot(Some(false)),
            two_times_two.to_dot(Some(false))
        );
    }
    let mut transitive_closure = graph.get_transitive_closure(None, verbose);
    let connected_components = graph.get_node_connected_component_ids(verbose);
    if !graph.is_directed() {
        for (src_node_id, src_component_id) in connected_components.iter().cloned().enumerate() {
            if unsafe { graph.is_unchecked_singleton_from_node_id(src_node_id as NodeT) } {
                continue;
            }
            for (dst_node_id, dst_component_id) in connected_components.iter().cloned().enumerate()
            {
                assert_eq!(
                    transitive_closure
                        .has_edge_from_node_ids(src_node_id as NodeT, dst_node_id as NodeT),
                    src_component_id == dst_component_id,
                    concat!(
                        "In an undirected graph, the transitive closure of the graph should ",
                        "contain an edge between all nodes in the same component, but ",
                        "the node {} and {} have as component IDs {} and {} respectively, ",
                        "and the test has edge has returned {}."
                    ),
                    src_node_id,
                    dst_node_id,
                    src_component_id,
                    dst_component_id,
                    transitive_closure
                        .has_edge_from_node_ids(src_node_id as NodeT, dst_node_id as NodeT)
                );
            }
        }
    }
    test_graph_properties(&mut transitive_closure, verbose)?;

    Ok(())
}

pub fn test_all_paths(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    // We skip this test of graph with more than 1000 nodes to avoid slowing down
    // too much the test suite.
    if graph.get_nodes_number() > 1000 {
        return Ok(());
    }
    for iteration in [None, Some(0), Some(1), Some(2)] {
        let mut unweighted_all_paths = graph.get_unweighted_all_shortest_paths(iteration, verbose);
        test_graph_properties(&mut unweighted_all_paths, verbose)?;
    }

    if !graph.has_edge_weights() || graph.has_negative_edge_weights().unwrap() {
        assert!(graph
            .get_weighted_all_shortest_paths(None, None, verbose)
            .is_err());
        return Ok(());
    }

    for iteration in [None, Some(0), Some(1), Some(2)] {
        let mut weighted_all_paths = graph
            .get_weighted_all_shortest_paths(iteration, None, verbose)
            .unwrap();
        test_graph_properties(&mut weighted_all_paths, verbose)?;
    }

    Ok(())
}

pub fn test_selfloops(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    assert_eq!(
        graph.add_selfloops(None, Some(1.0), verbose).is_ok(),
        graph.has_edge_weights()
    );
    let mut graph_with_selfloops = graph
        .add_selfloops(
            None,
            if graph.has_edge_weights() {
                Some(1.0)
            } else {
                None
            },
            verbose,
        )
        .unwrap();
    for node_id in graph.iter_node_ids() {
        assert!(graph_with_selfloops.has_selfloop_from_node_id(node_id));
    }
    test_graph_properties(&mut graph_with_selfloops, verbose)?;

    Ok(())
}

pub fn test_random_walks(graph: &mut Graph, _verbose: Option<bool>) -> Result<(), String> {
    // Testing principal random walk algorithms
    let walker = first_order_walker(&graph)?;
    assert_eq!(walker.clone(), walker);
    let walker2 = second_order_walker(&graph, 2.0, 2.0)?;
    assert_eq!(walker2.clone(), walker2);

    if !graph.directed {
        warn!("Executing random walks tests.");
        for mode in 0..2 {
            if mode == 1 {
                graph.enable(None, None, None)?;
                if let Some(cumulative_node_degrees) = &graph.cumulative_node_degrees {
                    assert_eq!(
                        cumulative_node_degrees.len(),
                        graph.get_nodes_number() as usize,
                        "Length of cumulative_node_degrees does not match number of nodes in the graph."
                    );
                }
                if let Some(destinations) = &graph.destinations {
                    assert_eq!(
                        destinations.len(),
                        graph.get_directed_edges_number() as usize,
                        "Length of destinations does not match number of edges in the graph."
                    );
                }
            }
            assert_eq!(
                graph
                    .iter_random_walks(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_random_walks(1, &walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of first order are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_random_walks(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_random_walks(1, &second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Walks of second order are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&walker)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete first order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 2.0, 1.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );

            assert_eq!(
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                graph
                    .iter_complete_walks(&second_order_walker(&graph, 1.0, 2.0)?)
                    .map(|iter| iter.collect::<Vec<Vec<NodeT>>>()),
                "Complete second order walks are not reproducible!"
            );
        }
    } else {
        assert!(graph.iter_complete_walks(&walker).is_err());
    }
    Ok(())
}

pub fn test_edge_holdouts(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    if !graph.has_edge_types() {
        assert!(graph
            .connected_holdout(0.8, None, Some(vec![None]), Some(false), None)
            .is_err());
    }
    for include_all_edge_types in &[false, true] {
        let (train, test) = graph.random_holdout(
            0.6,
            None,
            Some(*include_all_edge_types),
            None,
            None,
            verbose,
        )?;
        default_holdout_test_suite(graph, &train, &test)?;
        let (train, test) =
            graph.connected_holdout(0.8, None, None, Some(*include_all_edge_types), verbose)?;
        let (total, min_comp, max_comp) = graph.get_connected_components_number(verbose);
        assert_eq!(
            graph.get_connected_components_number(verbose),
            train.get_connected_components_number(verbose),
            "The number of components of the original graph and the connected training set does not match. Particularly, the number of nodes in the graph is {nodes_number}.",
            nodes_number=graph.get_nodes_number().to_string()
        );
        if total == 1 {
            assert_eq!(
                min_comp,
                graph.get_nodes_number(),
                concat!(
                    "We expect for the minimum size of connected components ",
                    "in a graph with a single connected component to ",
                    "match the number of nodes of the graph, but we got ",
                    "the minimum component with size {} and the number ",
                    "of nodes in the graph equal to {}.\n",
                    "The graph report is: \n {:?}",
                ),
                min_comp,
                graph.get_nodes_number(),
                graph.textual_report()
            );
            assert_eq!(max_comp, graph.get_nodes_number());
            assert_eq!(min_comp, test.get_nodes_number());
            assert_eq!(max_comp, test.get_nodes_number());
        }
        if total == 2 {
            assert_eq!(
                max_comp + min_comp, graph.get_nodes_number(),
                "We expected that the number of the minimum component ({}) plus the maximum component ({}), when the components are two, made up the graph nodes ({}).\nThe graph report is:\n {:?}",
                min_comp, max_comp, graph.get_nodes_number(),
                graph.textual_report()
            );
            assert_eq!(max_comp + min_comp, test.get_nodes_number());
        }
        default_holdout_test_suite(graph, &train, &test)?;
    }
    Ok(())
}

pub fn test_remove_components(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    if graph.get_connected_components_number(verbose).0 > 1 {
        let without_selfloops = graph.drop_selfloops(verbose);

        assert_eq!(
            graph.get_connected_components_number(verbose),
            without_selfloops.get_connected_components_number(verbose),
            concat!(
                "We expected the graph to have the same components once we remove the selfloops.\n",
                "The report of the original graph is {:?}\n",
                "The report of the filtered graph is {:?}\n",
                "The edge node ids of the original graph are {:?}\n",
                "The edge node ids of the filtered graph are {:?}\n"
            ),
            graph.textual_report(),
            without_selfloops.textual_report(),
            graph.get_edge_node_ids(true),
            without_selfloops.get_edge_node_ids(true),
        );

        let single_component = graph.remove_components(None, None, None, None, Some(1), verbose);
        assert!(
            single_component.is_ok(),
            concat!(
                "Removing all the components except the first one returned an error.\n",
                "The error is:\n{:?}\nand the graph report is:\n{:?}"
            ),
            single_component,
            graph.textual_report()
        );
        let single_component_number = single_component
            .unwrap()
            .get_connected_components_number(verbose)
            .0;
        assert_eq!(
            single_component_number,
            1,
            concat!(
                "Removing all the components except the first one returned a graph ",
                "with {} components, which is not one.\nThe report of the graph is:{:?}\n"
            ),
            single_component_number,
            graph.textual_report()
        );

        let test = graph.remove_components(
            Some(vec![graph.nodes.unchecked_translate(0)]),
            None,
            None,
            None,
            None,
            verbose,
        )?;
        let without_selfloops = test.drop_selfloops(verbose);
        assert_eq!(
            without_selfloops.get_connected_components_number(verbose).0,
            1,
            concat!(
                "Expected number of components (1) is not matched!\n",
                "The report of the original graph is {:?}\n",
                "The report of the graph with only one component is {:?}\n",
                "The report of the graph without selfloops is {:?}\n",
            ),
            graph.textual_report(),
            test.textual_report(),
            without_selfloops.textual_report()
        );
        if let Ok(node_type_name) = graph.get_node_type_name_from_node_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    Some(vec![Some(node_type_name)]),
                    None,
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_node_types()? {
            let without_unknowns =
                graph.remove_components(None, Some(vec![None]), None, None, None, verbose);
            assert!(
                without_unknowns.is_ok(),
                "Could not remove components without node type None.\nThe error is {:?}\nThe graph report is {:?}",
                without_unknowns, graph.textual_report()
            );
        }
        if let Ok(edge_type_name) = graph.get_edge_type_name_from_edge_type_id(0) {
            assert!(graph
                .remove_components(
                    None,
                    None,
                    Some(vec![Some(edge_type_name)]),
                    None,
                    None,
                    verbose
                )
                .is_ok());
        }
        if graph.has_unknown_edge_types()? {
            assert!(graph
                .remove_components(None, None, Some(vec![None]), None, None, verbose)
                .is_ok());
        }
    } else {
        assert!(
            graph
                .remove_components(None, None, None, None, None, verbose)
                .is_ok(),
            "We expect it to be possible, now, to create empty graphs."
        );
    }

    Ok(())
}

pub fn test_kfold(graph: &mut Graph, _verbose: Option<bool>) -> Result<(), String> {
    let k = 3;
    for i in 0..k {
        let (train, test) = graph.kfold(k, i, None, None, None)?;
        assert!(
            test.get_edges_number() <= (graph.get_edges_number() / k) + 1,
            concat!(
                "Check that test kfolds respect size bound has failed!\n",
                "The value of k is {}.\n",
                "The report of the original graph is:\n{:?}\n",
                "The report of the train graph is:\n{:?}\n",
                "The report of the test graph is:\n{:?}\n",
                "We expect that the test graph has at most {} edges but it has {}.\n",
                "The holdout index is {}.\n",
            ),
            k,
            graph.textual_report(),
            train.textual_report(),
            test.textual_report(),
            (graph.get_edges_number() / k) + 1,
            test.get_edges_number(),
            i
        );
        default_holdout_test_suite(graph, &train, &test)?;
    }

    if let Ok(edge_t) = graph.get_edge_type_name_from_edge_type_id(0) {
        for i in 0..k {
            let (train, test) = graph.kfold(k, i, Some(vec![Some(edge_t.clone())]), None, None)?;
            default_holdout_test_suite(graph, &train, &test)?;
        }
    }

    Ok(())
}

pub fn test_negative_edges_generation(
    graph: &mut Graph,
    verbose: Option<bool>,
) -> Result<(), String> {
    for only_from_same_component in &[true, false] {
        let negatives = graph.sample_negatives(
            graph.get_edges_number(),
            None,
            None,
            Some(*only_from_same_component),
            verbose,
        )?;
        assert_eq!(
            graph.get_edges_number(),
            negatives.get_edges_number(),
            "We expect the graph and its negative graph to have the same number of edges but we got {} and {}.",
            graph.get_edges_number(),
            negatives.get_edges_number()
        );
        validate_vocabularies(&negatives);
        if !graph.has_edge_types() {
            assert!(!graph.overlaps(&negatives)?);
            assert!(!negatives.overlaps(&graph)?);
        }
        // Testing holdouts executed on negative edges.
        let (neg_train, neg_test) =
            negatives.random_holdout(0.8, None, None, None, None, verbose)?;

        neg_test.get_trap_nodes_number();

        default_holdout_test_suite(&negatives, &neg_train, &neg_test)?;
    }

    Ok(())
}

pub fn test_subgraph_generation(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    let expected_nodes = graph.get_connected_nodes_number() / 10;
    let subgraph = graph.random_subgraph(expected_nodes, None, verbose)?;
    assert!(subgraph.overlaps(&graph)?);
    assert!(subgraph.get_connected_nodes_number() <= expected_nodes + 1);
    Ok(())
}

pub fn test_dump_graph(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    let node_file = random_path(None);
    let nodes_writer = NodeFileWriter::new(node_file.clone())
        .set_verbose(verbose)
        .set_separator(Some("\t"))
        .set_header(Some(true))
        .set_node_types_column_number(Some(4))
        .set_nodes_column_number(Some(6))
        .set_node_types_column(Some("node_types"))
        .set_nodes_column(Some("node_column".to_string()));
    nodes_writer.dump(&graph)?;
    fs::remove_file(node_file).unwrap();

    let edges_file = random_path(None);
    let edges_writer = EdgeFileWriter::new(edges_file.clone())
        .set_verbose(verbose)
        .set_separator(Some("\t"))
        .set_header(Some(true))
        .set_edge_types_column(Some("edge_types"))
        .set_destinations_column_number(Some(3))
        .set_weights_column(Some("weight".to_string()))
        .set_weights_column_number(Some(2))
        .set_sources_column(Some("The land of sushi".to_string()))
        .set_sources_column_number(Some(0))
        .set_destinations_column(Some("The land of pizza".to_string()))
        .set_destinations_column_number(Some(1));

    edges_writer.dump(&graph)?;
    fs::remove_file(edges_file).unwrap();

    Ok(())
}

pub fn test_embiggen_preprocessing(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    let walker = first_order_walker(&graph)?;
    if !graph.directed {
        let (terms_number, iterator) = graph.cooccurence_matrix(&walker, 3, verbose)?;
        assert_eq!(terms_number, iterator.count());

        let window_size = 3;
        let batch_size = 256;
        let data = graph
            .node2vec(&walker, batch_size, window_size)?
            .collect::<Vec<_>>();
        assert_eq!(
            data.len(),
            batch_size as usize
                * walker.iterations as usize
                * (walker.single_walk_parameters.walk_length as usize - window_size * 2)
        );
        for (context, _) in data.iter() {
            assert_eq!(context.len(), window_size * 2);
        }
    }
    if graph.has_edges() {
        graph
            .link_prediction_degrees(
                0,
                Some(256),
                Some(true),
                Some(0.3),
                Some(false),
                Some(10),
                Some(false),
                None,
            )
            .unwrap()
            .collect::<Vec<_>>();
        graph
            .link_prediction_ids(
                0,
                Some(256),
                Some(0.4),
                None,
                None,
                Some(false),
                Some(false),
                Some(10),
                Some(false),
                None,
            )
            .unwrap()
            .collect::<Vec<_>>();
    }

    Ok(())
}

pub fn test_edgelist_generation(graph: &mut Graph, _verbose: Option<bool>) -> Result<(), String> {
    let _clique = graph.get_clique_edge_names(
        None,
        None,
        Some(false),
        None,
        // limit to compute the clique for at most the first 3 nodes
        // because it's really expensive computationally.
        Some(
            graph
                .get_node_names()
                .iter()
                .take(3)
                .cloned()
                .collect::<HashSet<String>>(),
        ),
    );
    warn!("Running edge lists generator tests.");
    if graph.get_nodes_number() > 1 {
        let _bipartite = graph.get_bipartite_edge_names(
            None,
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(0) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(1) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
            None,
        )?;
        let _star = graph.get_star_edges(
            unsafe { graph.get_unchecked_node_name_from_node_id(0) },
            Some(false),
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(1) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
        let _star = graph.get_star_edge_names(
            unsafe { graph.get_unchecked_node_name_from_node_id(0) },
            Some(false),
            Some(
                [unsafe { graph.get_unchecked_node_name_from_node_id(1) }]
                    .iter()
                    .cloned()
                    .collect::<HashSet<String>>(),
            ),
            None,
        )?;
    }
    Ok(())
}

pub fn test_nodelabel_holdouts(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    for use_stratification in [true, false] {
        if graph.get_known_node_types_number()? < 2
            || (use_stratification
                && (graph.has_multilabel_node_types()? || graph.has_singleton_node_types()?))
        {
            assert!(graph
                .node_label_holdout(0.8, Some(use_stratification), Some(42))
                .is_err());
            continue;
        }

        let (train, test) = graph.node_label_holdout(0.8, Some(use_stratification), Some(42))?;
        assert!(train.has_unknown_node_types()?);
        assert!(test.has_unknown_node_types()?);
        assert!(!test
            .replace_unknown_node_types_with_node_type_name(vec!["HALLO!".to_string()], verbose)?
            .has_unknown_node_types()?);
        let remerged = &mut (&train | &test)?;
        assert_eq!(remerged.node_types, graph.node_types);
        assert!(
            remerged.contains(graph)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            graph.contains(remerged)?,
            "The re-merged holdouts does not contain the original graph."
        );
        assert!(
            train.node_types.as_ref().map_or(false, |train_nts| {
                test.node_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_node_type, test_node_type)| {
                            !(train_node_type.is_some() && test_node_type.is_some())
                        },
                    )
                })
            }),
            "The train and test node-label graphs are overlapping!"
        );
    }
    Ok(())
}

pub fn test_edgelabel_holdouts(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    for use_stratification in [true, false].iter() {
        if *use_stratification && graph.has_singleton_edge_types()?
            || graph.get_directed_edges_number() - graph.get_unknown_edge_types_number()? < 2
            || !graph.has_edge_types()
        {
            assert!(graph
                .edge_label_holdout(0.8, Some(*use_stratification), None)
                .is_err());
            continue;
        }
        let (train, test) = graph.edge_label_holdout(0.8, Some(*use_stratification), None)?;
        assert!(train.has_unknown_edge_types()?);
        assert!(test.has_unknown_edge_types()?);
        assert!(!test
            .replace_unknown_edge_types_with_edge_type_name("HALLO!".to_string(), verbose)?
            .has_unknown_edge_types()?);
        assert!(
            train.edge_types.as_ref().map_or(false, |train_nts| {
                test.edge_types.as_ref().map_or(false, |test_nts| {
                    train_nts.ids.iter().zip(test_nts.ids.iter()).all(
                        |(train_edge_type, test_edge_type)| {
                            !(train_edge_type.is_some() && test_edge_type.is_some())
                        },
                    )
                })
            }),
            "The train and test edge-label graphs are overlapping!"
        );
    }
    Ok(())
}

pub fn test_graph_filter(graph: &Graph, verbose: Option<bool>) -> Result<(), String> {
    let unfiltered = graph
        .filter_from_ids(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, verbose,
        )
        .unwrap();
    assert_eq!(&unfiltered, graph);
    assert!(graph
        .filter_from_names(
            None,
            Some(vec![NONEXISTENT]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            verbose,
        )
        .is_err());
    for node_name in graph.iter_node_names().take(10) {
        // The following test should remove ONLY the given node dijkstra
        let graph_without_given_name_result = graph.filter_from_names(
            None,
            Some(vec![node_name.as_str()]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            verbose,
        );
        assert!(graph_without_given_name_result.is_ok());
        let graph_without_given_id = graph_without_given_name_result.unwrap();
        assert_eq!(
            graph_without_given_id.has_nodes(),
            graph.get_nodes_number() > 1
        );
        assert!(!graph_without_given_id.has_node_name(node_name.as_str()));

        // The following test should keep ONLY the given node name
        let graph_with_given_name_result = graph.filter_from_names(
            Some(vec![node_name.as_str()]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            verbose,
        );
        assert!(graph_with_given_name_result.is_ok());
        let graph_with_given_node_name = graph_with_given_name_result.unwrap();
        assert_eq!(
            graph_with_given_node_name.has_selfloops(),
            graph.has_edge_from_node_names(node_name.as_ref(), node_name.as_ref())
        );
        assert_eq!(
            graph_with_given_node_name.has_edges(),
            graph_with_given_node_name.has_selfloops()
        );
        assert_eq!(graph_with_given_node_name.get_nodes_number(), 1);
        assert!(graph_with_given_node_name.has_node_name(node_name.as_str()));
    }

    for node_type_name in graph.iter_unique_node_type_names()?.take(10) {
        // The following test should remove ONLY the given node name
        let graph_without_given_node_type_name_result = graph.filter_from_names(
            None,
            None,
            None,
            None,
            None,
            Some(vec![Some(node_type_name.clone())]),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            verbose,
        );
        assert!(graph_without_given_node_type_name_result.is_ok());
        let graph_without_given_node_type_name = graph_without_given_node_type_name_result.unwrap();
        if graph.get_node_types_number()? > 1 && !graph.has_multilabel_node_types()? {
            assert!(graph_without_given_node_type_name.has_node_types());
            assert!(graph_without_given_node_type_name.has_nodes());
        }
        assert!(!graph_without_given_node_type_name.has_node_type_name(node_type_name.as_str()));
    }

    Ok(())
}

pub fn test_graph_removes(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    let without_edge_types = graph.remove_edge_types(verbose)?;
    validate_vocabularies(&without_edge_types);
    assert!(!without_edge_types.has_edge_types());
    assert_eq!(
        without_edge_types.has_edge_weights(),
        graph.has_edge_weights()
    );
    assert_eq!(without_edge_types.node_types, graph.node_types);
    if !graph.is_multigraph() {
        assert_eq!(
            without_edge_types.get_unique_edges_number(),
            graph.get_unique_edges_number(),
            concat!(
                "Number of unique edges does not match in graph without edge types.\n",
                "The report of the original graph is \n{:?}\n",
                "The report of the graph without edge types is \n{:?}",
            ),
            graph.textual_report(),
            without_edge_types.textual_report()
        );
        assert_eq!(
            without_edge_types.get_unique_selfloop_number(),
            graph.get_unique_selfloop_number(),
            "Number of unique self loops does not match in graph without edge types."
        );
    }
    assert_eq!(without_edge_types.nodes, graph.nodes);
    let without_node_types = graph.remove_node_types()?;
    validate_vocabularies(&without_node_types);
    assert!(!without_node_types.has_node_types());
    assert_eq!(
        graph.is_multigraph(),
        without_node_types.is_multigraph(),
        "If the original graph is a multigraph, the removal of node types should not change that."
    );
    assert_eq!(
        without_node_types.weights,
        graph.weights,
        concat!(
            "We expected the weights not to change when removig node types.",
            "\nThe report of the original graph is {:?}.",
            "\nThe report of the filtered graph is {:?}."
        ),
        graph.textual_report(),
        without_node_types.textual_report()
    );
    assert_eq!(without_node_types.has_selfloops(), graph.has_selfloops());
    assert_eq!(without_node_types.nodes, graph.nodes);
    let without_weights = graph.remove_edge_weights()?;
    validate_vocabularies(&without_weights);
    assert!(!without_weights.has_edge_weights());
    assert_eq!(without_weights.node_types, graph.node_types);
    assert_eq!(without_weights.has_selfloops(), graph.has_selfloops());
    assert_eq!(without_weights.nodes, graph.nodes);

    Ok(())
}

pub fn test_clone_and_setters(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    let mut clone = graph.clone();
    clone = clone.set_all_edge_types("TEST_SET_ALL_EDGE_TYPES", verbose)?;
    assert!(!clone.is_multigraph());
    clone = clone.set_all_node_types("TEST_SET_ALL_NODE_TYPES")?;

    assert_eq!(
        clone.get_edge_types_number().unwrap(),
        1,
        "Number of edge types of the graph is not 1."
    );
    if !graph.is_multigraph() {
        assert_eq!(
            unsafe{clone.get_unchecked_edge_count_from_edge_type_id(Some(0))},
            graph.get_directed_edges_number(),
            "Number of edges with the unique edge type does not match number of edges in the graph."
        );
    }

    assert_eq!(
        clone.get_node_types_number().unwrap(),
        1,
        "Number of node types of the graph is not 1."
    );
    assert_eq!(
        clone.get_unchecked_node_count_from_node_type_id(Some(0)),
        graph.get_nodes_number(),
        "Number of nodes with the unique node type does not match number of nodes in the graph."
    );

    Ok(())
}

pub fn test_graph_remapping(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    assert!(
        graph.are_nodes_remappable(&graph),
        "Graph always should be remappable to itself."
    );
    assert!(
        graph.remap(&graph, verbose).is_ok(),
        "Graph always should be remappable to itself."
    );
    Ok(())
}

pub fn test_graph_diameter(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    // TODO! update this when we will support the graph diameter on directed graphs
    if graph.is_directed() {
        return Ok(());
    }
    let (n_of_components, _, biggest) = graph.get_connected_components_number(verbose);

    match n_of_components {
        0 => {
            // on an empty graph this should always fail
            assert!(graph.get_unweighted_diameter(Some(false), verbose).is_err());
            assert!(graph.get_unweighted_diameter(Some(true), verbose).is_err());
        }

        1 => {
            // by definition the diameter of a graph with a single component
            // cannot be infinite unless it's just a singleton.
            if graph.get_nodes_number() == 1 {
                assert!(graph
                    .get_unweighted_diameter(Some(false), verbose)
                    .unwrap()
                    .is_infinite());
                assert!(graph
                    .get_unweighted_diameter(Some(true), verbose)
                    .unwrap()
                    .is_infinite());
            } else {
                assert!(graph
                    .get_unweighted_diameter(Some(false), verbose)
                    .unwrap()
                    .is_finite());
                assert!(graph
                    .get_unweighted_diameter(Some(true), verbose)
                    .unwrap()
                    .is_finite());
            }
        }

        _ => {
            assert!(graph
                .get_unweighted_diameter(Some(false), verbose)
                .unwrap()
                .is_infinite());

            // if all the nodes are singletons then the diameter should be infinite
            if biggest == 1 {
                assert!(graph
                    .get_unweighted_diameter(Some(true), verbose)
                    .unwrap()
                    .is_infinite());
            }
        }
    }

    Ok(())
}

/// Executes near-complete test of all functions for the given graph.
fn _default_test_suite(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    warn!("Starting default test suite.");
    let _ = test_graph_properties(graph, verbose);

    warn!("Testing SkipGram / CBOW / GloVe preprocessing.");
    let _ = test_embiggen_preprocessing(graph, verbose);

    warn!("Testing subgraph generation.");
    let _ = test_subgraph_generation(graph, verbose);

    warn!("Testing clone and setters.");
    let _ = test_clone_and_setters(graph, verbose);

    warn!("Testing edge-label holdouts tests.");
    let _ = test_edgelabel_holdouts(graph, verbose);

    warn!("Testing writing out graph to file.");
    let _ = test_dump_graph(graph, verbose);

    warn!("Testing generic filtering mechanism.");
    let _ = test_graph_filter(graph, verbose);

    warn!("Testing the spanning arborescences.");
    let _ = test_spanning_arborescence_bader(graph, verbose);

    warn!("Testing the graph diameter.");
    let _ = test_graph_diameter(graph, verbose);

    warn!("Running node-label holdouts tests.");
    let _ = test_nodelabel_holdouts(graph, verbose);

    warn!("Running remove components tests.");
    let _ = test_remove_components(graph, verbose);

    warn!("Testing removes.");
    let _ = test_graph_removes(graph, verbose);

    warn!("Testing negative edges generation.");
    let _ = test_negative_edges_generation(graph, verbose);

    warn!("Executing edge holdouts tests.");
    let _ = test_edge_holdouts(graph, verbose);

    warn!("Testing k-fold holdouts.");
    let _ = test_kfold(graph, verbose);

    warn!("Testing edge lists generation.");
    let _ = test_edgelist_generation(graph, verbose);

    warn!("Testing graph remapping.");
    let _ = test_graph_remapping(graph, verbose);

    warn!("Testing random walks.");
    let _ = test_random_walks(graph, verbose);

    warn!("Testing BFS.");
    let _ = test_bfs(graph, verbose);

    warn!("Testing dijkstra.");
    let _ = test_dijkstra(graph, verbose);

    warn!("Testing approximated vertex cover");
    let _ = test_vertex_cover(graph, verbose);

    warn!("Testing node centralities.");
    let _ = test_node_centralities(graph, verbose);

    warn!("Testing polygons.");
    let _ = test_polygons(graph, verbose);

    warn!("Testing transitivity.");
    // TODO! temporarily commented out!
    // let _ = test_transitivity(graph, verbose);

    warn!("Testing all paths.");
    let _ = test_all_paths(graph, verbose);

    warn!("Testing generation of selfloops.");
    let _ = test_selfloops(graph, verbose);

    Ok(())
}

macro_rules! test_mut_graph {
    ($graph:expr, $func:ident, $verbose:expr) => {{
        println!("Testing the graph transoformation: {}", stringify!($func));
        let mut transformed_graph = $graph.$func($verbose);
        let _ = _default_test_suite(&mut transformed_graph, $verbose);
    }};
    ($graph:expr, $func:ident, $verbose:expr, result) => {{
        println!("Testing the graph transoformation: {}", stringify!($func));
        let mut transformed_graph = $graph.$func($verbose)?;
        let _ = _default_test_suite(&mut transformed_graph, $verbose);
    }};
}

/// Executes near-complete test of all functions for the given graph.
pub fn default_test_suite(graph: &mut Graph, verbose: Option<bool>) -> Result<(), String> {
    warn!("Starting default test suite.");
    let _ = _default_test_suite(graph, verbose);
    warn!("Starting default test suite with speedups enabled.");
    graph.enable(Some(true), Some(true), Some(true))?;
    let _ = _default_test_suite(graph, verbose);
    warn!("Starting default test suite on transformed graphs.");

    test_mut_graph!(graph, get_unweighted_laplacian_transformed_graph, verbose);
    test_mut_graph!(
        graph,
        get_unweighted_symmetric_normalized_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_unweighted_symmetric_normalized_laplacian_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_unweighted_random_walk_normalized_laplacian_transformed_graph,
        verbose
    );
    test_mut_graph!(
        graph,
        get_weighted_laplacian_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_weighted_symmetric_normalized_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_weighted_symmetric_normalized_laplacian_transformed_graph,
        verbose,
        result
    );
    test_mut_graph!(
        graph,
        get_weighted_random_walk_normalized_laplacian_transformed_graph,
        verbose,
        result
    );

    Ok(())
}
