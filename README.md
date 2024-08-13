# Facebook Friends Graph Analysis

This project analyzes the structure and relationships within a Facebook friends network using graph theory. The dataset, sourced from the facebook_combined.txt file, represents users as nodes and friendships as edges in an undirected graph.

Key Features:
Graph Construction: The project reads and processes the Facebook dataset to create an undirected graph, with each node representing a user and each edge representing a friendship.
Breadth-First Search (BFS): Implements BFS to compute distances from a randomly selected starting node to all other nodes in the graph.
Graph Metrics: Calculates various metrics, including the average distance between nodes, the total number of connections for each node, mutual friends for specific nodes, and identifies the most and least connected users in the network.
Modular Design: The project is modular, with separate files handling different aspects of the analysis, ensuring code organization and reusability.
Comprehensive Testing: Includes robust testing to ensure the correctness of core functions, such as BFS distance calculation, average number of friends, and mutual friends counting.
This project is a practical application of graph theory, demonstrating how social networks can be analyzed to uncover interesting insights about user connectivity and relationships.
