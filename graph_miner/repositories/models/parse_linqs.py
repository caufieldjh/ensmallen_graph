"""Methods to parse the incidence matrix from LINQS."""
import re
import os
import pandas as pd
import numpy as np
from ensmallen_graph import EnsmallenGraph
from tqdm.auto import tqdm


def get_words_data(graph: EnsmallenGraph) -> pd.DataFrame:
    """Return dataframe with words features.
    
    Parameters
    --------------------
    graph: EnsmallenGraph,
        Graph containing the words features to be extracted.

    Returns
    --------------------
    Pandas DataFrame with words features as columns and nodes as rows.
    """
    word_node_type = graph.get_node_type_names().index("Word")
    weights = graph.get_weights() if graph.has_weights() else None
    return pd.DataFrame({
        node_name: {
            graph.get_node_name(source): weights[graph.get_edge_id_with_type_by_node_ids(source, node_id)] if graph.has_weights() else 1
            for source in graph.get_filtered_neighbours(node_id)
        }
        for node_id, node_name in enumerate(tqdm(graph.get_node_names(), desc="Extracting words features"))
        if graph.get_node_type(node_id) == word_node_type
    }).fillna(0)


def parse_linqs_pubmed_incidence_matrix(
    cites_path: str,
    content_path: str,
    edge_list_path: str,
    node_list_path: str
):
    """Parse PubMed incidence matrix and generates proper edge list and node file.

    Parameters
    -------------------
    cites_path: str,
        Path from where to load the cites file.
    content_path: str,
        Path from where to load the content file.
    edge_list_path: str,
        Path where to store the edge list.
    node_list_path: str,
        Path where to store the node list.
    """
    # Creating directories
    os.makedirs(os.path.dirname(edge_list_path), exist_ok=True)
    os.makedirs(os.path.dirname(node_list_path), exist_ok=True)
    # Loading data
    with open(content_path) as f:
        content = f.read()
    with open(cites_path) as f:
        cites = f.read()

    separator = "\t"

    edge_list_file = open(edge_list_path, "w")
    node_list_file = open(node_list_path, "w")

    unique_words = set()
    edge_regex = re.compile(r"paper:(\d+)")
    node_regex = re.compile(r"(\d+)\s+label=(\d+)")
    word_regex = re.compile(r"w-(\w+)=(\S+)")

    labels = [
        "Diabetes Mellitus, Experimental",
        "Diabetes Mellitus Type 1",
        "Diabetes Mellitus Type 2"
    ]

    edge_list_file.write(
        separator.join(("subject", "object", "edge_type", "weight")) + "\n"
    )
    node_list_file.write(separator.join(("id", "node_type")) + "\n")

    for line in cites.split("\n")[2:-1]:
        edge = re.findall(edge_regex, line)
        if len(edge) != 2:
            continue
        # Writing the edges between papers and papers
        edge_list_file.write(separator.join((*edge, "Paper2Paper", "")) + "\n")

    for line in content.split("\n")[2:]:
        vals = re.findall(node_regex, line)
        if len(vals) != 1:
            break

        src, label = vals[0]
        # Writing node and its node type to the node list.
        node_list_file.write(separator.join(
            (src, labels[int(label)-1])) + "\n")

        # Writing the edges between papers and words
        for (word, weight) in re.findall(word_regex, line):
            edge_list_file.write(
                separator.join((src, word, "Paper2Word", weight)) + "\n")
            # Add word to the unique words set
            unique_words.add(word)

    # Writing the nodes representing words
    for word in unique_words:
        node_list_file.write(separator.join((word, "Word")) + "\n")

    edge_list_file.close()
    node_list_file.close()


def parse_linqs_incidence_matrix(
    cites_path: str,
    content_path: str,
    edge_list_path: str,
    node_list_path: str
):
    """Parse Cora and Citeseer incidence matrix and generates proper edge list and node file.

    Parameters
    -------------------
    cites_path: str,
        Path from where to load the cites file.
    content_path: str,
        Path from where to load the content file.
    edge_list_path: str,
        Path where to store the edge list.
    node_list_path: str,
        Path where to store the node list.
    """
    # Creating directories
    os.makedirs(os.path.dirname(edge_list_path), exist_ok=True)
    os.makedirs(os.path.dirname(node_list_path), exist_ok=True)
    # Loading the content file (incidence matrix)
    content = pd.read_csv(
        content_path,
        sep="\t",
        header=None,
        index_col=0,
        dtype=str
    )
    # Loading the citations file (edge list)
    cities = pd.read_csv(
        cites_path,
        sep="\t",
        header=None,
        dtype=str
    )
    # Standardizing dataframe
    cities.columns = ["subject", "object"]
    cities["edge_type"] = "Paper2Paper"
    # Extract labels from incidence matrix
    labels = content[content.columns[-1]]
    # Removing labels column
    content.drop(columns=content.columns[-1], inplace=True)
    # Converting incidence matrix to edge list
    node_indices, word_indices = np.where(content.values.astype(int) == 1)
    # Create words vector
    words = np.array([
        "word_{}".format(word_id)
        for word_id in np.arange(max(word_indices)+1)
    ])
    # Create the node list
    node_list = pd.concat([
        pd.DataFrame({
            "id": content.index.astype(str),
            "node_type": labels
        }),
        pd.DataFrame({
            "id": words,
            "node_type": "Word"
        }),
        pd.DataFrame({
            "id": list(
                set(cities[["subject", "object"]].values.flatten()
                    ) - set(content.index.astype(str))
            ),
            "node_type": "Unknown"
        })
    ]).reset_index(drop=True)
    # Create the edge list
    edge_list = pd.concat([
        cities,
        pd.DataFrame({
            "subject": content.index[node_indices].astype(str),
            "object": words[word_indices],
            "edge_type": "Paper2Word"
        })
    ]).reset_index(drop=True)
    # Storing the generated node list
    node_list.to_csv(node_list_path, sep="\t", index=False)
    # Storing the generated edge list
    edge_list.to_csv(edge_list_path, sep="\t", index=False)
