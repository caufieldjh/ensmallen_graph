use crate::{
    add_numeric_id_to_csv, convert_edge_list_to_numeric, convert_node_list_node_types_to_numeric,
    densify_sparse_numeric_edge_list, get_minmax_node_from_numeric_edge_list, is_numeric_edge_list,
    sort_numeric_edge_list_inplace, EdgeT, EdgeTypeT, NodeT, NodeTypeT, Result, WeightT,
};
use log::info;

/// TODO: write the docstring
pub fn build_optimal_lists_files(
    mut original_node_type_path: Option<String>,
    mut original_node_type_list_separator: Option<String>,
    mut original_node_types_column_number: Option<usize>,
    original_node_types_column: Option<String>,
    mut original_node_types_ids_column_number: Option<usize>,
    mut original_node_types_ids_column: Option<String>,
    original_numeric_node_type_ids: Option<bool>,
    original_minimum_node_type_id: Option<NodeTypeT>,
    mut original_node_type_list_header: Option<bool>,
    mut original_node_type_list_rows_to_skip: Option<usize>,
    mut original_node_type_list_max_rows_number: Option<usize>,
    mut original_node_type_list_comment_symbol: Option<String>,
    original_load_node_type_list_in_parallel: Option<bool>,
    original_node_type_list_is_correct: Option<bool>,
    mut node_types_number: Option<NodeTypeT>,

    mut target_node_type_list_path: Option<String>,
    mut target_node_type_list_separator: Option<String>,
    mut target_node_type_list_node_types_column_number: Option<usize>,
    mut target_node_type_list_node_types_column: Option<String>,
    mut target_node_types_ids_column_number: Option<usize>,
    mut target_node_types_ids_column: Option<String>,
    mut target_node_type_list_header: Option<bool>,

    mut original_node_path: Option<String>,
    mut original_node_list_separator: Option<String>,
    mut original_node_list_header: Option<bool>,
    node_list_rows_to_skip: Option<usize>,
    node_list_is_correct: Option<bool>,
    node_list_max_rows_number: Option<usize>,
    node_list_comment_symbol: Option<String>,
    default_node_type: Option<String>,
    mut original_nodes_column_number: Option<usize>,
    mut original_nodes_column: Option<String>,
    original_node_types_separator: Option<String>,
    original_node_list_node_types_column_number: Option<usize>,
    original_node_list_node_types_column: Option<String>,
    mut original_node_ids_column: Option<String>,
    mut original_node_ids_column_number: Option<usize>,
    nodes_number: Option<NodeT>,
    original_minimum_node_id: Option<NodeT>,
    original_numeric_node_ids: Option<bool>,
    original_node_list_numeric_node_type_ids: Option<bool>,
    original_skip_node_types_if_unavailable: Option<bool>,
    original_load_node_list_in_parallel: Option<bool>,
    mut maximum_node_id: Option<EdgeT>,

    target_node_path: Option<String>,
    mut target_node_list_separator: Option<String>,
    target_node_list_header: Option<bool>,
    mut target_nodes_column: Option<String>,
    mut target_nodes_column_number: Option<usize>,
    target_node_types_separator: Option<String>,
    mut target_node_list_node_types_column: Option<String>,
    mut target_node_list_node_types_column_number: Option<usize>,
    mut target_node_ids_column: Option<String>,
    mut target_node_ids_column_number: Option<usize>,

    mut original_edge_type_path: Option<String>,
    mut original_edge_type_list_separator: Option<String>,
    mut original_edge_types_column_number: Option<usize>,
    original_edge_types_column: Option<String>,
    mut original_edge_types_ids_column_number: Option<usize>,
    mut original_edge_types_ids_column: Option<String>,
    original_numeric_edge_type_ids: Option<bool>,
    original_minimum_edge_type_id: Option<EdgeTypeT>,
    mut original_edge_type_list_header: Option<bool>,
    mut edge_type_list_rows_to_skip: Option<usize>,
    mut edge_type_list_max_rows_number: Option<usize>,
    mut edge_type_list_comment_symbol: Option<String>,
    load_edge_type_list_in_parallel: Option<bool>,
    edge_type_list_is_correct: Option<bool>,
    mut edge_types_number: Option<NodeTypeT>,

    mut target_edge_type_list_path: Option<String>,
    mut target_edge_type_list_separator: Option<String>,
    mut target_edge_type_list_edge_types_column_number: Option<usize>,
    mut target_edge_type_list_edge_types_column: Option<String>,
    mut target_edge_types_ids_column_number: Option<usize>,
    mut target_edge_types_ids_column: Option<String>,
    mut target_edge_type_list_header: Option<bool>,

    mut original_edge_path: String,
    original_edge_list_separator: Option<String>,
    original_edge_list_header: Option<bool>,
    original_sources_column_number: Option<usize>,
    original_sources_column: Option<String>,
    original_destinations_column_number: Option<usize>,
    original_destinations_column: Option<String>,
    original_edge_list_edge_types_column_number: Option<usize>,
    original_edge_list_edge_types_column: Option<String>,
    default_edge_type: Option<String>,
    original_weights_column_number: Option<usize>,
    original_weights_column: Option<String>,
    default_weight: Option<WeightT>,
    original_edge_list_numeric_node_ids: Option<bool>,
    skip_weights_if_unavailable: Option<bool>,
    skip_edge_types_if_unavailable: Option<bool>,
    edge_list_comment_symbol: Option<String>,
    edge_list_max_rows_number: Option<usize>,
    edge_list_rows_to_skip: Option<usize>,
    load_edge_list_in_parallel: Option<bool>,
    mut edges_number: Option<EdgeT>,

    target_edge_path: String,
    target_edge_list_separator: Option<String>,

    verbose: Option<bool>,
    directed: bool,
    name: Option<String>,
) -> Result<(Option<NodeTypeT>, NodeT, Option<EdgeTypeT>, EdgeT)> {
    // It does not make sense to provide a node types file
    // to be parsed but not providing any node type column
    // to be loaded within the node list file.
    if original_node_type_path.is_some()
        && original_node_list_node_types_column_number.is_none()
        && original_node_list_node_types_column.is_none()
    {
        return Err(concat!(
            "A node type path was provided, but no node type column name or number was specified ",
            "for the node list file."
        )
        .to_string());
    }

    let _ = target_node_ids_column.insert("node_id".to_string());
    let _ = target_node_ids_column_number.insert(0);
    let _ = target_nodes_column.insert("node_name".to_string());
    let _ = target_nodes_column_number.insert(1);

    if original_node_list_node_types_column_number.is_some()
        || original_node_list_node_types_column.is_some()
    {
        let _ = target_node_types_ids_column.insert("node_type_id".to_string());
        let _ = target_node_types_ids_column_number.insert(0);
        let _ = target_node_type_list_node_types_column.insert("node_type".to_string());
        let _ = target_node_type_list_node_types_column_number.insert(1);
        let _ = target_node_list_node_types_column.insert("node_type".to_string());
        let _ = target_node_list_node_types_column_number.insert(2);
    }

    if original_edge_list_edge_types_column_number.is_some()
        || original_edge_list_edge_types_column.is_some()
    {
        let _ = target_edge_types_ids_column.insert("edge_type_id".to_string());
        let _ = target_edge_types_ids_column_number.insert(0);
        let _ = target_edge_type_list_edge_types_column.insert("edge_type".to_string());
        let _ = target_edge_type_list_edge_types_column_number.insert(1);
    }

    // If the node types have been provided, we need to check that
    // the node type IDs have been provided, which are necessary
    // to guarantee that the node types are loaded in parallel
    // in a deterministic way.
    if let Some(original_node_type_path) = &mut original_node_type_path {
        if original_node_types_ids_column_number.is_none()
            && original_node_types_ids_column.is_none()
        {
            if target_node_type_list_path.is_none() {
                return Err(concat!(
                    "The original node type path was provided, and since ",
                    "neither the node type ids column number nor the ",
                    "node type ids column name was provided it has been ",
                    "assumed that the node types list does not come with ",
                    "an index.\n",
                    "Since the index is necessary to guarantee determinism ",
                    "when loading the file in parallel, it was attempted ",
                    "to create a version of the file with also the index.\n",
                    "Since the target node type path was not provided, ",
                    "the pipeline has failed because it is not known where ",
                    "to store this file.\n",
                    "Please provide the expected target node types list file path."
                )
                .to_string());
            }
            info!("Creating the node types file with numeric indices.");
            node_types_number = Some(add_numeric_id_to_csv(
                original_node_type_path.as_ref(),
                original_node_type_list_separator.clone(),
                original_node_type_list_header,
                target_node_type_list_path.clone().unwrap().as_ref(),
                target_node_type_list_separator.clone(),
                target_node_type_list_header,
                target_node_types_ids_column.clone(),
                target_node_types_ids_column_number,
                original_node_type_list_comment_symbol.clone(),
                original_node_type_list_max_rows_number,
                original_node_type_list_rows_to_skip,
                node_types_number
                    .as_ref()
                    .map(|node_types_number| (*node_types_number) as usize),
                verbose,
            )? as NodeTypeT);
            // After we recreate the correct file, we now refer
            // to this file as the original node type list file.
            // Since the file is now without commented lines,
            // lines top skip etc.. we can set all of these parameters
            // to None.
            original_node_type_list_comment_symbol = None;
            original_node_type_list_max_rows_number = None;
            original_node_type_list_rows_to_skip = None;
            original_node_types_ids_column = target_node_types_ids_column;
            target_node_types_ids_column = None;
            original_node_types_ids_column_number = target_node_types_ids_column_number;
            target_node_types_ids_column_number = None;
            *original_node_type_path = target_node_type_list_path.unwrap();
            target_node_type_list_path = None;
            // If the node types column number is higher or equal to the node type IDs column,
            // we need to offset it.
            if let (
                Some(original_node_types_column_number),
                Some(original_node_types_ids_column_number),
            ) = (
                &mut original_node_types_column_number,
                original_node_types_ids_column_number,
            ) {
                if *original_node_types_column_number >= original_node_types_ids_column_number {
                    *original_node_types_column_number += 1;
                }
            }
            if target_node_type_list_separator.is_some() {
                original_node_type_list_separator = target_node_type_list_separator;
            }
            if target_node_type_list_header.is_some() {
                original_node_type_list_header = target_node_type_list_header;
            }

            target_node_type_list_separator = None;
            target_node_type_list_header = None;
        }
    }

    // It does not make sense to provide a edge types file
    // to be parsed but not providing any edge type column
    // to be loaded within the edge list file.
    if original_edge_type_path.is_some()
        && original_edge_list_edge_types_column_number.is_none()
        && original_edge_list_edge_types_column.is_none()
    {
        return Err(concat!(
            "A edge type path was provided, but no edge type column name or number was specified ",
            "for the edge list file."
        )
        .to_string());
    }
    // If the edge types have been provided, we need to check that
    // the edge type IDs have been provided, which are necessary
    // to guarantee that the edge types are loaded in parallel
    // in a deterministic way.
    if let Some(original_edge_type_path) = &mut original_edge_type_path {
        if original_edge_types_ids_column_number.is_none()
            && original_edge_types_ids_column.is_none()
        {
            if target_edge_type_list_path.is_none() {
                return Err(concat!(
                    "The original edge type path was provided, and since ",
                    "neither the edge type ids column number nor the ",
                    "edge type ids column name was provided it has been ",
                    "assumed that the edge types list does not come with ",
                    "an index.\n",
                    "Since the index is necessary to guarantee determinism ",
                    "when loading the file in parallel, it was attempted ",
                    "to create a version of the file with also the index.\n",
                    "Since the target edge type path was not provided, ",
                    "the pipeline has failed because it is not known where ",
                    "to store this file.\n",
                    "Please provide the expected target edge types list file path."
                )
                .to_string());
            }
            info!("Creating the edge types file with numeric indices.");
            edge_types_number = Some(add_numeric_id_to_csv(
                original_edge_type_path.as_ref(),
                original_edge_type_list_separator.clone(),
                original_edge_type_list_header,
                target_edge_type_list_path.clone().unwrap().as_ref(),
                target_edge_type_list_separator.clone(),
                target_edge_type_list_header,
                target_edge_types_ids_column.clone(),
                target_edge_types_ids_column_number,
                edge_type_list_comment_symbol.clone(),
                edge_type_list_max_rows_number,
                edge_type_list_rows_to_skip,
                edge_types_number
                    .as_ref()
                    .map(|edge_types_number| (*edge_types_number) as usize),
                verbose,
            )? as EdgeTypeT);
            // After we recreate the correct file, we now refer
            // to this file as the original edge type list file.
            // Since the file is now without commented lines,
            // lines top skip etc.. we can set all of these parameters
            // to None.
            edge_type_list_comment_symbol = None;
            edge_type_list_max_rows_number = None;
            edge_type_list_rows_to_skip = None;
            original_edge_types_ids_column = target_edge_types_ids_column;
            original_edge_types_ids_column_number = target_edge_types_ids_column_number;
            *original_edge_type_path = target_edge_type_list_path.unwrap();
            // If the edge types column number is higher or equal to the edge type IDs column,
            // we need to offset it.
            if let (
                Some(original_edge_types_column_number),
                Some(original_edge_types_ids_column_number),
            ) = (
                &mut original_edge_types_column_number,
                original_edge_types_ids_column_number,
            ) {
                if *original_edge_types_column_number >= original_edge_types_ids_column_number {
                    *original_edge_types_column_number += 1;
                }
            }
            if target_edge_type_list_separator.is_some() {
                original_edge_type_list_separator = target_edge_type_list_separator;
            }
            if target_edge_type_list_header.is_some() {
                original_edge_type_list_header = target_edge_type_list_header;
            }

            target_edge_type_list_path = None;
            target_edge_type_list_separator = None;
            target_edge_type_list_header = None;
            target_edge_type_list_edge_types_column = None;
            target_edge_type_list_edge_types_column_number = None;
            target_edge_types_ids_column = None;
            target_edge_types_ids_column_number = None;
        }
    }

    // We need to handle the optimization of the
    // nodes list, which only includes making sure that if there are
    // node types, there is a node types list and the node types
    // provided in the nodes file are numerical and dense.
    // Also, we need to make sure that the node list does not
    // include additional ignored fields in it, like for instance
    // textual node descriptions, that make loading the file
    // into Ensmallen slower.
    // Finally, the produced node list will also include the
    // node ID as a field, Ensuring that the parallel loading
    // procedure produces a deterministic internal node ID to
    // node name mapping.
    if let Some(original_node_path) = &mut original_node_path {
        if target_node_path.is_none() {
            return Err(concat!(
                "When providing the original node path that must be ",
                "parsed to produce the op"
            )
            .to_string());
        }

        info!("Converting the node list node type names to numeric node type IDs.");
        node_types_number = Some(convert_node_list_node_types_to_numeric(
            original_node_type_path,
            original_node_type_list_separator,
            original_node_types_column_number,
            original_node_types_column,
            original_node_types_ids_column_number,
            original_node_types_ids_column,
            node_types_number,
            original_numeric_node_type_ids,
            original_minimum_node_type_id,
            original_node_type_list_header,
            original_node_type_list_rows_to_skip,
            original_node_type_list_is_correct,
            original_node_type_list_max_rows_number,
            original_node_type_list_comment_symbol,
            original_load_node_type_list_in_parallel,
            target_node_type_list_path,
            target_node_type_list_separator,
            target_node_type_list_header,
            target_node_type_list_node_types_column,
            target_node_type_list_node_types_column_number,
            target_node_types_ids_column,
            target_node_types_ids_column_number,
            original_node_path.clone(),
            original_node_list_separator,
            original_node_list_header,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol.clone(),
            default_node_type,
            original_nodes_column_number,
            original_nodes_column,
            original_node_types_separator,
            original_node_list_node_types_column_number,
            original_node_list_node_types_column,
            original_node_ids_column,
            original_node_ids_column_number,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_node_list_numeric_node_type_ids,
            original_skip_node_types_if_unavailable,
            target_node_path.clone().unwrap(),
            target_node_list_separator.clone(),
            target_node_list_header,
            target_nodes_column_number,
            target_nodes_column.clone(),
            target_node_types_separator.clone(),
            target_node_list_node_types_column_number,
            target_node_list_node_types_column,
            target_node_ids_column.clone(),
            target_node_ids_column_number,
            nodes_number,
        )? as NodeTypeT);
        // Now we need to update the node list parameters
        // that should be used in the next step.
        // We do not update again the node types as it
        // is not needed after this step.
        *original_node_path = target_node_path.clone().unwrap();
        original_node_list_separator = target_node_list_separator;
        target_node_list_separator = None;
        original_node_list_header = target_node_list_header;
        original_nodes_column_number = target_nodes_column_number;
        original_nodes_column = None;
        original_node_ids_column = None;
        original_node_ids_column_number = target_node_ids_column_number;
    }

    // We check if the edge list has numeric node IDs
    // unless the information was already provided.
    // We always treat as non-numeric the nodes if the
    // node list vocabulary has been provided.
    info!("Computing whether the edge list has numeric node IDs.");
    let numeric_edge_list_node_ids = !original_node_path.is_some()
        && (original_edge_list_numeric_node_ids.unwrap_or(false)
            || is_numeric_edge_list(
                original_edge_path.as_ref(),
                original_edge_list_separator.clone(),
                original_edge_list_header,
                original_sources_column.clone(),
                original_sources_column_number,
                original_destinations_column.clone(),
                original_destinations_column_number,
                edge_list_comment_symbol.clone(),
                edge_list_max_rows_number,
                edge_list_rows_to_skip,
                None,
                load_edge_list_in_parallel,
                verbose,
                name.clone(),
            )?);

    // We identify if the edge list is meant to have edge types
    let has_edge_types = original_edge_list_edge_types_column.is_some()
        || original_edge_list_edge_types_column_number.is_some();
    // We identify if the edge list is meant to have edge weights
    let has_edge_weights =
        original_weights_column.is_some() || original_weights_column_number.is_some();
    // We update the target path to a temporary one
    let target_numeric_edge_path: String =
        format!("{}.numeric_edge_list.tmp", target_edge_path.clone());

    // We convert the edge list to dense numeric
    let (nodes_number, edge_types_number) = if numeric_edge_list_node_ids {
        info!("Computing maximum node ID from sparse numeric edge list.");
        if maximum_node_id.is_none() {
            let (_, new_maximum_node_id, new_edges_number) =
                get_minmax_node_from_numeric_edge_list(
                    original_edge_path.as_ref(),
                    original_edge_list_separator.clone(),
                    original_edge_list_header,
                    original_sources_column.clone(),
                    original_sources_column_number,
                    original_destinations_column.clone(),
                    original_destinations_column_number,
                    edge_list_comment_symbol.clone(),
                    edge_list_max_rows_number,
                    edge_list_rows_to_skip,
                    None,
                    load_edge_list_in_parallel,
                    verbose,
                    name.clone(),
                )?;
            maximum_node_id = Some(new_maximum_node_id);
            edges_number = Some(new_edges_number);
        }

        info!("Converting sparse numeric edge list to dense numeric edge list.");
        densify_sparse_numeric_edge_list(
            maximum_node_id,
            original_edge_path.as_ref(),
            original_edge_list_separator.clone(),
            original_edge_list_header,
            original_sources_column.clone(),
            original_sources_column_number,
            original_destinations_column.clone(),
            original_destinations_column_number,
            original_edge_list_edge_types_column.clone(),
            original_edge_list_edge_types_column_number,
            original_weights_column.clone(),
            original_weights_column_number,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            original_edge_types_ids_column_number,
            original_edge_types_ids_column,
            edge_types_number,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            target_numeric_edge_path.as_ref(),
            target_edge_list_separator.clone(),
            Some(false),
            None,
            Some(0),
            None,
            Some(1),
            None,
            if has_edge_types { Some(2) } else { None },
            None,
            if has_edge_weights {
                Some(2 + has_edge_types as usize)
            } else {
                None
            },
            target_node_path.as_deref(),
            target_node_list_separator.clone(),
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_node_ids_column,
            target_node_ids_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator,
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            target_edge_types_ids_column,
            target_edge_types_ids_column_number,
            edge_list_comment_symbol.clone(),
            default_edge_type.clone(),
            default_weight,
            edge_list_max_rows_number,
            edge_list_rows_to_skip,
            edges_number.map(|edges_number| edges_number as usize),
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            directed,
            verbose,
            name.clone(),
        )
    } else {
        info!("Converting non-numeric edge list to numeric.");
        convert_edge_list_to_numeric(
            original_node_path,
            original_node_list_separator,
            original_node_list_header,
            node_list_rows_to_skip,
            node_list_is_correct,
            node_list_max_rows_number,
            node_list_comment_symbol,
            original_nodes_column_number,
            original_nodes_column,
            original_node_ids_column,
            original_node_ids_column_number,
            nodes_number,
            original_minimum_node_id,
            original_numeric_node_ids,
            original_load_node_list_in_parallel,
            original_edge_type_path,
            original_edge_types_column_number,
            original_edge_types_column,
            original_edge_types_ids_column_number,
            original_edge_types_ids_column,
            edge_types_number,
            original_numeric_edge_type_ids,
            original_minimum_edge_type_id,
            original_edge_type_list_separator,
            original_edge_type_list_header,
            edge_type_list_rows_to_skip,
            edge_type_list_is_correct,
            edge_type_list_max_rows_number,
            edge_type_list_comment_symbol,
            load_edge_type_list_in_parallel,
            original_edge_path.as_ref(),
            original_edge_list_separator.clone(),
            original_edge_list_header,
            original_sources_column_number,
            original_sources_column.clone(),
            original_destinations_column_number,
            original_destinations_column.clone(),
            original_edge_list_edge_types_column.clone(),
            original_edge_list_edge_types_column_number,
            original_weights_column.clone(),
            original_weights_column_number,
            target_numeric_edge_path.as_ref(),
            target_edge_list_separator.clone(),
            Some(false),
            None,
            Some(0),
            None,
            Some(1),
            None,
            if has_edge_types { Some(2) } else { None },
            None,
            if has_edge_weights {
                Some(2 + has_edge_types as usize)
            } else {
                None
            },
            target_node_path.as_deref(),
            target_node_list_separator,
            target_node_list_header,
            target_nodes_column,
            target_nodes_column_number,
            target_node_ids_column,
            target_node_ids_column_number,
            target_edge_type_list_path,
            target_edge_type_list_separator,
            target_edge_type_list_header,
            target_edge_type_list_edge_types_column,
            target_edge_type_list_edge_types_column_number,
            target_edge_types_ids_column,
            target_edge_types_ids_column_number,
            edge_list_comment_symbol.clone(),
            default_edge_type.clone(),
            default_weight,
            edge_list_max_rows_number,
            edge_list_rows_to_skip,
            edges_number.map(|edges_number| edges_number as usize),
            skip_edge_types_if_unavailable,
            skip_weights_if_unavailable,
            directed,
            verbose,
            name.clone(),
        )
    }?;

    original_edge_path = target_numeric_edge_path;

    // Sort the edge list
    info!("Sorting the edge list.");
    sort_numeric_edge_list_inplace(
        original_edge_path.as_ref(),
        original_edge_list_separator.clone(),
        Some(false),
        None,
        Some(0),
        None,
        Some(1),
        None,
        if has_edge_types { Some(2) } else { None },
        None,
        None,
    )?;

    // Add the edge IDs to the edge list
    info!("Adding edge ID to the sorted complete edge list.");
    let edges_number = add_numeric_id_to_csv(
        original_edge_path.as_ref(),
        original_edge_list_separator.clone(),
        Some(false),
        target_edge_path.as_ref(),
        target_edge_list_separator,
        None,
        None,
        Some(0),
        None,
        None,
        None,
        edges_number.map(|edges_number| edges_number as usize),
        verbose,
    )? as EdgeT;

    info!("Deleting previous temporary file with sorted complete edge list without edge ID.");
    match std::fs::remove_file(original_edge_path.clone()) {
        Ok(()) => {}
        Err(_) => {
            return Err(format!(
                concat!("It is not possible to delete the edge list without edge IDs file {}.",),
                original_edge_path.clone()
            ));
        }
    };

    Ok((
        node_types_number,
        nodes_number,
        edge_types_number,
        edges_number,
    ))
}