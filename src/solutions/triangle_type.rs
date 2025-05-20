use std::collections::HashMap;

pub fn triangle_type(nums: Vec<i32>) -> String {
  let mut map: HashMap<i32, i32> = HashMap::new();
  let mut is_invalid = false;

  nums.iter().enumerate().for_each(|(idx, &length)| {
    // check triangle validity
    let complement_sum: i32 = nums.iter().enumerate()
      .filter(|&(i, _)| i != idx)
      .map(|(_, &l)| l)
      .sum();

    if &complement_sum <= &length {
      is_invalid = true;
    }
 
    // count triangle side-lengths occurences
    let existing: Option<&i32> = map.get(&length);

    if let Some(&count) = existing {
      map.insert(length, &count + 1);
    } else {
      map.insert(length, 1);
    }
  });

  if is_invalid {
    return String::from("none")
  }
 
  let keys = map.keys().count();

  return match keys {
    1 => String::from("equilateral"),
    2 => String::from("isosceles"),
    3 => String::from("scalene"),
    _ => String::from("none")
  }
}
