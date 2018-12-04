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
