// graph.rs
use std::collections::VecDeque;
pub(crate) type Vertex = usize;
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;
#[derive(Debug)]
pub struct Graph {
    n: usize,
    outedges: AdjacencyLists,
}
impl Graph {
    pub fn bfs_distances(&self, start: Vertex) -> Vec<Option<u32>> {
        let mut distance: Vec<Option<u32>> = vec![None; self.n];
        distance[start] = Some(0);
        let mut queue: VecDeque<Vertex> = VecDeque::new();
        queue.push_back(start);
        while let Some(v) = queue.pop_front() {
            for u in &self.outedges[v] {
                if distance[*u].is_none() {
                    distance[*u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(*u);
                }
            }
        }
        distance
    }
    pub fn print(&self) {
        for (i, l) in self.outedges.iter().enumerate() {
            println!("{} {:?}", i, *l);
        }
    }
    pub fn average_friends(&self) -> usize {
        let mut total = 0;
        let num_nodes = self.n;
        for list in &self.outedges {
            total += list.len();
        }
        // Since this is an undirected graph, each edge is counted twice
        // Divide by 2 to get the actual count of unique edges
        total/ (2*num_nodes)
        //Included a function to calculate average friends instead of simply dividing(88324/4039) in order to not hard code.
    }
    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut graph = Graph {
            n,
            outedges: Vec::with_capacity(edges.len()),
        };
        for _ in 0..n {
            graph.outedges.push(Vec::new());
        }
        for &(u, v) in edges {
            add_edge(&mut graph, u, v);
        }
        graph
    }
    pub fn calculate_mutual_friends(&self, vertex: Vertex) -> usize {
        let mut mutual_friends = 0;
        let friends = &self.outedges[vertex];
        for &friend in friends {
            for &friend_of_friend in &self.outedges[friend] {
                if friend_of_friend != vertex && friends.contains(&friend_of_friend) {
                    mutual_friends += 1;
                }
            }
        }
        mutual_friends / 2 // Divide by 2 to count each mutual friend pair once
    }
    pub fn most_connected_node(&self) -> Option<Vertex> {
        // Find the node with the highest degree (number of friends)
        let most_connected_node = (0..self.n)
            .max_by_key(|&v| self.outedges[v].len())
            .map(|v| v);
        most_connected_node
    }
    pub fn least_connected_node(&self) -> Option<Vertex> {
        // Find the node with the lowest degree (number of friends)
        let least_connected_node = (0..self.n)
            .min_by_key(|&v| self.outedges[v].len())
            .map(|v| v);
        least_connected_node
    }
    pub fn graph_density(edges_count: usize, num_nodes: usize) -> f64 {
        let max_possible_edges = (num_nodes * (num_nodes - 1)) / 2;
        edges_count as f64 / max_possible_edges as f64
    }  
    pub fn is_connected(&self) -> bool {
        for node in 0..self.n {
            if self.bfs_distances(node).iter().any(|d| d.is_none()) {
                return false;
            }
        }
        true
    }
}
pub fn add_edge(graph: &mut Graph, u: Vertex, v: Vertex) {
    graph.outedges[u].push(v);
    graph.outedges[v].push(u); // Add the reverse edge for an undirected graph
}