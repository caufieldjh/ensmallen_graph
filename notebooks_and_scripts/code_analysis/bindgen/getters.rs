use super::*;
impl Graph {

	#[text_signature = "($self, verbose)"]
	/// Returns number a triple with (number of components, number of nodes of the smallest component, number of nodes of the biggest component )
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show a loading bar or not.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_connected_components_number(&self, verbose : bool) -> (NodeT, NodeT, NodeT) {
		self.graph.get_connected_components_number(verbose)
	}
	
	#[text_signature = "($self)"]
	/// Return vector with node cumulative_node_degrees, that is the comulative node degree.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_cumulative_node_degrees(&self) -> Vec<EdgeT> {
		self.graph.get_cumulative_node_degrees()
	}
	
	#[text_signature = "($self)"]
	/// Return mapping from instance not trap nodes to dense nodes.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_dense_nodes_mapping(&self) -> HashMap<NodeT, NodeT> {
		self.graph.get_dense_nodes_mapping()
	}
	
	#[text_signature = "($self)"]
	/// Returns density of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_density(&self) -> PyResult<f64> {
		pe!(self.graph.get_density())
	}
	
	#[text_signature = "($self)"]
	/// Returns number of directed edges in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_directed_edges_number(&self) -> EdgeT {
		self.graph.get_directed_edges_number()
	}
	
	#[text_signature = "($self, directed)"]
	/// Return vector with the sorted edge names.
	/// 
	/// Parameters
	/// --------------
	/// directed: bool,
	/// 	Whether to filter out the undirected edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_node_names(&self, directed : bool) -> Vec<(String, String)> {
		self.graph.get_edge_node_names(directed)
	}
	
	#[text_signature = "($self)"]
	/// Returns edge type counts hashmap.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_type_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_edge_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// Return the weights of the graph edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_edge_weights(&self) -> PyResult<Vec<WeightT>> {
		pe!(self.graph.get_edge_weights())
	}
	
	#[text_signature = "($self)"]
	/// Return the maximum weight, if graph has weights.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_max_edge_weight(&self) -> PyResult<WeightT> {
		pe!(self.graph.get_max_edge_weight())
	}
	
	#[text_signature = "($self)"]
	/// Return the minimum weight, if graph has weights.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_min_edge_weight(&self) -> PyResult<WeightT> {
		pe!(self.graph.get_min_edge_weight())
	}
	
	#[text_signature = "($self)"]
	/// Returns minimum number of edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_minimum_edge_types_number(&self) -> EdgeT {
		self.graph.get_minimum_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns minimum number of node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_minimum_node_types_number(&self) -> NodeT {
		self.graph.get_minimum_node_types_number()
	}
	
	#[text_signature = "($self)"]
	/// Return number of edges that have multigraph syblings.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_multigraph_edges_number(&self) -> EdgeT {
		self.graph.get_multigraph_edges_number()
	}
	
	#[text_signature = "($self, verbose)"]
	/// Return a vector with the components each node belongs to.
	/// 
	///  E.g. If we have two components `[0, 2, 3]` and `[1, 4, 5]` the result will look like
	///  `[0, 1, 0, 0, 1, 1]`
	/// 
	/// Parameters
	/// --------------
	/// verbose: bool,
	/// 	Whether to show the loading bar.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_connected_component_ids(&self, verbose : bool) -> Vec<NodeT> {
		self.graph.get_node_connected_component_ids(verbose)
	}
	
	#[text_signature = "($self)"]
	/// Returns the degree of every node in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_degrees(&self) -> Vec<NodeT> {
		self.graph.get_node_degrees()
	}
	
	#[text_signature = "($self)"]
	/// Returns node type counts hashmap.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_counts_hashmap(&self) -> PyResult<HashMap<EdgeTypeT, usize>> {
		pe!(self.graph.get_node_type_counts_hashmap())
	}
	
	#[text_signature = "($self)"]
	/// Return the node types of the graph nodes.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_node_type_ids(&self) -> PyResult<Vec<Option<Vec<NodeTypeT>>>> {
		pe!(self.graph.get_node_type_ids())
	}
	
	#[text_signature = "($self)"]
	/// Returns number of not singleton nodes within the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_not_singleton_nodes_number(&self) -> NodeT {
		self.graph.get_not_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// Return set of nodes that are not singletons.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_not_singletons_node_ids(&self) -> Vec<NodeT> {
		self.graph.get_not_singletons_node_ids()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of self-loops, including also those in eventual multi-edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_selfloop_nodes_number(&self) -> EdgeT {
		self.graph.get_selfloop_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns rate of self-loops.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_selfloop_nodes_rate(&self) -> PyResult<f64> {
		pe!(self.graph.get_selfloop_nodes_rate())
	}
	
	#[text_signature = "($self)"]
	/// Returns number of singleton nodes within the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_nodes_number(&self) -> NodeT {
		self.graph.get_singleton_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of singleton nodes with self-loops within the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_singleton_nodes_with_selfloops_number(&self) -> NodeT {
		self.graph.get_singleton_nodes_with_selfloops_number()
	}
	
	#[text_signature = "($self)"]
	/// Return the number of traps (nodes without any outgoing edges that are not singletons)
	///  This also includes nodes with only a self-loops, therefore singletons with
	///  only a self-loops are not considered traps because you could make a walk on them.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_trap_nodes_number(&self) -> EdgeT {
		self.graph.get_trap_nodes_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns the traps rate of the graph.
	/// 
	///  THIS IS EXPERIMENTAL AND MUST BE PROVEN!
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_trap_nodes_rate(&self) -> f64 {
		self.graph.get_trap_nodes_rate()
	}
	
	#[text_signature = "($self)"]
	/// Return number of the unique edges in the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_directed_edges_number(&self) -> EdgeT {
		self.graph.get_unique_directed_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of unique edges of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_edges_number(&self) -> EdgeT {
		self.graph.get_unique_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of unique self-loops, excluding those in eventual multi-edges.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_selfloop_number(&self) -> NodeT {
		self.graph.get_unique_selfloop_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of undirected edges of the graph.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unique_undirected_edges_number(&self) -> EdgeT {
		self.graph.get_unique_undirected_edges_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of unknown edge types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unknown_edge_types_number(&self) -> EdgeT {
		self.graph.get_unknown_edge_types_number()
	}
	
	#[text_signature = "($self)"]
	/// Returns number of unknown node types.
	///
	/// [Automatically generated binding]
	/// [Automatically generated documentation]
	fn get_unknown_node_types_number(&self) -> NodeT {
		self.graph.get_unknown_node_types_number()
	}
	
}
