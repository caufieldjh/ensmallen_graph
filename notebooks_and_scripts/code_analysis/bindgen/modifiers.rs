use super::*;
impl Graph {

	#[text_signature = "($self, vector_sources, vector_destinations, vector_cumulative_node_degrees, cache_size)"]
	/// TODO!: This binding was automatically generated
	/// Enable extra perks that buys you time as you accept to spend more memory.
	/// 
	/// Paramenters
	/// --------------
	/// vector_sources : bool,
	/// 	Whether to cache sources into a vector for faster walks.
	/// vector_destinations : bool,
	/// 	Whether to cache destinations into a vector for faster walks.
	/// vector_cumulative_node_degrees : bool,
	/// 	Whether to cache cumulative_node_degrees into a vector for faster walks.
	/// cache_size : float,
	/// 	percentage of nodes destinations to cache. This cannot be used with the vector destinations.
	fn enable(&mut self, vector_sources : bool, vector_destinations : bool, vector_cumulative_node_degrees : bool, cache_size : Option<f64>) -> PyResult<()> {
		pe!(self.graph.enable(vector_sources, vector_destinations, vector_cumulative_node_degrees, cache_size))
	}
	
	#[text_signature = "($self)"]
	/// TODO!: This binding was automatically generated
	/// Disable all extra perks, reducing memory impact but incresing time requirements.
	fn disable_all(&mut self){
		self.graph.disable_all()
	}
	
}
