use algo1::graphs::AdjacencyList;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_vec_from_file(path: &str) -> Vec<u32> {
  let mut vec: Vec<u32> = vec![];
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  for line in reader.lines() {
    let number_str = line.unwrap();
    vec.push(number_str.parse::<u32>().unwrap());
  }
  vec
}

pub fn load_graph_from_file(path: &str) -> AdjacencyList {
  let mut vec: Vec<Vec<u32>> = vec![];
  let file = File::open(path).unwrap();
  let reader = BufReader::new(file);
  for line in reader.lines() {
    let number_list = line.unwrap();
    let node_adj_list: Vec<u32> = number_list
      .split("\t")
      .filter_map(|s| s.parse::<u32>().ok())
      .collect();
    vec.push(node_adj_list);
  }
  AdjacencyList::from_vector(vec)
}
