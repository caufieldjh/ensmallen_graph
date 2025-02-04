"""Sub-module handling the retrieval and building of graphs from NetworkRepository."""
from typing import List, Dict
import os
import pandas as pd
import numpy as np
import requests
from glob import glob
from bs4 import BeautifulSoup
from userinput import userinput, set_validator, set_recoverer, clear
from .graph_repository import GraphRepository
from .custom_exceptions import UnsupportedGraphException


class NetworkRepositoryGraphRepository(GraphRepository):

    def __init__(self):
        """Create new NetworkRepository Graph Repository object."""
        super().__init__()
        self._base_url = "http://nrvis.com/download/data/{graph_type}/{graph_name}.zip"
        self._graph_page_url = "http://networkrepository.com/{}.php"
        self._organisms = pd.read_html(
            "http://networkrepository.com/networks.php"
        )[0]
        self._headers = {
            'User-Agent': 'My User Agent 1.0',
            'From': 'luca.cappelletti94@gmail.com'  # This is another valid field
        }

    def build_stored_graph_name(self, partial_graph_name: str) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        partial_graph_name: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        stored_graph_name = "".join([
            term.capitalize()
            for term in partial_graph_name.split("-")
        ])
        if stored_graph_name[0].isnumeric():
            stored_graph_name = "Graph{}".format(stored_graph_name)
        return stored_graph_name

    def get_formatted_repository_name(self) -> str:
        """Return formatted repository name."""
        return "NetworkRepository"

    def get_graph_name(self, graph_data) -> str:
        """Return built graph name.

        Parameters
        -----------------------
        graph_data: str,
            Partial graph name to be built.

        Returns
        -----------------------
        Complete name of the graph.
        """
        return graph_data["Graph Name"]

    def get_graph_urls(self, graph_data) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_data,
            Graph data to use to retrieve the URLs.

        Returns
        -----------------------
        The urls list from where to download the graph data.
        """
        return [self._base_url.format(
            graph_type=graph_data["Type"],
            graph_name=self.get_graph_name(graph_data)
        )]

    def get_graph_soup(self, graph_name: str) -> BeautifulSoup:
        """Return soup for given graph.

        Parameters
        -----------------------
        graph_name,
            Name of the graph to retrieve.

        Returns
        -----------------------
        Beautiful soup for given graph.
        """
        try:
            content = requests.get(
                self._graph_page_url.format(graph_name),
                headers=self._headers,
                timeout=10
            ).text
        except Exception:
            content = ""
        return BeautifulSoup(
            content,
            "lxml"
        )

    def get_graph_citations(self, graph_data) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_data,
            Graph data to use to retrieve the citations.

        Returns
        -----------------------
        Citations relative to the given Network Repository graph.
        """
        skitter_target = "Skitter"
        whois_target = "WHOIS"
        routeviews_target = "RouteViews"
        targets = [
            "The Network Data Repository",
            skitter_target,
            whois_target,
            routeviews_target
        ]
        baseline_citations = [
            open(
                "{}/models/network_repository_citation.bib".format(
                    os.path.dirname(os.path.abspath(__file__))
                ),
                "r"
            ).read()
        ]

        soup = self.get_graph_soup(self.get_graph_name(graph_data))
        citations = [
            reference.text.strip()
            for reference in soup.find_all("blockquote")
        ]
        if any(skitter_target in cite for cite in citations):
            baseline_citations.append(open(
                "{}/models/skitter.bib".format(
                    os.path.dirname(os.path.abspath(__file__))
                ),
                "r"
            ).read())

        if any(whois_target in cite for cite in citations):
            baseline_citations.append(open(
                "{}/models/whois.bib".format(
                    os.path.dirname(os.path.abspath(__file__))
                ),
                "r"
            ).read())

        if any(routeviews_target in cite for cite in citations):
            baseline_citations.append(open(
                "{}/models/routeviews.bib".format(
                    os.path.dirname(os.path.abspath(__file__))
                ),
                "r"
            ).read())

        return baseline_citations + [
            reference
            for reference in citations
            if not any(
                target in reference.text.strip()
                for target in targets
            )
        ]

    def get_graph_paths(self, graph_name: str, urls: List[str]) -> List[str]:
        """Return url for the given graph.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to retrievel URLs for.
        urls: List[str],
            Urls from where to download the graphs.

        Returns
        -----------------------
        The paths where to store the downloaded graphs.
        """
        return None

    def check_nominal_download(
        self,
        download_report: pd.DataFrame
    ) -> bool:
        """Return boolean representing if everything went ok.

        Parameters
        -----------------------
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        Boolean representing if everything went ok.
        """
        return "extraction_destination" in download_report.columns

    def is_graph_unsupported(self, graph_name: str) -> bool:
        """Return boolean representing if graph is known to be unsupported.

        Parameters
        -----------------------
        graph_name: str,
            Name of graph to check if it is unsupported.

        Returns
        -----------------------
        Wether the graph is known to be unsupported.
        """
        return (
            any(
                graph_name.startswith(term)
                for term in ("soc-gemsec-", "fb-pages-", "rec-", "ia-", "reptilia-", "mammalia-", "insecta-")
            ) or
            any(
                graph_name.endswith(term)
                for term in ("-trapping", "-ratings")
            ) or
            super().is_graph_unsupported(graph_name)
        )

    def build_graph_parameters(
        self,
        graph_name: str,
        edge_path: str,
        node_path: str = None,
    ) -> Dict:
        """Return dictionary with kwargs to load graph.

        Parameters
        ---------------------
        graph_name: str,
            Name of the graph to load.
        edge_path: str,
            Path from where to load the edge list.
        node_path: str = None,
            Optionally, path from where to load the nodes.

        Returns
        -----------------------
        Dictionary to build the graph object.
        """
        data = self.load_dataframe(edge_path)
        soup = self.get_graph_soup(graph_name)
        is_weighted = "<td><b>Edge weights</b></td><td>Weighted</td>" in str(
            soup)

        if (
            len(data.columns) == 3 and
            data[0].dtype == np.int64 and
            # len(data) != len(data[0].unique()) and
            data[1].dtype == np.int64 and
            # len(data) != len(data[1].unique()) and
            (data[2] == 1).all()
        ):
            sources_column_number = 0
            destinations_column_number = 1
            weights_column_number = None
            edge_types_column_number = None
        if (
            len(data.columns) == 3 and
            data[0].dtype == np.int64 and
            # len(data) != len(data[0].unique()) and
            data[1].dtype == np.int64 and
            # len(data) != len(data[1].unique()) and
            graph_name[0] == "G" and
            graph_name[1:].isnumeric() and
            {1, -1} == set(data[2].values)
        ):
            sources_column_number = 0
            destinations_column_number = 1
            weights_column_number = None
            edge_types_column_number = 2
        elif (
            len(data.columns) == 4 and
            data[0].dtype == np.int64 and
            # len(data) != len(data[0].unique()) and
            data[1].dtype == np.int64 and
            # len(data) != len(data[1].unique()) and
            (data[2] == 1).all() and
            data[3].isna().all()
        ):
            sources_column_number = 0
            destinations_column_number = 1
            weights_column_number = None
            edge_types_column_number = None
        elif (
            len(data.columns) == 3 and
            data[0].dtype == np.int64 and
            # len(data) != len(data[0].unique()) and
            data[1].dtype == np.int64 and
            # len(data) != len(data[1].unique()) and
            (data[2].dtype == np.float64 or is_weighted)
        ):
            sources_column_number = 0
            destinations_column_number = 1
            weights_column_number = 2
            edge_types_column_number = None
        elif (
            len(data.columns) == 2 and
            data[0].dtype == np.int64 and
            # len(data) != len(data[0].unique()) and
            data[1].dtype == np.int64  # and
            #len(data) != len(data[1].unique())
        ):
            sources_column_number = 0
            destinations_column_number = 1
            weights_column_number = None
            edge_types_column_number = None
        elif (
            len(data.columns) == 3 and
            all([
                data[col].dtype == np.int64
                for col in data.columns
            ]) and
            # len(data) != len(data[0].unique()) and
            # len(data) != len(data[1].unique()) and
            ((data[2] == 0) | (data[2] > 10_000_000)).all()
        ):
            raise UnsupportedGraphException(
                "Currently graphs with timestamps are not supported."
            )
            # sources_column_number = 0
            # destinations_column_number = 1
            # weights_column_number = 2
            # edge_types_column_number = None
        elif (
            len(data.columns) == 4 and
            all([
                data[col].dtype == np.int64
                for col in [0, 1]
            ]) and
            all([
                data[col].dtype == np.float64
                for col in [2, 3]
            ])
            # len(data) != len(data[0].unique()) and
            # len(data) != len(data[1].unique()) and
            #((data[2] == 0) | (data[2] > 10_000_000)).all()
        ):
            raise UnsupportedGraphException(
                "Currently graphs with timestamps are not supported."
            )
            # sources_column_number = 0
            # destinations_column_number = 1
            # weights_column_number = 2
            # edge_types_column_number = None
        elif (
            len(data.columns) == 4 and
            all([
                data[col].dtype == np.int64
                for col in data.columns
            ]) and
            len(data) != len(data[0].unique()) and
            len(data) != len(data[1].unique()) and
            ((data[3] == 0) | (data[3] > 10_000_000)).all()
        ):
            raise UnsupportedGraphException(
                "Currently graphs with timestamps are not supported."
            )
            # sources_column_number = 0
            # destinations_column_number = 1
            # weights_column_number = 3
            # edge_types_column_number = None
        else:
            print(graph_name)
            self.display_dataframe_preview(data)
            sources_column_number = userinput(
                "sources_column_number",
                default=0,
                validator="positive_integer",
                sanitizer="integer",
                auto_clear=False
            )
            destinations_column_number = userinput(
                "destinations_column_number",
                default=1,
                validator="positive_integer",
                sanitizer="integer",
                auto_clear=False
            )
            if len(data.columns) > 2:
                has_weight = userinput(
                    "has_weight",
                    default="yes",
                    validator="human_bool",
                    sanitizer="human_bool",
                    auto_clear=False
                )
                if has_weight:
                    weights_column_number = userinput(
                        "weights_column_number",
                        default=2,
                        validator="positive_integer",
                        sanitizer="integer",
                        auto_clear=False
                    )
                else:
                    weights_column_number = None
            else:
                weights_column_number = None

            if len(data.columns) == 3 and weights_column_number is None or len(data.columns) > 3:
                has_edge_type = userinput(
                    "has_edge_type",
                    default="yes",
                    validator="human_bool",
                    sanitizer="human_bool",
                    auto_clear=False
                )
                if has_edge_type:
                    edge_types_column_number = userinput(
                        "edge_types_column_number",
                        default=2 if weights_column_number is None else 3,
                        validator="positive_integer",
                        sanitizer="integer",
                        auto_clear=False
                    )
                else:
                    edge_types_column_number = None
            else:
                edge_types_column_number = None
            clear()

        if weights_column_number is not None and (data[weights_column_number] <= 0).any():
            raise UnsupportedGraphException(
                "Found illegal non-positive weight in graph {}!".format(graph_name))

        if weights_column_number is not None and data[weights_column_number].isna().any():
            default_weight = 1.0
        else:
            default_weight = None

        if node_path is not None:
            data = self.load_dataframe(node_path)
            if (
                len(data.columns) == 2 and
                all([
                    data[col].dtype == np.int64
                    for col in data.columns
                ]) and
                len(data) != len(data[1].unique()) and
                len(data[1].unique()) < 100
            ):
                nodes_column_number = 0
                node_types_column_number = 1
            else:
                print(graph_name)
                self.display_dataframe_preview(data)
                nodes_column_number = userinput(
                    "nodes_column_number",
                    default=0,
                    validator="positive_integer",
                    sanitizer="integer",
                    auto_clear=False
                )
                if len(data.columns) > 1:
                    try:
                        node_types_column_number = userinput(
                            "node_types_column_number",
                            default=1,
                            validator="positive_integer",
                            sanitizer="integer",
                            auto_clear=False
                        )
                    except KeyboardInterrupt:
                        node_types_column_number = None
                else:
                    node_types_column_number = None
                clear()
        else:
            nodes_column_number = None
            node_types_column_number = None

        return {
            **super().build_graph_parameters(
                graph_name,
                edge_path,
                node_path
            ),
            "edge_header": False,
            "node_header": False,
            "default_weight": default_weight,
            "sources_column_number": sources_column_number,
            "destinations_column_number": destinations_column_number,
            "weights_column_number": weights_column_number,
            "edge_types_column_number": edge_types_column_number,
            "nodes_column_number": nodes_column_number,
            "node_types_column_number": node_types_column_number,
        }

    def get_graph_list(self) -> List[str]:
        """Return list of graph names."""
        return [
            row
            for _, row in self._organisms.iterrows()
        ]

    def get_description(self, graph_name: str) -> str:
        """Return description to be added to model file.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.

        Returns
        -----------------------
        description.
        """
        return ""

    def get_node_list_path(
        self,
        graph_name: str,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the node files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the node files.
        """
        candidate_file_name = None
        directory = download_report.extraction_destination[0]
        file_names = [
            file_name
            for file_name in os.listdir(directory)
            if "readme" not in file_name.lower()
        ]
        if len(file_names) == 1:
            return None
        if any(
            file_name.endswith(".graph_idx")
            for file_name in file_names
        ):
            raise UnsupportedGraphException(
                "The graph file format with graph_idx files are not currently supported!"
            )
        for file_name in file_names:
            for target in ("node", "types"):
                if target in file_name:
                    candidate_file_name = file_name
                    break
        if candidate_file_name is not None and any(
            candidate_file_name.endswith(ext)
            for ext in (
                ".node_labels",
                ".nodes",
                ".types"
            )
        ):
            return os.path.join(directory, candidate_file_name)
        print(file_names)
        file_name = userinput(
            "node_list_path",
            default=candidate_file_name,
            cache=False,
            validator=set_validator(file_names),
            recoverer=set_recoverer(file_names),
            auto_clear=True
        )
        return os.path.join(directory, file_name)

    def get_edge_list_path(
        self,
        graph_name: str,
        download_report: pd.DataFrame
    ) -> str:
        """Return path from where to load the edge files.

        Parameters
        -----------------------
        graph_name: str,
            Name of the graph.
        download_report: pd.DataFrame,
            Report from downloader.

        Returns
        -----------------------
        The path from where to load the edge files.
        """
        candidate_file_name = None
        directory = download_report.extraction_destination[0]
        file_names = [
            file_name
            for file_name in os.listdir(directory)
            if "readme" not in file_name.lower()
        ]
        if any(
            file_name.endswith(".graph_idx")
            for file_name in file_names
        ):
            raise UnsupportedGraphException(
                "The graph file format with graph_idx files are not currently supported!"
            )
        if len(file_names) == 1:
            return os.path.join(directory, file_names[0])
        for file_name in file_names:
            for target in ("edge", ".mtx"):
                if target in file_name:
                    candidate_file_name = file_name
                    break
        if candidate_file_name is not None and any(
            candidate_file_name.endswith(ext)
            for ext in (
                ".edges",
                ".mtx"
            )
        ):
            return os.path.join(directory, candidate_file_name)
        file_name = userinput(
            "edge_list_path",
            default=candidate_file_name,
            cache=False,
            validator=set_validator(file_names),
            recoverer=set_recoverer(file_names),
            auto_clear=True
        )
        return os.path.join(directory, file_name)
