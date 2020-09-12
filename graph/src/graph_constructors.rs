use super::*;
use std::collections::{BTreeMap, HashMap, HashSet};

macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

/// Read node file and returns graph builder data structures.
///
/// Specifically, the returned objects are:
/// * nodes_mapping: an hashmap from the node name to the node id.
/// * node_reverse_mapping: vector of node names.
/// * node_types_mapping: an hashmap from node types names to the node type ids.
/// * node_types_reverse_mapping: vector of the node types names.
/// * node_types: vector of the numeric node types ids.
pub(crate) fn parse_nodes(
    nodes_iter: impl Iterator<Item = Result<(String, Option<String>), String>>,
    ignore_duplicated_nodes: bool,
) -> Result<(Vocabulary<NodeT>, Option<VocabularyVec<NodeTypeT>>), String> {
    let mut nodes: Vocabulary<NodeT> = Vocabulary::new();
    let mut node_types: VocabularyVec<NodeTypeT> = VocabularyVec::new();

    for values in nodes_iter {
        let (node_name, node_type) = values?;
        // if the node is already mapped => duplicated line
        if nodes.contains_key(&node_name) {
            if ignore_duplicated_nodes {
                continue;
            }
            return Err(format!(
                concat!(
                    "\nFound duplicated nodes!\n",
                    "The node is {node_name}.\n",
                    "The node type of the row is {node_type:?}.\n",
                    "The library does not currently support multiple node types for a single node."
                ),
                node_name = node_name,
                node_type = node_type
            ));
        }
        nodes.insert(node_name);
        if let Some(ndt) = node_type {
            node_types.insert(ndt);
        }
    }

    Ok((nodes, optionify!(node_types)))
}

/// Read edge file and returns graph builder data structures.
///
pub(crate) fn parse_edges(
    nodes: &mut Vocabulary<NodeT>,
    directed: bool,
    edges_iterator: impl Iterator<
        Item = Result<(String, String, Option<String>, Option<WeightT>), String>,
    >,
    skip_self_loops: bool,
    ignore_duplicated_edges: bool,
) -> Result<
    (
        BTreeMap<(NodeT, NodeT), ConstructorEdgeMetadata>,
        Option<Vocabulary<EdgeTypeT>>,
    ),
    String,
> {
    // save if the node file was loaded or not
    let empty_nodes_mapping: bool = nodes.is_empty();
    // edges mappings
    let mut edge_types_vocabulary: Vocabulary<NodeTypeT> = Vocabulary::new();
    // helper structure
    let mut unique_edges_tree: BTreeMap<(NodeT, NodeT), ConstructorEdgeMetadata> = BTreeMap::new();

    for values in edges_iterator {
        let (source_node_name, destination_node_name, edge_type, edge_weight) = values?;
        // Check if we need to skip self-loops
        if skip_self_loops && source_node_name == destination_node_name {
            // If current edge is a self-loop and we need to skip them we skip.
            continue;
        }
        // Handle missing node IDs when no node file was provided
        for node_name in &[source_node_name, destination_node_name] {
            if !nodes.contains_key(node_name) {
                if empty_nodes_mapping {
                    nodes.insert(node_name.clone());
                } else {
                    return Err(format!(
                        concat!(
                            "In the edge file was found the node {} ",
                            "which is not present in the given node file."
                        ),
                        node_name
                    ));
                }
            }
        }
        // Retrieve the node IDs
        let source_node_id = nodes.get(&source_node_name).unwrap();
        let destinations_node_id = nodes.get(&destination_node_name).unwrap();
        // Retrieve the edge type id if it was given.
        let edge_types_id = if let Some(et) = edge_type {
            Some(edge_types_vocabulary.insert(et))
        } else {
            None
        };

        // Get the metadata of the edge and if it's not present, add it
        let mut edge_metadata = unique_edges_tree
            .entry((*source_node_id, *destinations_node_id))
            .or_insert_with(ConstructorEdgeMetadata::new);

        // if the node is already mapped => duplicated line
        if let Some(eti) = edge_types_id {
            if edge_metadata.edge_types.contains(&eti) {
                if ignore_duplicated_edges {
                    continue;
                }
                return Err(format!(
                    concat!(
                        "\nFound duplicated edges!\n",
                        "The source node is {source} and the destination node is {destination}.\n",
                        "The edge type of the row is {edge_type:?}.",
                    ),
                    source = source_node_name,
                    destination = destination_node_name,
                    edge_type = edge_type,
                ));
            }
            // add the edge type in the metadata
            edge_metadata.edge_types.push(eti);
        }
        // add the weight is present
        if let Some(w) = edge_weight {
            edge_metadata.weights.push(w);
        }

        // If the graph is undirected, add the inverse edge
        //
        if !directed {
            let mut edge_metadata = unique_edges_tree
                .entry((*destinations_node_id, *source_node_id))
                .or_insert_with(ConstructorEdgeMetadata::new);

            if let Some(et) = edge_types_id {
                edge_metadata.edge_types.push(et);
            }
            if let Some(w) = edge_weight {
                edge_metadata.weights.push(w);
            }
        }
    }

    Ok((unique_edges_tree, optionify!(edge_types_vocabulary)))
}

