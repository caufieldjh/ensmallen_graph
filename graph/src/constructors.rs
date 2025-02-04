use super::*;
use bitvec::prelude::*;
use elias_fano_rust::EliasFano;
use indicatif::ProgressIterator;
use itertools::Itertools;
use log::info;
use rayon::prelude::ParallelSliceMut;
use std::cmp::Ordering;
use std::collections::BTreeMap;

type ParsedStringEdgesType = Result<
    (
        EliasFano,
        EliasFano,
        Vocabulary<NodeT>,
        Option<EdgeTypeVocabulary>,
        Option<Vec<WeightT>>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
    ),
    String,
>;

#[macro_export]
/// Take a vector and make it a None if its empty, Some(vector) otherwise
macro_rules! optionify {
    ($val:expr) => {
        if $val.is_empty() {
            None
        } else {
            Some($val)
        }
    };
}

fn check_numeric_ids_compatibility(
    has_nodes_list: bool,
    numeric_node_ids: bool,
    numeric_edge_node_ids: bool,
) -> Result<(), String> {
    if has_nodes_list && numeric_node_ids && !numeric_edge_node_ids {
        return Err(concat!(
            "You are trying to load a numeric node list and a non numeric edge list.\n",
            "This is a problem because an edge composed of two nodes (e.g. \"2, 8\") is ",
            "not necessarily mapped internally to the same node ids of the node list.\n",
            "Possibily you want to also enable the parameter for the numeric edge node ids."
        )
        .to_string());
    }
    Ok(())
}

/// Returns iterator of nodes handling the node IDs.
///
/// # Arguments
///
/// nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
///     Iterator over the node list.
/// ignore_duplicated_nodes: bool,
///     Whether to just ignore the duplicated node types.
/// node_list_is_correct: bool,
///     Parameter to pinky promise that the node list is correct.
///     If you provide a broken node list to this method while promising
///     that the node list is correct, be prepared to deal with the fallout.
///     This parameter is mainly meant to be used internally when creating
///     graphs that CANNOT BE BROKEN by design. If you use this parameter
///     from any of the bindings, be SURE that the node list is actually
///     correct.
///     We assume that any provided node list is broken until disproved.
/// nodes: &'b mut Vocabulary<NodeT>,
///     Vocabulary of the nodes to be populated.
pub(crate) fn parse_node_ids<'a, 'b>(
    nodes_iter: impl Iterator<Item = Result<(String, Option<Vec<String>>), String>> + 'a,
    ignore_duplicated_nodes: bool,
    node_list_is_correct: bool,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a
where
    'b: 'a,
{
    nodes_iter.filter_map(move |row| {
        row.map_or_else(|err| Some(Err(err)), |(node_name, node_type)| {
            if node_list_is_correct {
                Some(Ok((nodes.unchecked_insert(node_name), node_type)))
            } else {
                if node_name.is_empty() {
                    return Some(Err("Found an empty node name. Node names cannot be empty.".to_owned()));
                }
                if nodes.contains_key(&node_name){
                    if ignore_duplicated_nodes {
                        return None;
                    }
                    return Some(Err(format!(
                        concat!(
                            "The node {node_name} appears multiple times in the node list.\n",
                            "The node type of the row is {node_type:?}.\n",
                            "The library does not currently support multiple node types for a single node."
                        ),
                        node_name = node_name,
                        node_type = node_type
                    )));
                }
                Some(nodes.insert(node_name).map(|node_id| (node_id, node_type)))
            }
        })
    })
}

/// Returns iterator of nodes handling the node type IDs.
pub(crate) fn parse_node_type_ids<'a, 'b>(
    nodes_iter: impl Iterator<Item = Result<(NodeT, Option<Vec<String>>), String>> + 'a,
    node_types_vocabulary: &'b mut NodeTypeVocabulary,
) -> impl Iterator<Item = Result<(NodeT, Option<Vec<NodeTypeT>>), String>> + 'a
where
    'b: 'a,
{
    nodes_iter.map(move |row| match row {
        Ok((node_id, node_types)) => {
            Ok((node_id, node_types_vocabulary.insert_values(node_types)?))
        }
        Err(e) => Err(e),
    })
}

pub(crate) fn parse_edges_node_ids<'a, 'b>(
    edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>> + 'a,
    edge_list_is_correct: bool,
    nodes: &'b mut Vocabulary<NodeT>,
) -> impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>> + 'a
where
    'b: 'a,
{
    let empty_nodes_mapping = nodes.is_empty();
    edges_iterator.map(move |row: Result<StringQuadruple, String>| match row {
        Ok((src_name, dst_name, edge_type, weight)) => {
            let node_ids = [src_name, dst_name]
                .iter()
                .map(|node_name| {
                    // the source and destination nodes must either be
                    //  - both numeric node ids
                    //      - if the node list was provided
                    //          - The nodes must be less than the max nodes
                    //      - if the node list was not provided
                    //          - the nodes must be added to the node list which should be numeric.
                    //  - if the edge node ids are not numeric
                    //      - if the node list was provided
                    //          - the nodes must be added to the node list.
                    //      - if the node list was no provided
                    //          - the nodes must be added to the node list.
                    if empty_nodes_mapping {
                        if edge_list_is_correct {
                            Ok(nodes.unchecked_insert(node_name.to_owned()))
                        } else {
                            nodes.insert(node_name.to_owned())
                        }
                    } else if let Some(node_id) = nodes.get(&node_name) {
                        Ok(*node_id)
                    } else {
                        Err(format!(
                            concat!(
                                "In the edge list was found the node {} ",
                                "which is not present in the given node list."
                            ),
                            node_name
                        ))
                    }
                })
                .collect::<Result<Vec<NodeT>, String>>()?;
            Ok((node_ids[0], node_ids[1], edge_type, weight))
        }
        Err(e) => Err(e),
    })
}

/// Returns iterator of edges handling the edge type IDs.
pub(crate) fn parse_edge_type_ids_vocabulary<'a, 'b>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<String>, Option<WeightT>), String>>
        + 'a,
    edge_types: &'b mut Vocabulary<EdgeTypeT>,
) -> impl Iterator<Item = Result<Quadruple, String>> + 'a
where
    'b: 'a,
{
    edges_iter.map(move |row| match row {
        Ok((src, dst, edge_type, weight)) => {
            let edge_type_id = match edge_type {
                Some(et) => Some(edge_types.insert(et)?),
                None => None,
            };
            Ok((src, dst, edge_type_id, weight))
        }
        Err(e) => Err(e),
    })
}

