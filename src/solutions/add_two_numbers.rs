#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  pub fn vec_to_ll(input: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in input.iter().rev() {
      head = Some(Box::new(ListNode { val, next: head }));
    }
    head
  }
}

pub fn add_two_numbers(
  l1: Option<Box<ListNode>>,
  l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
  // algorithm
  //  compute: \sum_{k=0,i=0,j=0}^max(N, M) (i + j) * 10^k
  //  where
  //    N:=len(l1) and M:=len(l2)
  //    0<=i<=N & 0<=j<=M
  //  with i,j:=0 if out-of-bounds

  let mut l1_mref = l1.as_ref();
  let mut l2_mref = l2.as_ref();

  // initialize linked list
  let mut result: Option<Box<ListNode>> = None;
  let mut result_next = &mut result;
  let mut carry = 0;

  while l1_mref.is_some() || l2_mref.is_some() || carry > 0 {
    // compute base10 coefficients
    let val1: i32 = l1_mref.map_or(0, |node| node.val);
    let val2: i32 = l2_mref.map_or(0, |node| node.val);

    // compute result digit
    let sum = val1 + val2 + carry;
    let digit = sum % 10;

    // compute 10s digit carry over
    if sum >= 10 {
      carry = 1
    } else {
      carry = 0
    }

    // insert digit into linked list (at tail)
    let new_node = Some(Box::new(ListNode {
      val: digit,
      next: None,
    }));
    *result_next = new_node; // sets new_node as 'next' on the previous node

    if let Some(node) = result_next {
      result_next = &mut node.next; // update ref for the next iteration
    }

    l1_mref = l1_mref.as_ref().and_then(|node| node.next.as_ref());
    l2_mref = l2_mref.as_ref().and_then(|node| node.next.as_ref());
  }

  result
}
