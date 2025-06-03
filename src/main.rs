use crate::solutions::longest_substring_wo_repeating_chars::longest_substring_wo_repeating_chars;
use crate::solutions::triangle_type::triangle_type;
use crate::solutions::two_sum::two_sum;

mod solutions;

fn main() {
  println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
  println!("{:?}", triangle_type(vec![8, 4, 2]));
  println!(
    "{:?}",
    longest_substring_wo_repeating_chars(String::from("abcabcbb"))
  );
}
