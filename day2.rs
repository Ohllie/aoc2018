use std::fs;
use std::collections::HashMap;

fn count_letters(s: &str) -> HashMap<char, i32> {
  let mut word_map = HashMap::new();

  for letter in s.chars() {
    let counter = word_map.entry(letter).or_insert(0);
    *counter += 1;
  }

  word_map
}

fn count_diff(s: &str, s2: &str) -> (i32, usize) {
  let mut diff : i32 = 0;
  let mut diff_i : usize = 0;

  for i in 0..s.len() {
    if s.chars().nth(i) != s2.chars().nth(i) {
      diff += 1;
      diff_i = i;
    }
  }

  (diff, diff_i)
}

fn main() {
  let data = fs::read_to_string("day2.txt").expect("Failed to read file");

  // Part 1

  let mut two_counter = 0;
  let mut three_counter = 0;

  for line in data.split("\n") {
    let mut counts_two = 0;
    let mut counts_three = 0;

    for item in count_letters(line).values() {
      if *item == 2 {
        counts_two = 1;
      } else if *item == 3 {
        counts_three = 1;
      }
    }

    two_counter += counts_two;
    three_counter += counts_three;
  }

  print!("{:?}\n", two_counter * three_counter);

  // Part two

  let mut found = false;

  for line1 in data.split("\n") {
    for line2 in data.split("\n") {
      let (diff, diff_i) = count_diff(&line1, &line2);

      if diff == 1 {
        for i in 0..line1.len() {
          if i == diff_i { continue; }

          print!("{}", line1.chars().nth(i).unwrap())
        }
        print!("\n");


        found = true;
        break;
      }
    }

    if found {
      break;
    }
  }
}