pub(crate) fn parse_sorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>> + 'a,
    directed: bool,
    directed_edge_list: bool,
) -> Box<dyn Iterator<Item = Result<Quadruple, String>> + 'a> {
    if directed || directed_edge_list {
        return Box::new(edges_iter);
    }
    let mut sorting_tmp: BTreeMap<Triple, Option<WeightT>> = BTreeMap::new();
    Box::new(edges_iter
        .map(Some)
        .chain(vec![None])
        .flat_map(move |maybe_row| match maybe_row {
            Some(row) => {
                let mut results: Vec<Result<Quadruple, String>> = Vec::with_capacity(1);
                let result = match row {
                    Ok((src, dst, edge_type, weight)) => {
                        if !directed && src < dst {
                            sorting_tmp.insert((dst, src, edge_type), weight);
                        }
                        while !sorting_tmp.is_empty()
                            && *sorting_tmp.first_key_value().unwrap().0 < (src, dst, edge_type)
                        {
                            let ((smaller_src, smaller_dst, smaller_edge_type), smaller_weight) =
                                sorting_tmp.pop_first().unwrap();
                            results.push(Ok((
                                smaller_src,
                                smaller_dst,
                                smaller_edge_type,
                                smaller_weight,
                            )));
                        }
                        Ok((src, dst, edge_type, weight))
                    }
                    Err(e) => Err(e),
                };
                results.push(result);
                results
            }
            None => sorting_tmp
                .iter()
                .map(|((src, dst, edge_type), weight)| Ok((*src, *dst, *edge_type, *weight)))
                .collect::<Vec<_>>(),
        }))
}

