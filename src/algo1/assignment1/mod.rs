pub fn count_inversions(vector: Vec<u32>) -> u32 {
  let (_sorted, invert_count) = count(&vector, 0, vector.len());
  invert_count
}

fn count(v: &Vec<u32>, start: usize, end: usize) -> (Vec<u32>, u32) {
  if end - start <= 1 {
    (v[start..end].to_vec(), 0)
  } else {
    let halfway = ((end - start) / 2) + start;
    let (sorted_left, left_inverts) = count(v, start, halfway);
    let (sorted_right, right_inverts) = count(v, halfway, end);
    let (sorted, split_inverts) = count_splits(sorted_left, sorted_right);
    (sorted, left_inverts + right_inverts + split_inverts)
  }
}

fn count_splits(left_vec: Vec<u32>, right_vec: Vec<u32>) -> (Vec<u32>, u32) {
  let mut sorted: Vec<u32> = vec![];
  let mut split_invert_count = 0;
  let (mut l_head, mut r_head): (usize, usize) = (0, 0);
  while l_head < left_vec.len() && r_head < right_vec.len() {
    if left_vec[l_head] <= right_vec[r_head] {
      sorted.push(left_vec[l_head]);
      l_head += 1;
    } else {
      sorted.push(right_vec[r_head]);
      r_head += 1;
      split_invert_count += left_vec.len() - l_head;
    }
  }
  sorted.extend(left_vec[l_head..].to_vec());
  sorted.extend(right_vec[r_head..].to_vec());
  (sorted, split_invert_count as u32)
}

#[cfg(test)]
mod test {
  use super::*;
  use algo1::tests::helpers;

  #[test]
  fn solves_simple_min_inversions() {
    let trivial = vec![2, 1];
    let only_splits = vec![1, 3, 5, 2, 4, 6];
    let inversion_filled = vec![6, 5, 4, 3, 2, 1];

    assert_eq!(count_inversions(trivial), 1);
    assert_eq!(count_inversions(only_splits), 3);
    assert_eq!(count_inversions(inversion_filled), 15);
  }

  #[test]
  fn solves_the_homework_problem() {
    let num_vec: Vec<u32> =
      helpers::load_vec_from_file("src/algo1/assignment1/assignment1.txt");

    assert_eq!(count_inversions(num_vec), 2407905288);
  }
}
