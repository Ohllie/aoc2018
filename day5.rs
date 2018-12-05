use std::fs;

fn are_same(a: char, b: char) -> bool {
  return a.to_lowercase().to_string() == b.to_lowercase().to_string();
}

fn without_unit(unit: &str, s: &str) -> String {
  let mut new_str = String::new();
  for c in s.chars() {
    if c.to_lowercase().to_string() != unit {
      new_str.push(c);
    }
  }
  return new_str;
}

fn react(p: &str) -> usize {
  let mut polymer = p.to_string();

  loop {
    let mut new_str = String::new();
    let mut prev_char = '*';
    let mut i = 0;
    let mut reacted=false;

    for c in polymer.chars() {
      if c.is_lowercase() != prev_char.is_lowercase() && are_same(c, prev_char) {
        new_str.push_str(&polymer[..i-1].to_string());
        new_str.push_str(&polymer[i+1..polymer.len()].to_string());
        reacted=true;
        break;
      }

      i += 1;
      prev_char = c;
    }

    if !reacted {
      break;
    }

    polymer = new_str;
  }

  return polymer.len();
}

fn main() {
  let polymer = fs::read_to_string("day5.txt").expect("Failed to read file");

  // Part 1

  println!("{}", react(&polymer));

  // Part 2

  let v = vec![
    "a","b","c","d","e",
    "f","g","h","i","j",
    "k","l","m","n","o",
    "p","q","r","s","t",
    "u","v","w","x","y",
    "z"
  ];

  let mut best = polymer.len();

  for letter in v {
    let mod_polymer = without_unit(letter, &polymer);
    let v = react(&mod_polymer);
    if v < best {
      best = v;
    } 
  }

  println!("{}", best);
}