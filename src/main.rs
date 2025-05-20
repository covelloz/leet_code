use crate::solutions::two_sum::two_sum;
use crate::solutions::triangle_type::triangle_type;

mod solutions;

fn main() {
  println!("{:?}", two_sum(vec![2,7,11,15], 9));
  println!("{:?}", triangle_type(vec![8,4,2]));
}