pub(crate) fn parse_unsorted_quadruples(
    mut edges: Vec<Quadruple>,
    verbose: bool,
) -> (usize, impl Iterator<Item = Result<Quadruple, String>>) {

    info!("Sorting edges.");
    edges.par_sort_by(|(src1, dst1, edt1, _), (src2, dst2, edt2, _)| {
        (*src1, *dst1, *edt1).cmp(&(*src2, *dst2, *edt2))
    });

    let edges_number = edges.len();
    let pb = get_loading_bar(verbose, "Building sorted graph", edges_number);

    (edges_number, edges.into_iter().progress_with(pb).map(Result::Ok))
}

pub(crate) fn parse_integer_unsorted_edges<'a>(
    edges_iter: impl Iterator<Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>>,
    directed: bool,
    directed_edge_list: bool,
    verbose: bool,
) -> Result<(usize, impl Iterator<Item = Result<Quadruple, String>> + 'a), String> {
    let edge_quadruples: Vec<Quadruple> = edges_iter
        .flat_map(|tuple| match tuple {
            Ok((src, dst, edt, weight)) => {
                if !directed && src != dst && !directed_edge_list {
                    vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
                } else {
                    vec![Ok((src, dst, edt, weight))]
                }
            }
            Err(e) => vec![Err(e)],
        })
        .collect::<Result<Vec<Quadruple>, String>>()?;

    Ok(parse_unsorted_quadruples(edge_quadruples, verbose))
}

pub(crate) fn parse_string_unsorted_edges<'a>(
    // This parameter does not NEED a lifetime because it does NOT survive the function call
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    mut nodes: Vocabulary<NodeT>,
    directed: bool,
    directed_edge_list: bool,
    edge_list_is_correct: bool,
    has_edge_types: bool,
    verbose: bool,
    numeric_edge_type_ids: bool,
) -> Result<
    (
        usize,
        impl Iterator<Item = Result<Quadruple, String>> + 'a,
        Vocabulary<NodeT>,
        Option<Vocabulary<EdgeTypeT>>,
    ),
    String,
> {
    let mut edge_types_vocabulary = if has_edge_types {
        Some(Vocabulary::default().set_numeric_ids(numeric_edge_type_ids))
    } else {
        None
    };
    let (edges_number, edges_iter) = {
        let edges_iter = parse_edges_node_ids(edges_iter, edge_list_is_correct, &mut nodes);
        let edges_iter: Box<dyn Iterator<Item = Result<Quadruple, String>>> =
            if let Some(ets) = &mut edge_types_vocabulary {
                Box::new(parse_edge_type_ids_vocabulary(edges_iter, ets))
            } else {
                Box::new(edges_iter.map_ok(|(src, dst, _, weight)| (src, dst, None, weight)))
            };
        let edge_quadruples: Vec<Quadruple> = edges_iter
            .flat_map(|tuple| match tuple {
                Ok((src, dst, edt, weight)) => {
                    if !directed && src != dst && !directed_edge_list {
                        vec![Ok((src, dst, edt, weight)), Ok((dst, src, edt, weight))]
                    } else {
                        vec![Ok((src, dst, edt, weight))]
                    }
                }
                Err(e) => vec![Err(e)],
            })
            .collect::<Result<Vec<Quadruple>, String>>()?;

        parse_unsorted_quadruples(edge_quadruples, verbose)
    };
    info!("Building nodes reverse mapping.");
    nodes.build_reverse_mapping()?;
    if let Some(ets) = &mut edge_types_vocabulary {
        info!("Building edge types reverse mapping.");
        ets.build_reverse_mapping()?;
    }
    Ok((edges_number, edges_iter, nodes, edge_types_vocabulary))
}

pub(crate) fn build_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    ignore_duplicated_edges: bool,
    has_weights: bool,
    has_edge_types: bool,
    directed: bool,
    edge_list_is_correct: bool,
) -> Result<
    (
        EliasFano,
        EliasFano,
        Option<Vec<Option<EdgeTypeT>>>,
        Option<Vec<WeightT>>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u8,
        u64,
    ),
    String,
