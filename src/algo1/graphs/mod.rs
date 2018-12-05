#[derive(Clone)]
pub struct AdjacencyList {
  pub node_count: usize,
  pub edge_count: usize,
  nodes: Vec<Vec<u32>>,
}

impl AdjacencyList {
  pub fn from_vector(mut adjacency_lists: Vec<Vec<u32>>) -> AdjacencyList {
    let mut node_count: usize = 0;
    let mut edge_count: usize = 0;
    for (i, vector) in adjacency_lists.iter_mut().enumerate() {
      let node_num = vector.remove(0);
      assert_eq!(node_num, (i + 1) as u32);
      edge_count += vector.len();
      node_count += 1;
    }
    AdjacencyList {
      node_count,
      edge_count,
      nodes: adjacency_lists,
    }
  }

  pub fn print(&self) {
    for (i, n) in self.nodes.iter().enumerate() {
      println!("{}: {:?}", i + 1, n);
    }
  }

  pub fn collapse_edge(&mut self, edge_num: usize) {
    let (node1, node2) = self.find_edge(edge_num);
    self.combine_nodes(node1, node2);
    self.repoint_edges(node1, node2);
    self.remove_circular_edges(node1);
    self.node_count -= 1;
  }

  pub fn combine_nodes(&mut self, node1_idx: usize, node2_idx: usize) {
    let collapsing_edges = self.nodes[node2_idx][..].to_vec();
    self.nodes[node1_idx].extend(collapsing_edges);
    self.nodes[node2_idx].clear();
  }

  pub fn repoint_edges(&mut self, to_node_idx: usize, from_node_idx: usize) {
    let from_node = (from_node_idx + 1) as u32;
    let to_node = (to_node_idx + 1) as u32;
    for node in self.nodes.iter_mut() {
      for edge in node.iter_mut() {
        if edge == &from_node {
          *edge = to_node;
        }
      }
    }
  }

  pub fn remove_circular_edges(&mut self, node_idx: usize) {
    let node_num = (node_idx + 1) as u32;
    let drained: Vec<u32> = self.nodes[node_idx]
      .drain_filter(|&mut n| n == node_num)
      .collect();
    self.edge_count -= drained.len();
  }

  pub fn find_edge(&self, edge_num: usize) -> (usize, usize) {
    assert!(edge_num < self.edge_count);
    let mut total: usize = 0;
    for (i, node) in self.nodes.iter().enumerate() {
      if (total + node.len()) <= edge_num {
        total += node.len();
      } else {
        return (i, node[edge_num - total] as usize - 1);
      }
    }
    panic!("Didn't find an edge :/");
  }
}
