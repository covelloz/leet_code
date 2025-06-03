use std::collections::HashMap;

pub fn longest_sub(input: String) -> i32 {
  let mut char_idx: HashMap<char, usize> = HashMap::new();
  let mut max_len: usize = 0;
  let mut left_idx: usize = 0;

  // left-bound shrinking sliding window - O(N)
  for (right_idx, c) in input.chars().enumerate() {
    // check if char has been seen by window
    if let Some(&seen_idx) = char_idx.get(&c) {
      if seen_idx >= left_idx {
        // shrink window if char was seen
        left_idx = seen_idx + 1;
      }
    }

    // char not seen yet
    char_idx.insert(c, right_idx);
    max_len = max_len.max(right_idx - left_idx + 1)
  }

  return max_len.try_into().unwrap_or(0);
}
