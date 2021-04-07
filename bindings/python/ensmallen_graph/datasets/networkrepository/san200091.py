"""
This file offers the methods to automatically retrieve the graph san200-0-9-1.

The graph is automatically retrieved from the NetworkRepository repository. 



Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 22:56:32.741439

The undirected graph san200-0-9-1 has 200 nodes and 17910 unweighted edges,
of which none are self-loops. The graph is extremely dense as it has a
density of 0.90000 and is connected, as it has a single component. The
graph median node degree is 180, the mean node degree is 179.10 and the
node degree mode is 181. The top 5 most central nodes are 28 (degree 191),
107 (degree 190), 168 (degree 189), 83 (degree 189) and 82 (degree 189).


References
---------------------
Please cite the following if you use the data:

@inproceedings{nr,
    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
    author={Ryan A. Rossi and Nesreen K. Ahmed},
    booktitle = {AAAI},
    url={http://networkrepository.com},
    year={2015}
}

@misc{dimacs,
        author={{DIMACS}},
        title={DIMACS Challenge},
        note={http://dimacs.rutgers.edu/Challenges/}}

@article{rossi2014coloring,
        title={Coloring Large Complex Networks},
        author={Ryan A. Rossi and Nesreen K. Ahmed},
        booktitle={Social Network Analysis and Mining},
        pages={1--51},
        year={2014}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.networkrepository import San200091

    # Then load the graph
    graph = San200091()

    # Finally, you can do anything with it, for instance, compute its report:
    print(graph)

    # If you need to run a link prediction task with validation,
    # you can split the graph using a connected holdout as follows:
    train_graph, validation_graph = graph.connected_holdout(
        # You can use an 80/20 split the holdout, for example.
        train_size=0.8,
        # The random state is used to reproduce the holdout.
        random_state=42,
        # Wether to show a loading bar.
        verbose=True
    )

    # Remember that, if you need, you can enable the memory-time trade-offs:
    train_graph.enable(
        vector_sources=True,
        vector_destinations=True,
        vector_outbounds=True
    )

    # Consider using the methods made available in the Embiggen package
    # to run graph embedding or link prediction tasks.
"""
from typing import Dict

from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def San200091(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/networkrepository",
    **additional_graph_kwargs: Dict
) -> EnsmallenGraph:
    """Return new instance of the san200-0-9-1 graph.

    The graph is automatically retrieved from the NetworkRepository repository. 

	

    Parameters
    -------------------
    directed: bool = False,
        Wether to load the graph as directed or undirected.
        By default false.
    verbose: int = 2,
        Wether to show loading bars during the retrieval and building
        of the graph.
    cache_path: str = "graphs",
        Where to store the downloaded graphs.
    additional_graph_kwargs: Dict,
        Additional graph kwargs.

    Returns
    -----------------------
    Instace of san200-0-9-1 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 22:56:32.741439
	
	The undirected graph san200-0-9-1 has 200 nodes and 17910 unweighted edges,
	of which none are self-loops. The graph is extremely dense as it has a
	density of 0.90000 and is connected, as it has a single component. The
	graph median node degree is 180, the mean node degree is 179.10 and the
	node degree mode is 181. The top 5 most central nodes are 28 (degree 191),
	107 (degree 190), 168 (degree 189), 83 (degree 189) and 82 (degree 189).
	

	References
	---------------------
	Please cite the following if you use the data:
	
	@inproceedings{nr,
	    title = {The Network Data Repository with Interactive Graph Analytics and Visualization},
	    author={Ryan A. Rossi and Nesreen K. Ahmed},
	    booktitle = {AAAI},
	    url={http://networkrepository.com},
	    year={2015}
	}
	
	@misc{dimacs,
	        author={{DIMACS}},
	        title={DIMACS Challenge},
	        note={http://dimacs.rutgers.edu/Challenges/}}
	
	@article{rossi2014coloring,
	        title={Coloring Large Complex Networks},
	        author={Ryan A. Rossi and Nesreen K. Ahmed},
	        booktitle={Social Network Analysis and Mining},
	        pages={1--51},
	        year={2014}
	}
	

	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.networkrepository import San200091
	
	    # Then load the graph
	    graph = San200091()
	
	    # Finally, you can do anything with it, for instance, compute its report:
	    print(graph)
	
	    # If you need to run a link prediction task with validation,
	    # you can split the graph using a connected holdout as follows:
	    train_graph, validation_graph = graph.connected_holdout(
	        # You can use an 80/20 split the holdout, for example.
	        train_size=0.8,
	        # The random state is used to reproduce the holdout.
	        random_state=42,
	        # Wether to show a loading bar.
	        verbose=True
	    )
	
	    # Remember that, if you need, you can enable the memory-time trade-offs:
	    train_graph.enable(
	        vector_sources=True,
	        vector_destinations=True,
	        vector_outbounds=True
	    )
	
	    # Consider using the methods made available in the Embiggen package
	    # to run graph embedding or link prediction tasks.
    """
    return AutomaticallyRetrievedGraph(
        graph_name="San200091",
        dataset="networkrepository",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        additional_graph_kwargs=additional_graph_kwargs
    )()
