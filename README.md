# Facebook Friends Graph Analysis

This project analyzes the structure and relationships within a Facebook friends network using graph theory. The dataset, sourced from the facebook_combined.txt file, represents users as nodes and friendships as edges in an undirected graph.

## Key Features:

**Graph Construction:** The project reads and processes the Facebook dataset to create an undirected graph, with each node representing a user and each edge representing a friendship.

**Breadth-First Search (BFS):** Implements BFS to compute distances from a randomly selected starting node to all other nodes in the graph.

**Graph Metrics:** Calculates various metrics, including the average distance between nodes, the total number of connections for each node, mutual friends for specific nodes, and identifies the most and least connected users in the network.

**Modular Design:** The project is modular, with separate files handling different aspects of the analysis, ensuring code organization and reusability.

**Comprehensive Testing:** Includes robust testing to ensure the correctness of core functions, such as BFS distance calculation, average number of friends, and mutual friends counting.
This project is a practical application of graph theory, demonstrating how social networks can be analyzed to uncover interesting insights about user connectivity and relationships.

## Output:

### Network statistics:
- **Total Nodes:** 4039
- **Total Edges:** 88234
- **Average Friends per Person(Edges per Node):** 21

### Graph analysis:
- **Average Distance Between Individuals:** 3.53. This indicates the typical path length or
number of connections required to navigate from one random person to another. This
metric is called "six degrees of separation", it showcases the network's overall
interconnectedness. It takes ~4 friendships to traverse from any person to another
within this network.
- **Mutual Friends:** The calculation of mutual friends for a specific random vertex, such as
the 253 mutual friends identified for vertex 1010, identifies a significant aspect of the
graph. This metric represents instances where two individuals connected to a common
friend.
- Node 107 emerges as a 'Social Butterfly,' representing a highly connected and influential
individual within the network.
- Node 11 appears as a 'lonely node,' indicating a comparatively isolated individual within
the network structure.
- A graph density of 0.0108 indicates that the actual number of edges in the graph
represents approximately 1.08% of the total possible edges that could exist in a complete
graph with the same number of vertices. This level of density suggests a relatively sparse
graph.
- **Data validation:** Checks whether graph is connected.
Within a network of approximately 4000 individuals, consistently high mutual friend counts
across various vertices signify a network structure that's relatively open and interconnected.
