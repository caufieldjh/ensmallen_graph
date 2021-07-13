use super::*;
use rayon::iter::ParallelIterator;

impl Graph {
    /// Compute the maximum and minimum edge weight and cache it
    fn compute_edge_weights_properties(&self) {
        let mut cache = unsafe { &mut (*self.cache.get()) };

        let (min, max, total) = match self.par_iter_edge_weights() {
            Ok(iter) => {
                let (min, max, total) = iter.map(|w| (w, w, w as f64)).reduce(
                    || (WeightT::NAN, WeightT::NAN, 0.0f64),
                    |(min_a, max_a, total_a), (min_b, max_b, total_b)| {
                        (min_a.min(min_b), max_a.max(max_b), total_a + total_b)
                    },
                );
                (Ok(min), Ok(max), Ok(total))
            }
            Err(e) => (Err(e.clone()), Err(e), Err(e)),
        };

        cache.min_edge_weight = Some(min);
        cache.max_edge_weight = Some(max);
        cache.total_edge_weight = Some(total);
    }

    cached_property!(get_total_edge_weights, Result<f64>, compute_edge_weights_properties, total_edge_weight,
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

    cached_property!(get_mininum_edge_weight, Result<WeightT>, compute_edge_weights_properties, min_edge_weight,
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

    cached_property!(get_maximum_edge_weight, Result<WeightT>, compute_edge_weights_properties, max_edge_weight,
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

    cached_property!(get_weighted_mininum_node_degree, Result<f64>, compute_max_and_min_weighted_node_degree, min_weighted_node_degree,
    /// Return the minimum weighted node degree.
    );

    /// Compute how many selfloops and how many **uniques** selfloops  and how many singletons with selfloops the graph contains.
    fn compute_selfloops_number(&self) {

        /// Struct with the info we want to collect
        /// This is just a nice way to handle reduces
        struct Info {
            selfloops_number_unique: NodeT,
            selfloops_number: EdgeT,
            singleton_nodes_with_selfloops_number: NodeT,
        }

        impl Default for Info {
            fn default() -> Self {
                Info{
                    selfloops_number_unique: 0,
                    selfloops_number: 0,
                    singleton_nodes_with_selfloops_number: 0,
                }
            }
        }

        impl std::ops::Add<Self> for Info {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Info{
                    selfloops_number_unique: self.selfloops_number_unique + rhs.selfloops_number_unique,
                    selfloops_number: self.selfloops_number + rhs.selfloops_number,
                    singleton_nodes_with_selfloops_number: self.singleton_nodes_with_selfloops_number + rhs.singleton_nodes_with_selfloops_number,
                }
            }
        }

        let info = self
            .par_iter_node_ids()
            .map(|node_id| {
                let (selfloops_number, degree) =
                    unsafe { self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id) }
                        .map(|x| (if x == node_id {1} else {0}, 1))
                        .reduce(
                            |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2),
                        ).unwrap_or((0, 0));

                Info{
                    selfloops_number: selfloops_number as EdgeT,
                    selfloops_number_unique: (if selfloops_number > 0 { 1 } else { 0 }) as NodeT,
                    singleton_nodes_with_selfloops_number: (if selfloops_number == degree { 1 } else { 0 }) as NodeT,
                }
            })
            .reduce(
                Info::default,
                |a, b| a + b,
            );

        let mut cache = unsafe { &mut (*self.cache.get()) };
        cache.selfloops_number = Some(info.selfloops_number);
        cache.selfloops_number_unique = Some(info.selfloops_number_unique);
        cache.singleton_nodes_with_selfloops_number = Some(info.singleton_nodes_with_selfloops_number);
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

    cached_property!(get_singleton_nodes_with_selfloops_number, NodeT, compute_selfloops_number, singleton_nodes_with_selfloops_number,  
        /// Returns number of singleton nodes with self-loops within the graph.
        ///
        /// # Example
        ///```rust
        /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
        /// println!("The graph contains {} singleton nodes with self-loops", graph.get_singleton_nodes_with_selfloops_number());
        /// ```
    );

    /// Compute the bitvector and the number of nodes which have incoming edges.
    fn compute_connected_nodes(&self) {
        let bitvec = ConcurrentBitVec::with_capacity(self.get_nodes_number() as usize);

        // Compute in parallel the bitvector of all the nodes that have incoming edges
        self.par_iter_node_ids().for_each(|node_id| {
            unsafe{self.iter_unchecked_neighbour_node_ids_from_source_node_id(node_id)}.for_each(
                |dst_id| bitvec.set(dst_id as usize)
            );
        });

        let mut cache = unsafe { &mut (*self.cache.get()) };
        cache.connected_nodes_number = Some(bitvec.count_ones() as NodeT);
    }

    cached_property!(get_connected_nodes_number, NodeT, compute_connected_nodes, connected_nodes_number,  
        /// Returns number of not singleton nodes within the graph.
        ///
        /// # Example
        ///```rust
        /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
        /// println!("The graph contains {} not singleton nodes", graph.get_connected_nodes_number());
        /// ```
    );


}
