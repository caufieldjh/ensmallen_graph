"""
This file offers the methods to automatically retrieve the graph Prevotella bivia DNF00188.

The graph is automatically retrieved from the STRING repository. 

Report
---------------------
At the time of rendering these methods (please see datetime below), the graph
had the following characteristics:

Datetime: 2021-02-03 21:18:54.568337

The undirected graph Prevotella bivia DNF00188 has 2037 nodes and 173876 weighted
edges, of which none are self-loops. The graph is dense as it has a density of 0.08385
and has 16 connected components, where the component with most nodes has 2002 nodes
and the component with the least nodes has 2 nodes. The graph median node degree
is 158, the mean node degree is 170.72, and the node degree mode is 1. The top 5
most central nodes are 1287476.HMPREF1651_00805 (degree 838), 1287476.HMPREF1651_05290
(degree 740), 1287476.HMPREF1651_05565 (degree 732), 1287476.HMPREF1651_09585 (degree
617) and 1287476.HMPREF1651_02580 (degree 600).


References
---------------------
Please cite the following if you use the data:

@article{szklarczyk2019string,
    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
    journal={Nucleic acids research},
    volume={47},
    number={D1},
    pages={D607--D613},
    year={2019},
    publisher={Oxford University Press}
}


Usage example
----------------------
The usage of this graph is relatively straightforward:

.. code:: python

    # First import the function to retrieve the graph from the datasets
    from ensmallen_graph.datasets.string import PrevotellaBiviaDnf00188

    # Then load the graph
    graph = PrevotellaBiviaDnf00188()

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
from ..automatic_graph_retrieval import AutomaticallyRetrievedGraph
from ...ensmallen_graph import EnsmallenGraph  # pylint: disable=import-error


def PrevotellaBiviaDnf00188(
    directed: bool = False,
    verbose: int = 2,
    cache_path: str = "graphs/string"
) -> EnsmallenGraph:
    """Return new instance of the Prevotella bivia DNF00188 graph.

    The graph is automatically retrieved from the STRING repository. 

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

    Returns
    -----------------------
    Instace of Prevotella bivia DNF00188 graph.

	Report
	---------------------
	At the time of rendering these methods (please see datetime below), the graph
	had the following characteristics:
	
	Datetime: 2021-02-03 21:18:54.568337
	
	The undirected graph Prevotella bivia DNF00188 has 2037 nodes and 173876 weighted
	edges, of which none are self-loops. The graph is dense as it has a density of 0.08385
	and has 16 connected components, where the component with most nodes has 2002 nodes
	and the component with the least nodes has 2 nodes. The graph median node degree
	is 158, the mean node degree is 170.72, and the node degree mode is 1. The top 5
	most central nodes are 1287476.HMPREF1651_00805 (degree 838), 1287476.HMPREF1651_05290
	(degree 740), 1287476.HMPREF1651_05565 (degree 732), 1287476.HMPREF1651_09585 (degree
	617) and 1287476.HMPREF1651_02580 (degree 600).
	


	References
	---------------------
	Please cite the following if you use the data:
	
	@article{szklarczyk2019string,
	    title={STRING v11: protein--protein association networks with increased coverage, supporting functional discovery in genome-wide experimental datasets},
	    author={Szklarczyk, Damian and Gable, Annika L and Lyon, David and Junge, Alexander and Wyder, Stefan and Huerta-Cepas, Jaime and Simonovic, Milan and Doncheva, Nadezhda T and Morris, John H and Bork, Peer and others},
	    journal={Nucleic acids research},
	    volume={47},
	    number={D1},
	    pages={D607--D613},
	    year={2019},
	    publisher={Oxford University Press}
	}
	


	Usage example
	----------------------
	The usage of this graph is relatively straightforward:
	
	.. code:: python
	
	    # First import the function to retrieve the graph from the datasets
	    from ensmallen_graph.datasets.string import PrevotellaBiviaDnf00188
	
	    # Then load the graph
	    graph = PrevotellaBiviaDnf00188()
	
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
        "PrevotellaBiviaDnf00188",
        directed=directed,
        verbose=verbose,
        cache_path=cache_path,
        dataset="string"
    )()