> {
    info!("Started building of EliasFano edges data structure.");
    let node_bits = get_node_bits(nodes_number);
    let node_bit_mask = (1 << node_bits) - 1;
    let mut edges: EliasFano =
        EliasFano::new(encode_max_edge(nodes_number, node_bits), edges_number)?;

    let mut edge_type_ids: Option<Vec<Option<EdgeTypeT>>> = if has_edge_types {
        Some(Vec::with_capacity(edges_number))
    } else {
        None
    };

    let mut weights: Option<Vec<WeightT>> = if has_weights {
        Some(Vec::with_capacity(edges_number))
    } else {
        None
    };

    // TODO: the following data structure could be better to be a bitvector.
    // This is because universe == number of elements
    let mut unique_sources: EliasFano = EliasFano::new(nodes_number as u64, nodes_number as usize)?;
    // Last source inserted
    let mut last_src: NodeT = 0;
    let mut last_dst: NodeT = 0;
    let mut last_edge_type: Option<EdgeTypeT> = None;
    let mut unique_edges_number: EdgeT = 0;
    let mut unique_self_loop_number: NodeT = 0;
    let mut self_loop_number: EdgeT = 0;
    let mut forward_undirected_edges_counter: EdgeT = 0;
    let mut backward_undirected_edges_counter: EdgeT = 0;
    let mut nodes_with_edges = bitvec![Msb0, u8; 0; nodes_number as usize];
    let mut not_singleton_node_number: NodeT = 0;
    let mut singleton_nodes_with_self_loops = bitvec![Msb0, u8; 0; nodes_number as usize];
    let mut singleton_nodes_with_self_loops_number: NodeT = 0;
    let mut first = true;

    for value in edges_iter {
        let (src, dst, edge_type, weight) = value?;
        let different_src = last_src != src || first;
        let different_dst = last_dst != dst || first;
        let self_loop = src == dst;
        let different_edge_type = last_edge_type != edge_type || first;
        if !(different_src || different_dst || different_edge_type) {
            if ignore_duplicated_edges {
                continue;
            } else {
                return Err("A duplicated edge was found while building the graph.".to_owned());
            }
        }

        if let Some(ets) = &mut edge_type_ids {
            ets.push(edge_type);
        }
        match (&mut weights, weight) {
            (Some(ws), Some(w)) => {
                validate_weight(w)?;
                ws.push(w);
                Ok(())
            }
            (None, Some(_)) => Err(concat!(
                "A non-None weight was provided but no weights are expected ",
                "because the has_weights flag has been set to false."
            )),
            (Some(_), None) => Err(concat!(
                "A None weight was found.\n",
                "This might mean you have either provided a None weight to the edge list or ",
                "you may have an empty weight in your edge list file.\n",
                "If you intend to load this edge list WITHOUT weights, do not provide the ",
                "edge weights colum or column number.\n",
                "If you intend to load this edge with its weight, add a default weight."
            )),
            _ => Ok(()),
        }?;

        if !directed && !edge_list_is_correct {
            match src.cmp(&dst) {
                Ordering::Greater => {
                    // We retrieve the edge id of the forward edge, the one going from
                    // dst to src.
                    let maybe_edge_id = edges.rank(encode_edge(dst, src, node_bits));
                    // Now we need to find, starting from edge id, if the edge types are given,
                    // the correct edge id: if we are in a multi-graph the edge may be the same
                    // but have multiple edge types and hence be reported multiple times.
                    let maybe_edge_id = maybe_edge_id.and_then(|min_edge_id| {
                        edge_type_ids.as_ref().map_or(Some(min_edge_id), |ets| {
                            (min_edge_id
                                ..edges.unchecked_rank(encode_edge(dst, src + 1, node_bits)))
                                .find(|edge_id| ets[*edge_id as usize] == edge_type)
                        })
                    });
                    // Finally now we need to check if the weights of the two edges, if given
                    // are actually equal.
                    let has_unbalanced_undirected_edge = maybe_edge_id.map_or(true, |edge_id| {
                        weights.as_ref().map_or(false, |ws| {
                            (ws[edge_id as usize] - weight.unwrap()).abs() >= f32::EPSILON
                        })
                    });
                    if has_unbalanced_undirected_edge {
                        return Err(concat!(
                            "You are trying to load an undirected ",
                            "graph using the directed edge list ",
                            "paremeter that requires for ALL edges to ",
                            "be fully defined in both directions.\n",
                            "The edge list you have provided does not ",
                            "provide the edges in both directions.",
                        )
                        .to_string());
                    }
                    backward_undirected_edges_counter += 1
                }
                Ordering::Less => forward_undirected_edges_counter += 1,
                Ordering::Equal => {}
            }
        }
        last_edge_type = edge_type;
        edges.unchecked_push(encode_edge(src, dst, node_bits));
        if self_loop {
            self_loop_number += 1;
        }
        if different_src || different_dst {
            for node in &[src, dst] {
                if !nodes_with_edges[*node as usize] {
                    nodes_with_edges.set(*node as usize, true);
                    if !self_loop {
                        not_singleton_node_number += 1;
                    } else {
                        singleton_nodes_with_self_loops.set(*node as usize, true);
                        singleton_nodes_with_self_loops_number += 1;
                    }
                } else if !self_loop && singleton_nodes_with_self_loops[*node as usize] {
                    singleton_nodes_with_self_loops.set(*node as usize, false);
                    singleton_nodes_with_self_loops_number -= 1;
                    not_singleton_node_number += 1;
                }
            }
            unique_edges_number += 1;
            if self_loop {
                unique_self_loop_number += 1;
            }
            if different_src {
                unique_sources.unchecked_push(src as u64);
                last_src = src;
            }
            if different_dst {
                last_dst = dst;
            }
        }
        if first {
            first = false;
        }
    }

    if forward_undirected_edges_counter != backward_undirected_edges_counter {
        return Err(concat!(
            "You are trying to load an undirected graph ",
            "from a directed edge list but the edge list is not ",
            "complete."
        )
        .to_owned());
    }

    if let Some(ws) = &weights {
        if edges.len() != ws.len() {
            panic!(
                "The number of weights {} does not match the number of edges {}.",
                ws.len(),
                edges.len()
            );
        }
    }

    if let Some(ets) = &edge_type_ids {
        if edges.len() != ets.len() {
            panic!(
                "The number of edge types {} does not match the number of edges {}.",
                ets.len(),
                edges.len()
            );
        }
    }

    Ok((
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_node_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
    ))
}

