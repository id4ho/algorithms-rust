pub fn quicksort(vector: &mut Vec<u32>) -> usize {
  let indices = (0, vector.len() - 1);
  let mut comparisons: usize = 0; // needed for homework solutions
  let pivot_selector = |_v: &[u32], s: usize, _e: usize| s;
  qs_sub_vec(vector, indices, &mut comparisons, &pivot_selector);
  comparisons
}

fn qs_sub_vec<F>(
  vector: &mut Vec<u32>,
  indices: (usize, usize),
  comps: &mut usize,
  pivot_selector: &F,
) where
  F: Fn(&[u32], usize, usize) -> usize,
{
  let (start, end) = indices;
  if end > start {
    let pivot_idx = pivot_selector(&vector, start, end);
    swap_elements(vector, start, pivot_idx);

    *comps += end - start;
    let pivot_end_idx = partition_vec(vector, start, end);
    let left_partition_idxs = (start, pivot_end_idx.saturating_sub(1));
    let right_partition_idxs = (pivot_end_idx + 1, end);

    qs_sub_vec(vector, left_partition_idxs, comps, pivot_selector);
    qs_sub_vec(vector, right_partition_idxs, comps, pivot_selector);
  }
}

fn partition_vec(vector: &mut Vec<u32>, start: usize, end: usize) -> usize {
  let pivot = vector[start];
  let mut i = start + 1;
  let mut j = start + 1;
  while j <= end {
    if vector[j] <= pivot {
      if j != i {
        swap_elements(vector, i, j);
      }
      i += 1;
    }
    j += 1;
  }
  let correct_pivot_idx = i - 1;
  swap_elements(vector, correct_pivot_idx, start);
  correct_pivot_idx
}

fn swap_elements<T: Copy>(vector: &mut Vec<T>, idx_one: usize, idx_two: usize) {
  let tmp = vector[idx_one];
  vector[idx_one] = vector[idx_two];
  vector[idx_two] = tmp;
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs::File;
  use std::io::{BufRead, BufReader};

  #[test]
  fn sorts_array_in_place() {
    let mut simple = vec![3, 8, 2, 5, 1, 4, 7, 6];

    quicksort(&mut simple);

    assert_eq!(simple, vec![1, 2, 3, 4, 5, 6, 7, 8]);
  }

  #[test]
  fn sorts_slightly_more_complicated_array_in_place() {
    let mut simple = vec![3, 8, 2, 5, 1, 4, 7, 6, 10, 9, 11, 12, 14, 13];

    quicksort(&mut simple);

    assert_eq!(simple, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
  }

  fn load_hw_vector() -> Vec<u32> {
    let mut number_vec: Vec<u32> = vec![];
    let file = File::open("src/algo1/assignment2/assignment2.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
      let number_str = line.unwrap();
      number_vec.push(number_str.parse::<u32>().unwrap());
    }
    number_vec
  }

  #[test]
  fn solves_homework_problem1() {
    let mut vec = load_hw_vector();
    let mut comps: usize = 0;
    let end = vec.len() - 1;
    let pivot_selection = |_v: &[u32], s: usize, _e: usize| s;
    qs_sub_vec(&mut vec, (0, end), &mut comps, &pivot_selection);

    assert_eq!(comps, 162085);
  }

  #[test]
  fn solves_homework_problem2() {
    let mut vec = load_hw_vector();
    let mut comps: usize = 0;
    let end = vec.len() - 1;
    let pivot_selection = |_v: &[u32], _s: usize, e: usize| e;
    qs_sub_vec(&mut vec, (0, end), &mut comps, &pivot_selection);

    assert_eq!(comps, 164123);
  }

  #[test]
  fn solves_homework_problem3() {
    let mut vec = load_hw_vector();
    let mut comps: usize = 0;
    let end = vec.len() - 1;
    let pivot_selection = |v: &[u32], s: usize, e: usize| {
      let mut pivot_options: Vec<usize> = vec![s, ((e - s) / 2 + s), e];
      if v[pivot_options[0]] > v[pivot_options[1]] {
        swap_elements(&mut pivot_options, 0, 1);
      }
      if v[pivot_options[1]] > v[pivot_options[2]] {
        swap_elements(&mut pivot_options, 1, 2);
        if v[pivot_options[0]] > v[pivot_options[1]] {
          swap_elements(&mut pivot_options, 0, 1);
        }
      }
      pivot_options[1]
    };

    qs_sub_vec(&mut vec, (0, end), &mut comps, &pivot_selection);

    assert_eq!(comps, 138382);
  }
}
