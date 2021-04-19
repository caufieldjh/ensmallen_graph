use super::*;

/// # Validators
/// The naming convention we follow is:
/// * `validate_(.+)`
/// * `must_have_(.+)`
/// * `must_be_(.+)`
/// * `must_not_be_(.+)`
impl Graph {
    /// Validates provided node ID.
    ///
    /// # Arguments
    /// * `node_id`: NodeT - node ID to validate.
    ///
    /// # Example
    /// In order to validate a given node ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_node_id(0).is_ok());
    /// assert!(graph.validate_node_id(100000000).is_err());
    /// ```
    pub fn validate_node_id(&self, node_id: NodeT) -> Result<NodeT, String> {
        if node_id >= self.get_nodes_number() {
            return Err(format!(
                "The given node id ({}) is higher than the number of nodes within the graph ({}).",
                node_id,
                self.get_nodes_number()
            ));
        }
        Ok(node_id)
    }

    /// Validates provided edge ID.
    ///
    /// # Arguments
    /// * `edge_id`: EdgeT - Edge ID to validate.
    ///
    /// # Example
    /// In order to validate a given edge ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_edge_id(0).is_ok());
    /// assert!(graph.validate_edge_id(10000000000).is_err());
    /// ```
    pub fn validate_edge_id(&self, edge_id: EdgeT) -> Result<EdgeT, String> {
        if edge_id >= self.get_directed_edges_number() {
            return Err(format!(
                "The given edge id ({}) is higher than the number of edges within the graph ({}).",
                edge_id,
                self.get_directed_edges_number()
            ));
        }
        Ok(edge_id)
    }

    /// Validates provided node type ID.
    ///
    /// # Arguments
    /// * `node_type_id`: Option<NodeTypeT> - Node type ID to validate.
    ///
    /// # Example
    /// In order to validate a given node type ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_node_type_id(Some(0)).is_ok());
    /// assert!(graph.validate_node_type_id(Some(1000)).is_err());
    /// ```
    pub fn validate_node_type_id(
        &self,
        node_type_id: Option<NodeTypeT>,
    ) -> Result<Option<NodeTypeT>, String> {
        self.must_have_node_types()?;
        if let Some(nti) = node_type_id {
            if self.get_node_types_number() <= nti {
                return Err(format!(
                    "Given node type ID {:?} is bigger than number of node types in the graph {}.",
                    nti,
                    self.get_node_types_number()
                ));
            }
        }
        Ok(node_type_id)
    }

    /// Validates provided edge type ID.
    ///
    /// # Arguments
    /// * `edge_type_id`: Option<EdgeTypeT> - edge type ID to validate.
    ///
    /// # Example
    /// In order to validate a given edge type ID, you can use the following:
    ///
    /// ```rust
    /// # let graph = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// assert!(graph.validate_edge_type_id(Some(0)).is_ok());
    /// assert!(graph.validate_edge_type_id(Some(1000)).is_err());
    /// ```
    pub fn validate_edge_type_id(
        &self,
        edge_type_id: Option<EdgeTypeT>,
    ) -> Result<Option<EdgeTypeT>, String> {
        self.must_have_edge_types()?;
        if let Some(eti) = edge_type_id {
            if self.get_edge_types_number() <= eti {
                return Err(format!(
                    "Given edge type ID {:?} is bigger than number of edge types in the graph {}.",
                    eti,
                    self.get_edge_types_number()
                ));
            }
        }
        Ok(edge_type_id)
    }

    /// Raises an error if the graph does not have node types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_node_types = graph::test_utilities::load_ppi(true, true, true, true, false, false);
    /// # let graph_without_node_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// assert!(graph_with_node_types.must_have_node_types().is_ok());
    /// assert!(graph_without_node_types.must_have_node_types().is_err());
    /// ```
    pub fn must_have_node_types(&self) -> Result<(), String> {
        if !self.has_node_types() {
            return Err("The current graph instance does not have node types.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_edge_types = graph::test_utilities::load_ppi(false, true, true, true, false, false);
    /// # let graph_without_edge_types = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// assert!(graph_with_edge_types.must_have_edge_types().is_ok());
    /// assert!(graph_without_edge_types.must_have_edge_types().is_err());
    /// ```
    pub fn must_have_edge_types(&self) -> Result<(), String> {
        if !self.has_edge_types() {
            return Err("The current graph instance does not have edge types.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let undirecte_graph = graph::test_utilities::load_ppi(false, false, false, false, false, false);
    /// # let directed_graph = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// assert!(undirecte_graph.must_be_undirected().is_ok());
    /// assert!(directed_graph.must_be_undirected().is_err());
    /// ```
    pub fn must_be_undirected(&self) -> Result<(), String> {
        if self.is_directed() {
            return Err("The current graph instance is not undirected.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let multigraph = graph::test_utilities::load_ppi(false, true, false, false, false, false);
    /// # let homogeneous = graph::test_utilities::load_ppi(false, false, false, false, false, false);
    /// assert!(multigraph.must_be_multigraph().is_ok());
    /// assert!(homogeneous.must_be_multigraph().is_err());
    /// ```
    pub fn must_be_multigraph(&self) -> Result<(), String> {
        if !self.is_multigraph() {
            return Err(
                "The current graph instance must be a multigraph to run this method.".to_string(),
            );
        }
        Ok(())
    }

    /// Raises an error if the graph does not have edge types.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let multigraph = graph::test_utilities::load_ppi(false, true, false, false, false, false);
    /// # let homogeneous = graph::test_utilities::load_ppi(false, false, false, false, false, false);
    /// assert!(multigraph.must_be_multigraph().is_ok());
    /// assert!(homogeneous.must_be_multigraph().is_err());
    /// ```
    pub fn must_not_be_multigraph(&self) -> Result<(), String> {
        if self.is_multigraph() {
            return Err(
                "The current graph instance must not be a multigraph to run this method."
                    .to_string(),
            );
        }
        Ok(())
    }

    /// Raises an error if the graph does not have weights.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_weights = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_weights = graph::test_utilities::load_ppi(false, false, false, true, false, false);
    /// assert!(graph_with_weights.must_have_edge_weights().is_ok());
    /// assert!(graph_without_weights.must_have_edge_weights().is_err());
    /// ```
    pub fn must_have_edge_weights(&self) -> Result<(), String> {
        if !self.has_edge_weights() {
            return Err("The current graph instance does not have weights.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have any edge.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_edges = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_edges = graph::test_utilities::load_empty_graph(false);
    /// assert!(graph_with_edges.must_have_edges().is_ok());
    /// assert!(graph_without_edges.must_have_edges().is_err());
    /// ```
    pub fn must_have_edges(&self) -> Result<(), String> {
        if !self.has_edges() {
            return Err("The current graph instance does not have any edge.".to_string());
        }
        Ok(())
    }

    /// Raises an error if the graph does not have any node.
    ///
    /// # Example
    /// In order to validate a graph instance, you can use:
    ///
    /// ```rust
    /// # let graph_with_nodes = graph::test_utilities::load_ppi(false, false, true, true, false, false);
    /// # let graph_without_nodes = graph::test_utilities::load_empty_graph(false);
    /// assert!(graph_with_nodes.must_have_nodes().is_ok());
    /// assert!(graph_without_nodes.must_have_nodes().is_err());
    /// ```
    pub fn must_have_nodes(&self) -> Result<(), String> {
        if !self.has_nodes() {
            return Err("The current graph instance does not have any node.".to_string());
        }
        Ok(())
    }
}
