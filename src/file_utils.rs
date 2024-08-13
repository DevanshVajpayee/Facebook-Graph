//file_utils.rs
use std::io::{self, BufRead};
use std::fs::File;
use crate::graph::Vertex;
type ListOfEdges = Vec<(Vertex, Vertex)>;
pub fn read_edges_from_file(file_path: &str) -> io::Result<ListOfEdges> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut edges = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            if let (Some(u_str), Some(v_str)) = (parts.next(), parts.next()) {
                if let (Ok(u), Ok(v)) = (u_str.parse::<Vertex>(), v_str.parse::<Vertex>()) {
                    edges.push((u, v));
                }
            }
        }
    }
    Ok(edges)
}
