use std::fs;
use std::collections::HashMap;

fn crop_letters(s: &mut String, pos: usize) {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => { s.drain(0..pos); },
        None => { s.clear(); },
    }
}

fn get_guard_id(s: &str) -> i32 {
  let parts : Vec<&str> = s.split(" ").collect();
  let mut id : String = parts[1].to_string();
  crop_letters(&mut id, 1);
  return id.parse().unwrap();
}

fn get_minute(s: &str) -> usize {
  let parts : Vec<&str> = s.split(" ").collect();
  let hour_part : Vec<&str> = parts[1].split(":").collect();
  return hour_part[1].parse().unwrap();
}

fn main() {
  let data = fs::read_to_string("day4.txt").expect("Failed to read file");
  let mut actions = Vec::new();

  for line in data.split("\n") {
    let mut trimmed = line.trim().to_string();
    crop_letters(&mut trimmed, 1);

    if trimmed.len() == 0 {
      continue;
    }
    
    let start_split: Vec<&str> = trimmed.split("]").collect();
    let date = start_split[0].to_string();
    let action_string = start_split[1].trim().to_string();

    actions.push((date, action_string.clone()));
  }

  actions.sort();

  // Part 1

  let mut minute_counts : HashMap<i32, [i32; 60]> = HashMap::new();
  let mut current_guard_id = -1;
  let mut started_sleep = 0;
  let mut stopped_sleep;

  for (date, action_string) in actions {
    if action_string.find("Guard") == Some(0) {
      // Guard changed
      current_guard_id = get_guard_id(&action_string);

      if !minute_counts.contains_key(&current_guard_id) {
        minute_counts.insert(current_guard_id, [0; 60]);
      }
    } else if action_string.find("wakes") == Some(0) {
      // Guard wakes up
      stopped_sleep = get_minute(&date);
      let mut guard_counts = minute_counts.get_mut(&current_guard_id).unwrap();

      for i in started_sleep..stopped_sleep {
        guard_counts[i] += 1;
      }
    } else {
      // Guard sleeps
      started_sleep = get_minute(&date);
    }
  }

  let mut most_slept = 0;
  let mut most_id = 0;

  for (guard_id, minute_counter) in &minute_counts {
    let s : i32 = minute_counter.iter().sum();
    if s > most_slept {
      most_slept = s;
      most_id = *guard_id;
    }
  }

  let mut most_min = 0;
  let mut most_min_cnt = 0;
  let guard_counts = minute_counts.get(&most_id).unwrap();

  for i in 0..60 {
    if guard_counts[i] > most_min_cnt {
      most_min_cnt = guard_counts[i];
      most_min = i;
    }
  }

  println!("{}", most_id * (most_min as i32));


  // Part 2

  let mut most_min_id = 0;
  let mut most_min_v = 0;
  let mut most_min_guard = 0;

  for (guard_id, minute_counter) in &minute_counts {
    for i in 0..60 {
      if minute_counter[i] > most_min_guard {
        most_min_guard = minute_counter[i];
        most_min_v = i;
        most_min_id = *guard_id;
      }
    }
  }

  println!("{}", most_min_id * (most_min_v as i32));
}