use std::fs;
use std::collections::HashSet;

fn main() {
  let data = fs::read_to_string("day1.txt").expect("Failed to read file");

  let mut numbers : Vec<i32> = Vec::new();
  for line in data.split("\n") {
    let trimmed = line.trim();
    if trimmed.len() > 0 {
      numbers.push(trimmed.parse().expect("All lines should be numbers"));
    }
  }

  let changes : i32 = numbers.iter().sum();
  print!("First part: {:?}\n", changes);

  let mut seen: HashSet<i32> = HashSet::new();
  let mut cur = 0;

  loop {
    for num in numbers.iter() {
      cur += num;

      if seen.contains(&cur) {
        print!("Second part: {:?}\n", cur);
        return;
      }

      seen.insert(cur);
    }
  }
}