fn parse_nodes(
    nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
    ignore_duplicated_nodes: bool,
    node_list_is_correct: bool,
    numeric_node_ids: bool,
    numeric_node_types_ids: bool,
    numeric_edge_node_ids: bool,
    has_node_types: bool,
) -> Result<(Vocabulary<NodeT>, Option<NodeTypeVocabulary>), String> {
    let mut nodes = Vocabulary::default()
        .set_numeric_ids(numeric_node_ids || numeric_edge_node_ids && nodes_iterator.is_none());

    let node_types = if let Some(ni) = nodes_iterator {
        // TODO: the following can likely be dealt with in a better way.
        let node_iterator = parse_node_ids(
            ni,
            ignore_duplicated_nodes,
            node_list_is_correct,
            &mut nodes,
        );
        // In the case there is a node types we need to add its proper iterator.
        if has_node_types {
            let mut node_types =
                NodeTypeVocabulary::default().set_numeric_ids(numeric_node_types_ids);
            for row in parse_node_type_ids(node_iterator, &mut node_types) {
                row?;
            }
            node_types.build_reverse_mapping()?;
            node_types.build_counts();
            Ok::<_, String>(Some(node_types))
        } else {
            for row in node_iterator {
                row?;
            }
            Ok::<_, String>(None)
        }?
    } else {
        None
    };

    Ok((nodes, node_types))
}

