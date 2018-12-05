use algo1::rand::thread_rng;
use algo1::rand::prelude::IteratorRandom;

use algo1::graphs::AdjacencyList;

pub fn kargers_min_cut(graph: AdjacencyList) -> Option<usize> {
  let mut min_cut: Option<usize> = None; 
  let num_iterations = determine_num_iterations(&graph.node_count);
  // 100 takes ~2 secs and num_iterations winds up being ~100k. Running this 20
  // times I only got the wrong result once. Seems like 100k is overkill
  for _ in 0..100 {
    let mut mutgraph = graph.clone();
    while &mutgraph.node_count > &2 { // this seems weird
      let edge_to_collapse = select_random_edge(&mutgraph.edge_count);
      mutgraph.collapse_edge(edge_to_collapse);
    }
    match min_cut {
      Some(prev_min) => {
        if prev_min > (mutgraph.edge_count / 2) {
          min_cut = Some(mutgraph.edge_count / 2);
        }
      },
      None => min_cut = Some(mutgraph.edge_count / 2),
    }
  }
  min_cut
}

fn determine_num_iterations(node_count: &usize) -> usize {
  let node_count_ln = (node_count.clone() as f32).ln();
  let n_choose_2 = (node_count * (node_count - 1)) / 2;
  n_choose_2 as usize * node_count_ln as usize
}

fn select_random_edge(num_edges: &usize) -> usize {
  let mut rng = thread_rng();
  // this is fine because it doens't change the derefed mem?
  (0..*num_edges).choose(&mut rng).unwrap()
}

#[cfg(test)]
mod test {
  use super::*;
  use algo1::tests::helpers;

  fn load_hw_graph() -> AdjacencyList {
    helpers::load_graph_from_file("src/algo1/assignment3/assignment3.txt")
  }

  #[test]
  fn solves_simple_min_cut() {
    let adjacency_list: Vec<Vec<u32>> = vec![
      vec![1, 2, 3],
      vec![2, 1, 3, 4],
      vec![3, 1, 2, 4],
      vec![4, 3, 2],
    ];
    let graph = AdjacencyList::from_vector(adjacency_list);

    let min_cut = kargers_min_cut(graph);

    assert_eq!(min_cut.unwrap(), 2);
  }

  #[test]
  fn solves_min_cut_hw() {
    let graph = load_hw_graph();
    let min_cut = kargers_min_cut(graph);

    assert_eq!(min_cut.unwrap(), 17);
  }
}
