use super::*;
use rayon::iter::ParallelIterator;

impl Graph {
    /// Compute the maximum and minimum edge weight and cache it
    fn compute_max_and_min_edge_weight(&self) {
        let mut cache = unsafe { &mut (*self.cache.get()) };

        let (min, max, total) = match self.par_iter_edge_weights() {
            Ok(iter) => {
                let (min, max, total) = iter.map(|w| (w, w, w)).reduce(
                    || (WeightT::NAN, WeightT::NAN, 0.0),
                    |(min_a, max_a, total_a), (min_b, max_b, total_b)| (min_a.min(min_b), max_a.max(max_b), total_a + total_b),
                );
                (Ok(min), Ok(max), Ok(total))
            }
            Err(e) => (Err(e.clone()), Err(e), Err(e)),
        };

        cache.min_edge_weight = Some(min);
        cache.max_edge_weight = Some(max);
        cache.total_edge_weight = Some(total);
    }

    cached_property!(get_total_edge_weights, Result<WeightT>, compute_max_and_min_edge_weight, min_edge_weight,
    /// Return total edge weights, if graph has weights.
    ///
    /// # Example
    /// To get the total edge weights you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_total_edge_weights().is_ok());
    /// assert!(graph_without_weights.get_total_edge_weights().is_err());
    /// println!("The graph total edge weights is {:?}.", graph_with_weights.get_total_edge_weights());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
        );

    cached_property!(get_mininum_edge_weight, Result<WeightT>, compute_max_and_min_edge_weight, min_edge_weight,
    /// Return the minimum weight, if graph has weights.
    ///
    /// # Example
    /// To get the minimum edge weight you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_mininum_edge_weight().is_ok());
    /// assert!(graph_without_weights.get_mininum_edge_weight().is_err());
    /// println!("The graph minimum weight is {:?}.", graph_with_weights.get_mininum_edge_weight());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    );

    cached_property!(get_maximum_edge_weight, Result<WeightT>, compute_max_and_min_edge_weight, max_edge_weight,
    /// Return the maximum weight, if graph has weights.
    ///
    /// # Example
    /// To get the maximum edge weight you can use:
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.get_maximum_edge_weight().is_ok());
    /// assert!(graph_without_weights.get_maximum_edge_weight().is_err());
    /// println!("The graph maximum weight is {:?}.", graph_with_weights.get_maximum_edge_weight());
    /// ```
    ///
    /// # Raises
    /// * If the graph does not contain edge weights.
    );


    /// Compute the maximum and minimum node degree and cache it
    fn compute_max_and_min_node_degree(&self) {
        let mut cache = unsafe { &mut (*self.cache.get()) };

        let (min, max) = self.par_iter_node_degrees().map(|w| (w, w)).reduce(
            || (NodeT::MAX, 0),
            |(min_a, max_a), (min_b, max_b)| (min_a.min(min_b), max_a.max(max_b)),
        );

        cache.min_node_degree = Some(min);
        cache.max_node_degree = Some(max);
    }

    cached_property!(get_unchecked_maximum_node_degree, NodeT, compute_max_and_min_node_degree, max_node_degree,
    /// Return the maximum node degree.
    ///
    /// # Safety
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    ///
    );

    cached_property!(get_unchecked_minimum_node_degree, NodeT, compute_max_and_min_node_degree, min_node_degree,
    /// Return the minimum node degree.
    ///
    /// # Safety
    /// The method will return an undefined value (0) when the graph
    /// does not contain nodes. In those cases the value is not properly
    /// defined.
    ///
    );

    /// Compute the maximum and minimum weighted node degree and cache it
    fn compute_max_and_min_weighted_node_degree(&self) {
        let mut cache = unsafe { &mut (*self.cache.get()) };

        let (min, max) = match self.par_iter_weighted_node_degrees() {
            Ok(iter) => {
                let (min, max) = iter.map(|w| (w, w)).reduce(
                    || (f64::NAN, f64::NAN),
                    |(min_a, max_a), (min_b, max_b)| (min_a.min(min_b), max_a.max(max_b)),
                );
                (Ok(min), Ok(max))
            }
            Err(e) => (Err(e.clone()), Err(e)),
        };

        cache.min_weighted_node_degree = Some(min);
        cache.max_weighted_node_degree = Some(max);
    }

    cached_property!(get_weighted_maximum_node_degree, Result<f64>, compute_max_and_min_weighted_node_degree, max_weighted_node_degree,
    /// Return the maximum weighted node degree.
    );

    cached_property!(get_weighted_minimum_node_degree, Result<f64>, compute_max_and_min_weighted_node_degree, min_weighted_node_degree,
    /// Return the minimum weighted node degree.
    );

    /// Compute how many selfloops and how many **uniques** selfloops the graph contains.
    fn compute_selfloops_number(&self) {
        let (selfloops_number, selfloops_number_unique) = self
            .par_iter_node_ids()
            .map(|node_id| {
                let self_loops_number =
                    unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id) }
                        .filter(|x| *x == node_id)
                        .count();
                (
                    self_loops_number as EdgeT,
                    (if self_loops_number > 0 { 1 } else { 0 }) as NodeT,
                )
            })
            .reduce(
                || (0, 0),
                |(selfloops_number_a, selfloops_number_unique_a),
                 (selfloops_number_b, selfloops_number_unique_b)| {
                    (
                        selfloops_number_a + selfloops_number_b,
                        selfloops_number_unique_a + selfloops_number_unique_b,
                    )
                },
            );

        let mut cache = unsafe { &mut (*self.cache.get()) };
        cache.selfloops_number = Some(selfloops_number);
        cache.selfloops_number_unique = Some(selfloops_number_unique);
    }

    cached_property!(get_selfloops_number, EdgeT, compute_selfloops_number, selfloops_number,
        /// Returns number of self-loops, including also those in eventual multi-edges.
        ///
        /// # Example
        ///```rust
        /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
        /// println!("The number of self-loops in the graph is  {}", graph.get_selfloops_number());
        /// ```
    );

    cached_property!(get_unique_selfloop_number, NodeT, compute_selfloops_number, selfloops_number_unique,
        /// Returns number of unique self-loops, excluding those in eventual multi-edges.
        ///
        /// # Example
        ///```rust
        /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
        /// println!("The number of unique self-loops in the graph is  {}", graph.get_unique_selfloop_number());
        /// ```
    );
}