pub(crate) fn parse_string_edges(
    edges_iter: impl Iterator<Item = Result<StringQuadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    directed: bool,
    mut nodes: Vocabulary<NodeT>,
    numeric_edge_type_ids: bool,
    directed_edge_list: bool,
    edge_list_is_correct: bool,
    ignore_duplicated_edges: bool,
    has_edge_types: bool,
    has_weights: bool,
) -> ParsedStringEdgesType {
    let mut edge_types_vocabulary: Vocabulary<EdgeTypeT> =
        Vocabulary::default().set_numeric_ids(numeric_edge_type_ids);

    let edges_iter = parse_sorted_edges(
        parse_edge_type_ids_vocabulary(
            parse_edges_node_ids(edges_iter, edge_list_is_correct, &mut nodes),
            &mut edge_types_vocabulary,
        ),
        directed,
        directed_edge_list,
    );

    let (
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        has_weights,
        has_edge_types,
        directed,
        edge_list_is_correct,
    )?;

    nodes.build_reverse_mapping()?;
    edge_types_vocabulary.build_reverse_mapping()?;
    let edge_types =
        EdgeTypeVocabulary::from_option_structs(edge_type_ids, optionify!(edge_types_vocabulary));

    Ok((
        edges,
        unique_sources,
        nodes,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bit_mask,
        node_bits,
    ))
}

pub(crate) fn parse_integer_edges(
    edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
    edges_number: usize,
    nodes_number: NodeT,
    edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
    ignore_duplicated_edges: bool,
    directed: bool,
    edge_list_is_correct: bool,
    has_edge_types: bool,
    has_weights: bool,
) -> Result<
    (
        EliasFano,
        EliasFano,
        Option<EdgeTypeVocabulary>,
        Option<Vec<WeightT>>,
        EdgeT,
        EdgeT,
        NodeT,
        NodeT,
        NodeT,
        u64,
        u8,
    ),
    String,
> {
    let (
        edges,
        unique_sources,
        edge_type_ids,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bits,
        node_bit_mask,
    ) = build_edges(
        edges_iter,
        edges_number,
        nodes_number,
        ignore_duplicated_edges,
        has_weights,
        has_edge_types,
        directed,
        edge_list_is_correct,
    )?;

    let edge_types = EdgeTypeVocabulary::from_option_structs(edge_type_ids, edge_types_vocabulary);

    Ok((
        edges,
        unique_sources,
        edge_types,
        weights,
        unique_edges_number,
        self_loop_number,
        unique_self_loop_number,
        not_singleton_nodes_number,
        singleton_nodes_with_self_loops_number,
        node_bit_mask,
        node_bits,
    ))
}

/// # Graph Constructors
impl Graph {
    pub(crate) fn build_graph<S: Into<String>>(
        edges_iter: impl Iterator<Item = Result<Quadruple, String>>,
        edges_number: usize,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        edge_list_is_correct: bool,
        name: S,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_weights: bool,
    ) -> Result<Graph, String> {
        let (
            edges,
            unique_sources,
            edge_types,
            weights,
            unique_edges_number,
            self_loop_number,
            unique_self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            node_bit_mask,
            node_bits,
        ) = parse_integer_edges(
            edges_iter,
            edges_number,
            nodes.len() as NodeT,
            edge_types_vocabulary,
            ignore_duplicated_edges,
            directed,
            edge_list_is_correct,
            has_edge_types,
            has_weights,
        )?;

        Ok(Graph::new(
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights,
            node_types,
        ))
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    ///
    /// TODO: UPDATE THE DOCSTRING!
    ///
    /// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
    ///     Iterator of the edges.
    /// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ///     Iterator of the nodes.
    /// * directed: bool,
    ///     Wether the graph should be directed or undirected.
    /// * ignore_duplicated_nodes: bool,
    ///     Wether to ignore duplicated nodes or to raise a proper exception.
    /// * ignore_duplicated_edges: bool,
    ///     Wether to ignore duplicated edges or to raise a proper exception.
    /// * skip_self_loops: bool,
    ///     Wether to skip self loops while reading the the edges iterator.
    pub fn from_string_unsorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        name: S,
        ignore_duplicated_nodes: bool,
        node_list_is_correct: bool,
        ignore_duplicated_edges: bool,
        edge_list_is_correct: bool,
        verbose: bool,
        numeric_edge_type_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_weights: bool,
    ) -> Result<Graph, String> {
        check_numeric_ids_compatibility(
            nodes_iterator.is_some(),
            numeric_node_ids,
            numeric_edge_node_ids,
        )?;
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            node_list_is_correct,
            numeric_node_ids,
            numeric_node_types_ids,
            numeric_edge_node_ids,
            has_node_types,
        )?;

