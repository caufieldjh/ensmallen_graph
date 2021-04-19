use super::*;
use indicatif::ProgressIterator;

impl Graph {
    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// # Arguments
    /// * node_ids_to_keep: Option<Vec<NodeT>> - List of node IDs to keep during filtering.
    /// * node_ids_to_filter: Option<Vec<NodeT>> - List of node IDs to remove during filtering.
    /// * node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>> - List of node type IDs to keep during filtering. The node types must match entirely the given node types vector provided.
    /// * node_type_ids_to_filter: Option<Vec<Option<Vec<NodeTypeT>>>> - List of node type IDs to remove during filtering. The node types must match entirely the given node types vector provided.
    /// * node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>> - List of node type IDs to keep during filtering. Any of node types must match with one of the node types given.
    /// * node_type_id_to_filter: Option<Vec<Option<NodeTypeT>>> - List of node type IDs to remove during filtering. Any of node types must match with one of the node types given.
    /// * edge_ids_to_keep: Option<Vec<EdgeT>> - List of edge IDs to keep during filtering.
    /// * edge_ids_to_filter: Option<Vec<EdgeT>> - List of edge IDs to remove during filtering.
    /// * edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>> - List of tuple of node IDs to keep during filtering.
    /// * edge_node_ids_to_filter: Option<Vec<(NodeT, NodeT)>> - List of tuple of node IDs to remove during filtering.
    /// * edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>> - List of edge type IDs to keep during filtering.
    /// * edge_type_ids_to_filter: Option<Vec<Option<EdgeTypeT>>> - List of edge type IDs to remove during filtering.
    /// * min_edge_weight: Option<WeightT> - Minimum edge weight. Values lower than this are removed.
    /// * max_edge_weight: Option<WeightT> - Maximum edge weight. Values higher than this are removed.
    /// * filter_singletons: bool - Whether to filter out singletons.
    /// * filter_selfloops: bool - Whether to filter out selfloops.
    /// * verbose: bool - Whether to show loading bar while building the graphs.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when removing edges
    /// Some of the remove operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    pub fn filter_from_ids(
        &self,
        node_ids_to_keep: Option<Vec<NodeT>>,
        node_ids_to_filter: Option<Vec<NodeT>>,
        node_type_ids_to_keep: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_ids_to_filter: Option<Vec<Option<Vec<NodeTypeT>>>>,
        node_type_id_to_keep: Option<Vec<Option<NodeTypeT>>>,
        node_type_id_to_filter: Option<Vec<Option<NodeTypeT>>>,
        edge_ids_to_keep: Option<Vec<EdgeT>>,
        edge_ids_to_filter: Option<Vec<EdgeT>>,
        edge_node_ids_to_keep: Option<Vec<(NodeT, NodeT)>>,
        edge_node_ids_to_filter: Option<Vec<(NodeT, NodeT)>>,
        edge_type_ids_to_keep: Option<Vec<Option<EdgeTypeT>>>,
        edge_type_ids_to_filter: Option<Vec<Option<EdgeTypeT>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singletons: bool,
        filter_selfloops: bool,
        verbose: bool,
    ) -> Graph {
        let pb_edges = get_loading_bar(
            verbose,
            format!(
                "Building edges of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_directed_edges_number() as usize,
        );

        let pb_nodes = get_loading_bar(
            verbose,
            format!(
                "Building nodes of graph {} without required attributes",
                self.name
            )
            .as_ref(),
            self.get_nodes_number() as usize,
        );

        let has_node_filters = self.has_nodes()
            && [
                node_ids_to_keep.is_some(),
                node_ids_to_filter.is_some(),
                node_type_ids_to_keep.is_some(),
                node_type_ids_to_filter.is_some(),
                node_type_id_to_keep.is_some(),
                node_type_id_to_filter.is_some(),
                filter_singletons && self.has_singletons(),
            ]
            .iter()
            .any(|value| *value);

        let has_edge_filters = self.has_edges()
            && [
                edge_ids_to_keep.is_some(),
                edge_ids_to_filter.is_some(),
                edge_node_ids_to_keep.is_some(),
                edge_node_ids_to_filter.is_some(),
                edge_type_ids_to_keep.is_some(),
                edge_type_ids_to_filter.is_some(),
                min_edge_weight.is_some() && max_edge_weight.is_some() && self.has_edge_weights(),
                filter_selfloops && self.has_selfloops(),
            ]
            .iter()
            .any(|value| *value);

        let min_edge_weight = min_edge_weight.unwrap_or(WeightT::NEG_INFINITY);
        let max_edge_weight = max_edge_weight.unwrap_or(WeightT::INFINITY);

        let edge_filter = |(edge_id, src, dst, edge_type_id, weight): &(
            EdgeT,
            NodeT,
            NodeT,
            Option<EdgeTypeT>,
            Option<WeightT>,
        )| {
            edge_ids_to_keep.as_ref().map_or(true, |edge_ids| edge_ids.contains(edge_id)) &&
            edge_ids_to_filter.as_ref().map_or(true, |edge_ids| !edge_ids.contains(edge_id)) &&
            // If selfloops need to be filtered out.
            (!filter_selfloops || src != dst) &&
            // If the allow edge types set was provided
            edge_node_ids_to_keep.as_ref().map_or(true, |edge_node_ids| edge_node_ids.contains(&(*src, *dst))) &&
            // If the deny edge types set was provided
            edge_node_ids_to_filter.as_ref().map_or(true, |edge_node_ids| !edge_node_ids.contains(&(*src, *dst))) &&
            edge_type_ids_to_keep.as_ref().map_or(true, |ntitk| ntitk.contains(edge_type_id)) &&
            edge_type_ids_to_filter.as_ref().map_or(true, |ntitf| !ntitf.contains(edge_type_id)) &&
            weight.map_or(true, |weight| weight >= min_edge_weight && weight <= max_edge_weight)
        };

        let node_filter = |(node_id, _, node_type_ids, _): &(
            NodeT,
            String,
            Option<Vec<NodeTypeT>>,
            Option<Vec<String>>,
        )| {
            node_ids_to_keep
                .as_ref()
                .map_or(true, |nitk| nitk.contains(node_id))
                && node_ids_to_filter
                    .as_ref()
                    .map_or(true, |nitf| !nitf.contains(node_id))
                && node_type_ids_to_keep
                    .as_ref()
                    .map_or(true, |ntitk| ntitk.contains(node_type_ids))
                && node_type_ids_to_filter
                    .as_ref()
                    .map_or(true, |ntitf| !ntitf.contains(node_type_ids))
                && node_type_id_to_keep
                    .as_ref()
                    .map_or(true, |ntitk| match node_type_ids {
                        Some(node_type_ids) => node_type_ids
                            .iter()
                            .all(|node_type_id| ntitk.contains(&Some(*node_type_id))),
                        None => ntitk.contains(&None),
                    })
                && node_type_id_to_filter
                    .as_ref()
                    .map_or(true, |ntitf| match node_type_ids {
                        Some(node_type_ids) => !node_type_ids
                            .iter()
                            .all(|node_type_id| node_type_ids.contains(node_type_id)),
                        None => !ntitf.contains(&None),
                    })
                && !(filter_singletons && self.is_unchecked_singleton_from_node_id(*node_id))
                && !(filter_singletons
                    && filter_selfloops
                    && self.is_singleton_with_selfloops_from_node_id(*node_id))
        };

        match (has_node_filters, has_edge_filters) {
            (false, false) => Ok(self.clone()),
            (false, true) => Graph::build_graph(
                self.iter_edge_node_ids_and_edge_type_id_and_edge_weight(true)
                    .progress_with(pb_edges)
                    .filter(edge_filter)
                    .map(|(_, src, dst, edge_type, weight)| Ok((src, dst, edge_type, weight))),
                self.get_directed_edges_number() as usize,
                self.nodes.clone(),
                self.node_types.clone(),
                self.edge_types.as_ref().map(|ets| ets.vocabulary.clone()),
                self.directed,
                true,
                self.get_name(),
                false,
                self.has_edge_types(),
                self.has_edge_weights(),
                true,
                self.has_singletons_with_selfloops() && !filter_selfloops,
                true,
            ),
            (true, _) => {
                Graph::from_string_sorted(
                    self.iter_edge_node_names_and_edge_type_name_and_edge_weight(true)
                        .progress_with(pb_edges)
                        .filter(
                            |(edge_id, src, src_name, dst, dst_name, edge_type, _, weight)| {
                                edge_filter(&(*edge_id, *src, *dst, *edge_type, *weight))
                                    && node_filter(&(
                                        *src,
                                        src_name.clone(),
                                        self.get_unchecked_node_type_id_from_node_id(*src),
                                        None,
                                    ))
                                    && node_filter(&(
                                        *dst,
                                        dst_name.clone(),
                                        self.get_unchecked_node_type_id_from_node_id(*dst),
                                        None,
                                    ))
                            },
                        )
                        .map(|(_, _, src_name, _, dst_name, _, edge_type_name, weight)| {
                            Ok((src_name, dst_name, edge_type_name, weight))
                        }),
                    Some(
                        self.iter_nodes()
                            .progress_with(pb_nodes)
                            .filter(node_filter)
                            .map(|(_, node_name, _, node_types)| Ok((node_name, node_types))),
                    ),
                    self.is_directed(),
                    true,
                    false,
                    true,
                    false,
                    true,
                    self.get_directed_edges_number() as usize,
                    self.get_nodes_number(),
                    // TODO: UPDATE THE FOLLOWING FOUR BOOLEANS
                    false,
                    false,
                    false,
                    false,
                    self.has_node_types(),
                    self.has_edge_types(),
                    self.has_edge_weights(),
                    // TODO: Almost any edge filtering procedure may produce singletons.
                    // Consider refining the following for the subset that do not
                    // which should basically be only those that remove singletons.
                    true,
                    self.has_selfloops() && !filter_selfloops,
                    true,
                    self.get_name(),
                )
            }
        }
        .unwrap()
    }

    /// Returns a **NEW** Graph that does not have the required attributes.
    ///
    /// # Arguments
    /// * node_ids_to_keep: Option<Vec<&str>> - List of node names to keep during filtering.
    /// * node_names_to_filter: Option<Vec<&str>> - List of node names to remove during filtering.
    /// * node_type_names_to_keep: Option<Vec<Option<Vec<String>>>> - List of node type names to keep during filtering. The node types must match entirely the given node types vector provided.
    /// * node_type_names_to_filter: Option<Vec<Option<Vec<String>>>> - List of node type names to remove during filtering. The node types must match entirely the given node types vector provided.
    /// * node_type_name_to_keep: Option<Vec<Option<String>>> - List of node type name to keep during filtering. Any of node types must match with one of the node types given.
    /// * node_type_name_to_filter: Option<Vec<Option<String>>> - List of node type name to remove during filtering. Any of node types must match with one of the node types given.
    /// * edge_node_names_to_keep: Option<Vec<(&str, &str)>> - List of tuple of node names to keep during filtering.
    /// * edge_node_names_to_filter: Option<Vec<(&str, &str)>> - List of tuple of node names to remove during filtering.
    /// * edge_type_names_to_keep: Option<Vec<Option<String>>> - List of edge type names to keep during filtering.
    /// * edge_type_names_to_filter: Option<Vec<Option<String>>> - List of edge type names to remove during filtering.
    /// * min_edge_weight: Option<WeightT> - Minimum edge weight. Values lower than this are removed.
    /// * max_edge_weight: Option<WeightT> - Maximum edge weight. Values higher than this are removed.
    /// * filter_singletons: bool - Whether to filter out singletons.
    /// * filter_selfloops: bool - Whether to filter out selfloops.
    /// * verbose: bool - Whether to show loading bar while building the graphs.
    ///
    /// ## Implementation details
    ///
    /// ### How the collapse of multigraphs is handled
    /// We keep only the first edge when a multigraph is collapsed while removing
    /// the edge types, in the order provided when first reading from the CSV file.
    ///
    /// ### Generation of new singleton nodes when removing edges
    /// Some of the remove operations allowed in this method might lead to the
    /// generation of new singleton nodes that will not be handled within this
    /// function call even if you provide the flag singletons to true, but you
    /// will need to call the method again if you want to get reed of also those
    /// newly created singleton nodes.
    ///
    pub fn filter_from_names(
        &self,
        node_names_to_keep: Option<Vec<&str>>,
        node_names_to_filter: Option<Vec<&str>>,
        node_type_names_to_keep: Option<Vec<Option<Vec<&str>>>>,
        node_type_names_to_filter: Option<Vec<Option<Vec<&str>>>>,
        node_type_name_to_keep: Option<Vec<Option<String>>>,
        node_type_name_to_filter: Option<Vec<Option<String>>>,
        edge_node_names_to_keep: Option<Vec<(&str, &str)>>,
        edge_node_names_to_filter: Option<Vec<(&str, &str)>>,
        edge_type_names_to_keep: Option<Vec<Option<String>>>,
        edge_type_names_to_filter: Option<Vec<Option<String>>>,
        min_edge_weight: Option<WeightT>,
        max_edge_weight: Option<WeightT>,
        filter_singletons: bool,
        filter_selfloops: bool,
        verbose: bool,
    ) -> Result<Graph, String> {
        Ok(self.filter_from_ids(
            node_names_to_keep.map_or(Ok::<_, String>(None), |nntk| {
                Ok(Some(self.get_node_ids_from_node_names(nntk)?))
            })?,
            node_names_to_filter.map_or(Ok::<_, String>(None), |nntf| {
                Ok(Some(self.get_node_ids_from_node_names(nntf)?))
            })?,
            node_type_names_to_keep.map_or(Ok::<_, String>(None), |ntntk| {
                Ok(Some(
                    self.get_multiple_node_type_ids_from_node_type_names(ntntk)?,
                ))
            })?,
            node_type_names_to_filter.map_or(Ok::<_, String>(None), |ntntf| {
                Ok(Some(
                    self.get_multiple_node_type_ids_from_node_type_names(ntntf)?,
                ))
            })?,
            node_type_name_to_keep.map_or(Ok::<_, String>(None), |ntntf| {
                Ok(Some(self.get_node_type_ids_from_node_type_names(ntntf)?))
            })?,
            node_type_name_to_filter.map_or(Ok::<_, String>(None), |ntntf| {
                Ok(Some(self.get_node_type_ids_from_node_type_names(ntntf)?))
            })?,
            None,
            None,
            edge_node_names_to_keep.map_or(Ok::<_, String>(None), |enntk| {
                Ok(Some(self.get_edge_node_ids_from_edge_node_names(enntk)?))
            })?,
            edge_node_names_to_filter.map_or(Ok::<_, String>(None), |enntf| {
                Ok(Some(self.get_edge_node_ids_from_edge_node_names(enntf)?))
            })?,
            edge_type_names_to_keep.map_or(Ok::<_, String>(None), |etnk| {
                Ok(Some(self.get_edge_type_ids_from_edge_type_names(etnk)?))
            })?,
            edge_type_names_to_filter.map_or(Ok::<_, String>(None), |etnf| {
                Ok(Some(self.get_edge_type_ids_from_edge_type_names(etnf)?))
            })?,
            min_edge_weight,
            max_edge_weight,
            filter_singletons,
            filter_selfloops,
            verbose,
        ))
    }

    /// Returns new graph without singleton nodes.
    ///
    /// A node is singleton when does not have neither incoming or outgoing edges.
    ///
    /// # Arguments
    /// * `verbose`: bool - Whether to show a loading bar while building the graph.
    pub fn drop_singletons(&self, verbose: bool) -> Graph {
        self.filter_from_ids(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            true, false, verbose,
        )
    }

    /// Returns new graph without selfloops.
    ///
    /// # Arguments
    /// * `verbose`: bool - Whether to show a loading bar while building the graph.
    pub fn drop_selfloops(&self, verbose: bool) -> Graph {
        self.filter_from_ids(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            false, true, verbose,
        )
    }
}
