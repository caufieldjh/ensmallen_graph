var data = {lines:[
{"lineNum":"    1","line":"use super::*;"},
{"lineNum":"    2","line":"use log::info;"},
{"lineNum":"    3","line":"use std::collections::{HashMap};"},
{"lineNum":"    4","line":"use std::iter::FromIterator;"},
{"lineNum":"    5","line":"use rayon::prelude::*;"},
{"lineNum":"    6","line":""},
{"lineNum":"    7","line":"pub fn validate(","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"    8","line":"    sources: &[NodeT],"},
{"lineNum":"    9","line":"    destinations: &[NodeT],"},
{"lineNum":"   10","line":"    nodes_mapping: &HashMap<String, NodeT>,"},
{"lineNum":"   11","line":"    nodes_reverse_mapping: &[String],"},
{"lineNum":"   12","line":"    node_types: &Option<Vec<NodeTypeT>>,"},
{"lineNum":"   13","line":"    edge_types: &Option<Vec<EdgeTypeT>>,"},
{"lineNum":"   14","line":"    weights: &Option<Vec<WeightT>>"},
{"lineNum":"   15","line":") -> Result<(), String> {"},
{"lineNum":"   16","line":""},
{"lineNum":"   17","line":"    info!(\"Checking that the nodes mappings are of the same length.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   18","line":"    if nodes_mapping.len() != nodes_reverse_mapping.len() {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   19","line":"        return Err(format!(\"The size of the node_mapping ({}) does not match the size of the nodes_reverse_mapping ({}).\",","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"   20","line":"            nodes_mapping.len(), nodes_reverse_mapping.len()","class":"lineNoCov","hits":"0","possible_hits":"2",},
{"lineNum":"   21","line":"        ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   22","line":"    }"},
{"lineNum":"   23","line":""},
{"lineNum":"   24","line":"    if let Some(nt) = &node_types {","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"   25","line":"        info!(\"Checking that nodes and node types are of the same length.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   26","line":"        if nt.len() != nodes_reverse_mapping.len() {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   27","line":"            return Err(format!(\"The number of given nodes ({}) does not match the number of node_types ({}).\",","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"   28","line":"                nt.len(), nodes_reverse_mapping.len()","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   29","line":"            ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   30","line":"        }"},
{"lineNum":"   31","line":"    }"},
{"lineNum":"   32","line":""},
{"lineNum":"   33","line":"    if let Some(nt) = &node_types{","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   34","line":"        info!(\"Checking if every node used by the edges exists.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   35","line":"        for node in sources.iter().chain(destinations.iter()) {","class":"linePartCov","hits":"1","possible_hits":"5",},
{"lineNum":"   36","line":"            if *node >= nt.len() {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   37","line":"                return Err(format!(","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"   38","line":"                    \"A node provided with the edges (\'{}\') does not exists within given nodes.\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   39","line":"                    node"},
{"lineNum":"   40","line":"                ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   41","line":"            }"},
{"lineNum":"   42","line":"        }","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"   43","line":"    }"},
{"lineNum":"   44","line":""},
{"lineNum":"   45","line":"    if let Some(w) = weights {","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   46","line":"        info!(\"Checking for length between weights and given edges.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   47","line":"        if w.len() != sources.len(){","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   48","line":"            return Err(format!(\"Length of given weights ({}) does not match length of given edges ({}).\",","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"   49","line":"            w.len(), sources.len()));","class":"lineNoCov","hits":"0","possible_hits":"2",},
{"lineNum":"   50","line":"        }"},
{"lineNum":"   51","line":"        info!(\"Checking for non-zero weights.\");","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"   52","line":"        for weight in w.iter() {","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"   53","line":"            if *weight == 0.0 {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   54","line":"                return Err(format!(","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"   55","line":"                    \"One of the provided weights \'{}\' is either 0 or within float error to zero.\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   56","line":"                    weight"},
{"lineNum":"   57","line":"                ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   58","line":"            }"},
{"lineNum":"   59","line":"            if *weight < 0.0 {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   60","line":"                return Err(format!(","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"   61","line":"                    \"One of the provided weights \'{}\' is negative.\",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   62","line":"                    weight"},
{"lineNum":"   63","line":"                ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   64","line":"            }"},
{"lineNum":"   65","line":"            if weight.is_nan(){","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   66","line":"                return Err(String::from(","class":"lineNoCov","hits":"0","possible_hits":"2",},
{"lineNum":"   67","line":"                    \"One of the provided weights is NaN.\""},
{"lineNum":"   68","line":"                ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   69","line":"            }"},
{"lineNum":"   70","line":"            if weight.is_infinite(){","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   71","line":"                return Err(String::from(","class":"lineNoCov","hits":"0","possible_hits":"2",},
{"lineNum":"   72","line":"                    \"One of the provided weights is infinite.\""},
{"lineNum":"   73","line":"                ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   74","line":"            }"},
{"lineNum":"   75","line":"        }","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"   76","line":"    }"},
{"lineNum":"   77","line":""},
{"lineNum":"   78","line":"    if let Some(et) = edge_types {","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"   79","line":"        info!(\"Checking for length between edge types and given edges.\");","class":"lineNoCov","hits":"0","possible_hits":"2",},
{"lineNum":"   80","line":"        if et.len() != sources.len(){","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   81","line":"            return Err(format!(","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"   82","line":"                \"The len of edge types ({}) is different than the len of given edges ({}).  \",","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   83","line":"                et.len(), sources.len()","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   84","line":"            ));","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"   85","line":"        }"},
{"lineNum":"   86","line":"    }"},
{"lineNum":"   87","line":""},
{"lineNum":"   88","line":"    Ok(())","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   89","line":"}","class":"linePartCov","hits":"2","order":"1","possible_hits":"3",},
{"lineNum":"   90","line":""},
{"lineNum":"   91","line":"impl Graph {"},
{"lineNum":"   92","line":""},
{"lineNum":"   93","line":"    pub fn new_directed(","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"   94","line":"        sources: Vec<NodeT>,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"   95","line":"        destinations: Vec<NodeT>,"},
{"lineNum":"   96","line":"        nodes_mapping: HashMap<String, NodeT>,"},
{"lineNum":"   97","line":"        nodes_reverse_mapping: Vec<String>,"},
{"lineNum":"   98","line":"        node_types: Option<Vec<NodeTypeT>>,"},
{"lineNum":"   99","line":"        node_types_mapping: Option<HashMap<String, NodeTypeT>>,"},
{"lineNum":"  100","line":"        node_types_reverse_mapping: Option<Vec<String>>,"},
{"lineNum":"  101","line":"        edge_types: Option<Vec<EdgeTypeT>>,"},
{"lineNum":"  102","line":"        edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,"},
{"lineNum":"  103","line":"        edge_types_reverse_mapping: Option<Vec<String>>,"},
{"lineNum":"  104","line":"        weights: Option<Vec<WeightT>>,"},
{"lineNum":"  105","line":"        validate_input_data: Option<bool>,"},
{"lineNum":"  106","line":"    ) -> Result<Graph, String> {"},
{"lineNum":"  107","line":"        if validate_input_data.unwrap_or_else(|| true) {","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"  108","line":"            validate(","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"  109","line":"                &sources,","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  110","line":"                &destinations,","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  111","line":"                &nodes_mapping,"},
{"lineNum":"  112","line":"                &nodes_reverse_mapping,","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  113","line":"                &node_types,"},
{"lineNum":"  114","line":"                &edge_types,"},
{"lineNum":"  115","line":"                &weights"},
{"lineNum":"  116","line":"            )?;","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"  117","line":"        }"},
{"lineNum":"  118","line":""},
{"lineNum":"  119","line":"        let nodes_number = nodes_reverse_mapping.len();","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  120","line":""},
{"lineNum":"  121","line":"        info!(\"Computing unique edges.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  122","line":"        let unique_edges: HashMap<(NodeT, NodeT), EdgeT> =","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  123","line":"            HashMap::from_iter(","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  124","line":"                sources.iter().cloned().zip(","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  125","line":"                    destinations.iter().cloned()","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  126","line":"                ).enumerate().map(|(i, (src, dst))| ((src, dst), i))","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  127","line":"            );","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  128","line":""},
{"lineNum":"  129","line":"        info!(\"Computing sorting of given edges based on sources.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  130","line":"        let mut pairs: Vec<(usize, &NodeT)> = sources.par_iter().enumerate().collect();","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  131","line":"        pairs.par_sort_unstable_by_key(|(_, &v)| v);","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  132","line":"        let indices: Vec<&usize> = pairs.par_iter().map(|(i, _)| i).collect();","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  133","line":""},
{"lineNum":"  134","line":"        info!(\"Sorting given sources.\");","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  135","line":"        let sorted_sources: Vec<NodeT> = indices.par_iter()","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  136","line":"            .map(|&&x| sources[x]).collect();","class":"linePartCov","hits":"1","possible_hits":"5",},
{"lineNum":"  137","line":"        info!(\"Sorting given destinations.\");","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  138","line":"        let sorted_destinations: Vec<NodeT> = indices.par_iter()","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  139","line":"            .map(|&&x| destinations[x]).collect();","class":"linePartCov","hits":"1","possible_hits":"5",},
{"lineNum":"  140","line":"        info!(\"Sorting given weights.\");","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  141","line":"        let sorted_weights: Option<Vec<WeightT>> = weights.map(|w|","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  142","line":"            indices.par_iter()","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  143","line":"            .map(|&&x| w[x]).collect()","class":"linePartCov","hits":"1","possible_hits":"5",},
{"lineNum":"  144","line":"        );","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  145","line":"        info!(\"Sorting given edge types.\");","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  146","line":"        let sorted_edge_types: Option<Vec<EdgeTypeT>> = edge_types.map(|et|","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"  147","line":"            indices.par_iter()","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"  148","line":"            .map(|&&x| et[x]).collect()","class":"lineNoCov","hits":"0","possible_hits":"5",},
{"lineNum":"  149","line":"        );","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  150","line":""},
{"lineNum":"  151","line":"        let outbounds = Graph::compute_outbounds(nodes_number, &sorted_sources);","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  152","line":""},
{"lineNum":"  153","line":"        Ok(","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  154","line":"            Graph {","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  155","line":"            nodes_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  156","line":"            nodes_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  157","line":"            unique_edges,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  158","line":"            node_types,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  159","line":"            node_types_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  160","line":"            node_types_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  161","line":"            edge_types_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  162","line":"            edge_types_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  163","line":"            outbounds,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  164","line":"            sources: sorted_sources,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  165","line":"            destinations: sorted_destinations,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  166","line":"            weights: sorted_weights,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  167","line":"            edge_types: sorted_edge_types,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  168","line":"        })","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  169","line":"    }","class":"lineNoCov","hits":"0","possible_hits":"10",},
{"lineNum":"  170","line":""},
{"lineNum":"  171","line":"    pub fn new_undirected(","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  172","line":"        sources: Vec<NodeT>,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  173","line":"        destinations: Vec<NodeT>,"},
{"lineNum":"  174","line":"        nodes_mapping: HashMap<String, NodeT>,"},
{"lineNum":"  175","line":"        nodes_reverse_mapping: Vec<String>,"},
{"lineNum":"  176","line":"        node_types: Option<Vec<NodeTypeT>>,"},
{"lineNum":"  177","line":"        node_types_mapping: Option<HashMap<String, NodeTypeT>>,"},
{"lineNum":"  178","line":"        node_types_reverse_mapping: Option<Vec<String>>,"},
{"lineNum":"  179","line":"        edge_types: Option<Vec<EdgeTypeT>>,"},
{"lineNum":"  180","line":"        edge_types_mapping: Option<HashMap<String, EdgeTypeT>>,"},
{"lineNum":"  181","line":"        edge_types_reverse_mapping: Option<Vec<String>>,"},
{"lineNum":"  182","line":"        weights: Option<Vec<WeightT>>,"},
{"lineNum":"  183","line":"        validate_input_data: Option<bool>,"},
{"lineNum":"  184","line":"    ) -> Result<Graph, String> {"},
{"lineNum":"  185","line":""},
{"lineNum":"  186","line":"        if validate_input_data.unwrap_or_else(|| true) {","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  187","line":"            validate(","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  188","line":"                &sources,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  189","line":"                &destinations,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  190","line":"                &nodes_mapping,"},
{"lineNum":"  191","line":"                &nodes_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  192","line":"                &node_types,"},
{"lineNum":"  193","line":"                &edge_types,"},
{"lineNum":"  194","line":"                &weights"},
{"lineNum":"  195","line":"            )?;","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  196","line":"        }"},
{"lineNum":"  197","line":""},
{"lineNum":"  198","line":"        info!(\"Identifying self-loops present in given graph.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  199","line":"        let loops_mask: Vec<bool> = sources","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  200","line":"            .par_iter()"},
{"lineNum":"  201","line":"            .zip(destinations.par_iter())","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  202","line":"            .map(|(a, b)| a == b)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  203","line":"            .collect();","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  204","line":""},
{"lineNum":"  205","line":"        info!(\"Building undirected graph sources.\");","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  206","line":"        let mut full_sources: Vec<NodeT> = sources.clone();","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  207","line":"        full_sources.extend(","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  208","line":"            destinations","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  209","line":"                .par_iter()"},
{"lineNum":"  210","line":"                .zip(loops_mask.par_iter())","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  211","line":"                .filter(|&(_, &mask)| !mask)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  212","line":"                .map(|(value, _)| *value)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  213","line":"                .collect::<Vec<NodeT>>(),","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  214","line":"        );","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  215","line":""},
{"lineNum":"  216","line":"        info!(\"Building undirected graph destinations.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  217","line":"        let mut full_destinations: Vec<NodeT> = destinations;","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  218","line":"        full_destinations.extend(","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  219","line":"            sources","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  220","line":"                .par_iter()"},
{"lineNum":"  221","line":"                .zip(loops_mask.par_iter())","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  222","line":"                .filter(|&(_, &mask)| !mask)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  223","line":"                .map(|(value, _)| *value)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  224","line":"                .collect::<Vec<NodeT>>(),","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  225","line":"        );","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  226","line":""},
{"lineNum":"  227","line":"        let mut full_edge_types = edge_types;","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  228","line":"        if let Some(e) = &mut full_edge_types {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  229","line":"            info!(\"Building undirected graph edge types.\");","class":"lineNoCov","hits":"0","possible_hits":"2",},
{"lineNum":"  230","line":"            e.extend(","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  231","line":"                e.par_iter()","class":"lineNoCov","hits":"0","possible_hits":"4",},
{"lineNum":"  232","line":"                    .zip(loops_mask.par_iter())","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  233","line":"                    .filter(|&(_, &mask)| !mask)","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"  234","line":"                    .map(|(value, _)| *value)","class":"lineNoCov","hits":"0","possible_hits":"3",},
{"lineNum":"  235","line":"                    .collect::<Vec<NodeTypeT>>(),","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  236","line":"            );","class":"lineNoCov","hits":"0","possible_hits":"1",},
{"lineNum":"  237","line":"        };"},
{"lineNum":"  238","line":""},
{"lineNum":"  239","line":"        let mut full_weights = weights;","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  240","line":"        if let Some(w) = &mut full_weights {","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  241","line":"            info!(\"Building undirected graph weights.\");","class":"linePartCov","hits":"1","possible_hits":"2",},
{"lineNum":"  242","line":"            w.extend(","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  243","line":"                w.par_iter()","class":"linePartCov","hits":"1","possible_hits":"4",},
{"lineNum":"  244","line":"                    .zip(loops_mask.par_iter())","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  245","line":"                    .filter(|&(_, &mask)| !mask)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  246","line":"                    .map(|(value, _)| *value)","class":"linePartCov","hits":"1","possible_hits":"3",},
{"lineNum":"  247","line":"                    .collect::<Vec<WeightT>>(),","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  248","line":"            );","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  249","line":"        };"},
{"lineNum":"  250","line":""},
{"lineNum":"  251","line":"        Graph::new_directed(","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  252","line":"            full_sources,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  253","line":"            full_destinations,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  254","line":"            nodes_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  255","line":"            nodes_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  256","line":"            node_types,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  257","line":"            node_types_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  258","line":"            node_types_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  259","line":"            full_edge_types,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  260","line":"            edge_types_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  261","line":"            edge_types_reverse_mapping,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  262","line":"            full_weights,","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  263","line":"            Some(false),"},
{"lineNum":"  264","line":"        )","class":"lineCov","hits":"1","possible_hits":"1",},
{"lineNum":"  265","line":"    }","class":"lineNoCov","hits":"0","possible_hits":"5",},
{"lineNum":"  266","line":"}"},
]};
var percent_low = 25;var percent_high = 75;
var header = { "command" : "with_nodes", "date" : "2020-06-22 09:48:45", "instrumented" : 170, "covered" : 124,};
var merged_data = [];