        info!("Parse unsorted edges.");
        // TODO: ADD USE OF edge_list_is_correct
        let (edges_number, edges_iterator, nodes, edge_types_vocabulary) =
            parse_string_unsorted_edges(
                edges_iterator,
                nodes,
                directed,
                directed_edge_list,
                edge_list_is_correct,
                has_edge_types,
                verbose,
                numeric_edge_type_ids,
            )?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            edge_list_is_correct || !directed_edge_list,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights,
        )
    }

    /// Create new Graph object from unsorted source.
    ///
    /// # Arguments
    ///
    /// * edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
    ///     Iterator of the edges.
    /// * nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<String>), String>>>,
    ///     Iterator of the nodes.
    /// * directed: bool,
    ///     Wether the graph should be directed or undirected.
    /// * ignore_duplicated_nodes: bool,
    ///     Wether to ignore duplicated nodes or to raise a proper exception.
    /// * ignore_duplicated_edges: bool,
    ///     Wether to ignore duplicated edges or to raise a proper exception.
    /// * skip_self_loops: bool,
    ///     Wether to skip self loops while reading the the edges iterator.
    pub fn from_integer_unsorted(
        edges_iterator: impl Iterator<
            Item = Result<(NodeT, NodeT, Option<NodeTypeT>, Option<WeightT>), String>,
        >,
        nodes: Vocabulary<NodeT>,
        node_types: Option<NodeTypeVocabulary>,
        edge_types_vocabulary: Option<Vocabulary<EdgeTypeT>>,
        directed: bool,
        name: String,
        ignore_duplicated_edges: bool,
        has_edge_types: bool,
        has_weights: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        let (edges_number, edges_iterator) =
            parse_integer_unsorted_edges(edges_iterator, directed, true, verbose)?;

        Graph::build_graph(
            edges_iterator,
            edges_number,
            nodes,
            node_types,
            edge_types_vocabulary,
            directed,
            true,
            name,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights,
        )
    }

    /// Create new Graph object from sorted sources.
    pub fn from_string_sorted<S: Into<String>>(
        edges_iterator: impl Iterator<Item = Result<StringQuadruple, String>>,
        nodes_iterator: Option<impl Iterator<Item = Result<(String, Option<Vec<String>>), String>>>,
        directed: bool,
        directed_edge_list: bool,
        ignore_duplicated_nodes: bool,
        node_list_is_correct: bool,
        ignore_duplicated_edges: bool,
        edge_list_is_correct: bool,
        edges_number: usize,
        nodes_number: NodeT,
        numeric_edge_type_ids: bool,
        numeric_node_ids: bool,
        numeric_edge_node_ids: bool,
        numeric_node_types_ids: bool,
        has_node_types: bool,
        has_edge_types: bool,
        has_weights: bool,
        name: S,
    ) -> Result<Graph, String> {
        check_numeric_ids_compatibility(
            nodes_iterator.is_some(),
            numeric_node_ids,
            numeric_edge_node_ids,
        )?;
        let (nodes, node_types) = parse_nodes(
            nodes_iterator,
            ignore_duplicated_nodes,
            node_list_is_correct,
            numeric_node_ids,
            numeric_node_types_ids,
            numeric_edge_node_ids,
            has_node_types,
        )?;

        let (
            edges,
            unique_sources,
            nodes,
            edge_types,
            weights,
            unique_edges_number,
            self_loop_number,
            unique_self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            node_bit_mask,
            node_bits,
        ) = parse_string_edges(
            edges_iterator,
            edges_number,
            nodes_number,
            directed,
            nodes,
            numeric_edge_type_ids,
            directed_edge_list,
            edge_list_is_correct,
            ignore_duplicated_edges,
            has_edge_types,
            has_weights,
        )?;

        Ok(Graph::new(
            directed,
            unique_self_loop_number,
            self_loop_number,
            not_singleton_nodes_number,
            singleton_nodes_with_self_loops_number,
            unique_edges_number,
            edges,
            unique_sources,
            nodes,
            node_bit_mask,
            node_bits,
            edge_types,
            name,
            weights,
            node_types,
        ))
    }
}
