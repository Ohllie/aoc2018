use std::fs;
use std::collections::HashSet;

fn main() {
  let data = fs::read_to_string("day3.txt").expect("Failed to read file");
  const SIZE : usize = 1000;

  let mut fabric : [[i32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
  let mut fabric_prev : [[usize; SIZE]; SIZE] = [[0; SIZE]; SIZE];
  let mut overlapping : HashSet<usize> = HashSet::new();
  let mut claim_id = 1;

  for line in data.split("\n") {
    let parts: Vec<&str> = line.trim().split(" ").collect();

    if parts.len() == 1 {
      continue;
    }

    let mut start = parts[2].to_string();
    start.pop();

    let start_split: Vec<&str> = start.split(",").collect();
    let x : usize = start_split[0].parse().unwrap();
    let y : usize = start_split[1].parse().unwrap();


    let size = parts[3].to_string();
    let size_split: Vec<&str> = size.split("x").collect();
    let w : usize = size_split[0].parse().unwrap();
    let h : usize = size_split[1].parse().unwrap();

    for i in y..y+h {
      for j in x..x+w {
        fabric[i][j] += 1;

        if fabric[i][j] >= 2 {
          overlapping.insert(claim_id);
          if fabric_prev[i][j] != 0 {
            overlapping.insert(fabric_prev[i][j]);
          }
        }

        fabric_prev[i][j] = claim_id;
      }
    }

    claim_id += 1;
  }

  // Part 1

  let mut cnt = 0;
  for i in 0..SIZE {
    for j in 0..SIZE {
      if fabric[i][j] >= 2 {
        cnt += 1;
      }
    }
  }

  println!("{}", cnt);

  // Part 2

  for i in 1usize..claim_id {
    if !overlapping.contains(&i) {
      println!("{}", i);
      break;
    }
  }
}