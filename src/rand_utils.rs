//rand_utils.rs
use rand::Rng;
use crate::graph::Vertex;
pub fn generate_random_start_vertex(max_vertex: Vertex) -> Vertex {
    rand::thread_rng().gen_range(0..max_vertex)
}
