use super::*;
use graph::NodeT;
use numpy::PyArray2;

struct ThreadSafe<'a, T> {
    t: &'a PyArray2<T>,
}

unsafe impl<'a, T> Sync for ThreadSafe<'a, T> {}

#[pymethods]
impl EnsmallenGraph {
    #[text_signature = "($self, verbose)"]
    /// Returns set of edges forming the spanning tree of given graph.
    ///
    /// Parameters
    /// ------------------------
    /// verbose: bool = True,
    ///     Wether to show a loading bar.
    ///
    /// Raises
    /// ------------------------
    /// ValueError,
    ///     If the given graph is not undirected.
    ///
    /// Returns
    /// ------------------------
    /// Numpy array of tuples of NodeIds forming the spanning tree.
    ///
    /// References
    /// ------------------------
    /// This is the implementaiton of the algorithm presented in the paper
    /// A Fast, Parallel Spanning Tree Algorithm for Symmetric Multiprocessors
    /// by David A. Bader and Guojing Cong.
    fn spanning_arborescence(&self, verbose: Option<bool>) -> PyResult<Py<PyArray2<NodeT>>> {
        let py = pyo3::Python::acquire_gil();
        let (edges_number, iter) =
            pe!(self.graph.spanning_arborescence(verbose.unwrap_or(true)))?;
        let array = ThreadSafe {
            t: PyArray2::new(py.python(), [edges_number, 2], false),
        };
        unsafe {
            iter.enumerate().for_each(|(index, (src, dst))| {
                *(array.t.uget_mut([index, 0])) = src;
                *(array.t.uget_mut([index, 1])) = dst;
            });
        }
        Ok(array.t.to_owned())
    }
}
