use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map: HashMap<i32, i32> = HashMap::new();

  for (i, &num) in nums.iter().enumerate() {
    let complement = target - num;

    let existing: Option<&i32> = map.get(&complement);
    if let Some(&j) = existing {
      return vec![j, i as i32];
    }

    map.insert(num, i as i32);
  }

  return Vec::new();
}