pub(crate) fn build_graph(
    unique_edges_tree: BTreeMap<(NodeT, NodeT), ConstructorEdgeMetadata>,
    nodes: Vocabulary<NodeT>,
    node_types: Option<VocabularyVec<NodeTypeT>>,
    edge_types: Option<Vocabulary<EdgeTypeT>>,
    directed: bool,
) -> Graph {
    // structures to fill for the graph
    let mut outbounds: Vec<EdgeT> = Vec::new();
    let mut sources: Vec<NodeT> = Vec::new();
    let mut not_trap_nodes: Vec<NodeT> = Vec::new();
    let mut destinations: Vec<NodeT> = Vec::new();
    let mut weights: Vec<WeightT> = Vec::new();
    let mut unique_edges: HashMap<(NodeT, NodeT), EdgeMetadata> = HashMap::new();
    let mut edge_types_vector: Vec<NodeTypeT> = Vec::new();

    // now that the tree is built
    // we can iter on the edge in order (no further sorting required)
    // during the iteration we pop the minimum value each time
    let mut last_src = 0;
    let mut i = 0;
    while !unique_edges_tree.is_empty() {
        // we gradually destroy the tree while we fill the other structures
        // in this way we reduce the memory peak
        // the unwrap is guaranteed to succeed because we check if the tree is empty
        let ((src, dst), metadata) = unique_edges_tree.pop_first().unwrap();

        // fill the outbounds vector
        // this is a vector that have the offset of the last
        // edge of each src
        if last_src != src {
            // Assigning to range instead of single value, so that traps
            // have as delta between previous and next node zero.
            for o in &mut outbounds[last_src..src] {
                *o = i;
            }
            not_trap_nodes.push(last_src as NodeT);
            last_src = src;
        }

        // initalize the hashmap
        unique_edges.insert(
            (src, dst),
            EdgeMetadata {
                edge_id: sources.len(),
                edge_types: metadata
                    .edge_types
                    .into_iter()
                    .collect::<HashSet<EdgeTypeT>>(),
            },
        );

        // initialize the vectors
        if metadata.edge_types.is_empty() {
            // if there are no edge types
            // its not a multigraph and therefore we have
            // only one edge with optionally a weight.
            sources.push(src);
            destinations.push(dst);
            if !metadata.weights.is_empty() {
                weights.push(metadata.weights[0]);
            }
        } else {
            // else we are in a multigraph and we must initialize
            // all the edges
            for edt in metadata.edge_types {
                sources.push(src);
                destinations.push(dst);
                edge_types_vector.push(edt)
            }
            // If there are some weights, they should
            // be equal in number to the edge_types
            for w in metadata.weights {
                weights.push(w);
            }
        }
        i += 1;
    }

    Graph {
        not_trap_nodes,
        sources,
        destinations,
        nodes,
        unique_edges,
        outbounds,
        node_types,
        is_directed: directed,
        has_traps: not_trap_nodes.len() != outbounds.len(),
        weights: optionify!(weights),
        edge_types: match edge_types {
            Some(et) => Some(VocabularyVec::<EdgeTypeT>{
                vocabulary:et,
                ids:edge_types_vector
            }),
            None => None
        }
    }
}

/// # Graph Constructors
impl Graph {
    pub fn new(
        edges_iterator: impl Iterator<
            Item = Result<(String, String, Option<String>, Option<WeightT>), String>,
        >,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
        directed: bool,
        ignore_duplicated_nodes: bool,
        ignore_duplicated_edges: bool,
        skip_self_loops: bool,
    ) -> Result<Graph, String> {
        let (mut nodes, node_types) = if let Some(ni) = nodes_iterator {
            parse_nodes(ni, ignore_duplicated_nodes)?
        } else {
            (Vocabulary::new(), None)
        };

        let (unique_edges_tree, edge_types_vocabulary) = parse_edges(
            &mut nodes,
            directed,
            edges_iterator,
            skip_self_loops,
            ignore_duplicated_edges,
        )?;

        Ok(build_graph(
            unique_edges_tree,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
        ))
    }
}
