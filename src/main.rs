// main.rs
mod graph;
mod rand_utils;
mod file_utils;
use graph::Graph;
use rand_utils::generate_random_start_vertex;
use file_utils::read_edges_from_file;
fn main() {
    if let Ok(edges) = read_edges_from_file("facebook_combined.txt") {
        let max_vertex = edges.iter().map(|&(u, v)| u.max(v)).max().unwrap_or(0) + 1;
        let graph = Graph::create_undirected(max_vertex, &edges);
        let start_vertex = generate_random_start_vertex(max_vertex);
        let total_edges: usize = edges.len();
        let distances = graph.bfs_distances(start_vertex);
        //println!("Graph: "); //Remove comment(s) if you want to see the graph
        //graph.print();
        let valid_distances: Vec<u32> = distances.into_iter().filter_map(|d| d).collect();
        let total_distance: u32 = valid_distances.iter().sum();
        let average_distance = total_distance as f64 / valid_distances.len() as f64;
        let sum_of_friends = graph.average_friends();
        let start_vertex = generate_random_start_vertex(max_vertex);
        let mutual_friends_count = graph.calculate_mutual_friends(start_vertex);
        let density = Graph::graph_density(total_edges, max_vertex);     
        println!("Average Number of Friends per Person in the graph: {}", sum_of_friends);
        println!("Number of mutual friends for vertex {}: {}", start_vertex, mutual_friends_count);
        println!("Average Distance: {:.2}", average_distance);
        println!("Graph Density: {:.4}", density);
        if let Some(most_connected) = graph.most_connected_node() {
            println!("Social Butterfly: {}", most_connected);
        } else {
            println!("Graph is empty.");
        }
        if let Some(least_connected) = graph.least_connected_node() {
            println!("You look like a lonely node: {}", least_connected);
        } else {
            println!("Graph is empty.");
        }
        if graph.is_connected() {
            println!("The graph is connected.");
        } else {
            println!("The graph is not connected.");
        }  
    } else {
        println!("Error reading edges from file.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs_distances() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_undirected(4, &edges);
        let distances = graph.bfs_distances(0);
        // The graph looks like this:
        // 0 -- 1
        // |  \ |
        // 2 -- 3
        assert_eq!(distances, vec![Some(0), Some(1), Some(2), Some(3)]);
    }
    #[test]
    fn test_average_friends() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
        let graph = Graph::create_undirected(4, &edges);
        // The graph looks like this:
        // 0 -- 1
        // | \/ |
        // 2 -- 3
        assert_eq!(graph.average_friends(), 1);
    }
    #[test]
    fn test_calculate_mutual_friends() {
        let edges = vec![(0, 1), (1, 2), (2, 3), (3, 0), (1,3)]; 
        let graph = Graph::create_undirected(4, &edges);
        assert_eq!(graph.calculate_mutual_friends(0), 1); 
        assert_eq!(graph.calculate_mutual_friends(1), 2); 
    }
    #[test]
    fn test_most_and_least_connected_nodes() {
        let edges = vec![(0, 1), (0, 2), (0, 3), (1, 3)];
        let graph = Graph::create_undirected(4, &edges);
        // The graph looks like this:
        // 0 -- 1
        // |  \ |
        // 2 -- 3
        assert_eq!(graph.most_connected_node(), Some(0));
        assert_eq!(graph.least_connected_node(), Some(2)); 
    }
}
//~165 lines of code
//Takes ~10 seconds to check if graph is connected.