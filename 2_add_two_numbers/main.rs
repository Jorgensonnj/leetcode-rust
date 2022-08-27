use std::time::Instant;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// first attempt
// would have worked if there was not a size limit for ints
//pub fn recursive_sum(l1: Option<Box<ListNode>>, factor: i32) -> i32 {
//    match l1 {
//        None => 0,
//        Some(l1) => {
//            let current_node = *l1;
//            current_node.val * factor + recursive_sum(current_node.next, factor * 10)
//        }
//    }
//}
//
//pub fn recursive_build(sum: i32) -> Option<Box<ListNode>> {
//    let left = (sum - (sum % 10)) / 10;
//    if sum != 0 && left != 0 {
//        Some(Box::new(ListNode{val: (sum % 10), next: recursive_build(left)}))
//    }else{
//        Some(Box::new(ListNode{val: sum, next: None}))
//    }
//}
//
//pub fn my_add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//    recursive_build( recursive_sum(l1, 1) + recursive_sum(l2, 1))
//}

// my original code
pub fn my_add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut l1 = l1.clone();
    let mut l2 = l2.clone();

    let mut remainder = 0;
    let mut head      = Box::new(ListNode::new(0));
    let mut head_ref  = &mut head;
    while l1.is_some() || l2.is_some() {

        let val = match (&l1, &l2) {
            (Some(node1), Some(node2)) => node1.val + node2.val + remainder,
            (Some(node1), None)        => node1.val + remainder,
            (None,        Some(node2)) => node2.val + remainder,
            (None,        None)        => remainder,
        };

        remainder = val / 10;

        head_ref.next = Some(Box::new(ListNode::new(val % 10)));
        head_ref = head_ref.next.as_mut().unwrap();

        l1 = if let Some(l) = l1 { l.next } else { None };
        l2 = if let Some(l) = l2 { l.next } else { None };

    }

    if remainder > 0 {
        head_ref.next = Some(Box::new(ListNode::new(remainder)));
    }

    return head.next;
}

// someone else's code, similar to mine
// https://leetcode.com/problems/add-two-numbers/discuss/412727/Rust-solution-(0ms)
pub fn other1_add_two_numbers( l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>> ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut p = l1;
        let mut q = l2;

        let mut carry: i32 = 0;

        while p != None || q != None {
            let sum = match (&p, &q) {
                (Some(l1), Some(l2)) => l1.val + l2.val + carry,
                (Some(l1), None) => l1.val + carry,
                (None, Some(l2)) => l2.val + carry,
                (None, None) => carry,
            };

            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            p = if p != None { p.unwrap().next } else { p };
            q = if q != None { q.unwrap().next } else { q };
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy_head.next
}

// someone else's code, recursive
// https://leetcode.com/problems/add-two-numbers/discuss/1450822/Recursive-Solution-C%2B%2B-or-Rust
fn other2_add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        if carry == 0 {
            return None;
        } else {
            return Some(Box::new(ListNode::new(carry)));
        }
    }
    let l1 = l1.unwrap_or_else(|| Box::new(ListNode::new(0)));
    let l2 = l2.unwrap_or_else(|| Box::new(ListNode::new(0)));
    let digit = l1.val + l2.val + carry;
    let carry = digit / 10;
    let digit = digit % 10;
    let mut cur_node = Box::new(ListNode::new(digit));
    cur_node.next = other2_add_two_numbers(l1.next, l2.next, carry);
    Some(cur_node)
}

fn main() {

    // test one
    //let head_one = Box::new(ListNode::new(0));
    //let head_two   = Box::new(ListNode::new(0));

    // test two
    //let mut head_one = Box::new(ListNode::new(2));
    //let mut link_one = Box::new(ListNode::new(4));
    //let     link_two = Box::new(ListNode::new(3));

    //link_one.next = Some(link_two);
    //head_one.next = Some(link_one);

    //let mut head_two   = Box::new(ListNode::new(5));
    //let mut link_three = Box::new(ListNode::new(6));
    //let     link_four  = Box::new(ListNode::new(4));

    //link_three.next = Some(link_four);
    //head_two.next   = Some(link_three);

    // test three
    let mut h_one   = Box::new(ListNode::new(1));
    let mut l_one   = Box::new(ListNode::new(9));
    let mut l_two   = Box::new(ListNode::new(9));
    let mut l_three = Box::new(ListNode::new(9));
    let mut l_four  = Box::new(ListNode::new(9));
    let mut l_five  = Box::new(ListNode::new(9));
    let mut l_six   = Box::new(ListNode::new(9));
    let mut l_seven = Box::new(ListNode::new(9));
    let mut l_eight = Box::new(ListNode::new(9));
    let     l_nine  = Box::new(ListNode::new(9));

    l_eight.next = Some(l_nine);
    l_seven.next = Some(l_eight);
    l_six.next   = Some(l_seven);
    l_five.next  = Some(l_six);
    l_four.next  = Some(l_five);
    l_three.next = Some(l_four);
    l_two.next   = Some(l_three);
    l_one.next   = Some(l_two);
    h_one.next   = Some(l_one);

    let h_two   = Box::new(ListNode::new(9));

    let mut h1_one   = Box::new(ListNode::new(1));
    let mut l1_one   = Box::new(ListNode::new(9));
    let mut l1_two   = Box::new(ListNode::new(9));
    let mut l1_three = Box::new(ListNode::new(9));
    let mut l1_four  = Box::new(ListNode::new(9));
    let mut l1_five  = Box::new(ListNode::new(9));
    let mut l1_six   = Box::new(ListNode::new(9));
    let mut l1_seven = Box::new(ListNode::new(9));
    let mut l1_eight = Box::new(ListNode::new(9));
    let     l1_nine  = Box::new(ListNode::new(9));

    l1_eight.next = Some(l1_nine);
    l1_seven.next = Some(l1_eight);
    l1_six.next   = Some(l1_seven);
    l1_five.next  = Some(l1_six);
    l1_four.next  = Some(l1_five);
    l1_three.next = Some(l1_four);
    l1_two.next   = Some(l1_three);
    l1_one.next   = Some(l1_two);
    h1_one.next   = Some(l1_one);

    let h1_two   = Box::new(ListNode::new(9));

    let mut h2_one   = Box::new(ListNode::new(1));
    let mut l2_one   = Box::new(ListNode::new(9));
    let mut l2_two   = Box::new(ListNode::new(9));
    let mut l2_three = Box::new(ListNode::new(9));
    let mut l2_four  = Box::new(ListNode::new(9));
    let mut l2_five  = Box::new(ListNode::new(9));
    let mut l2_six   = Box::new(ListNode::new(9));
    let mut l2_seven = Box::new(ListNode::new(9));
    let mut l2_eight = Box::new(ListNode::new(9));
    let     l2_nine  = Box::new(ListNode::new(9));

    l2_eight.next = Some(l2_nine);
    l2_seven.next = Some(l2_eight);
    l2_six.next   = Some(l2_seven);
    l2_five.next  = Some(l2_six);
    l2_four.next  = Some(l2_five);
    l2_three.next = Some(l2_four);
    l2_two.next   = Some(l2_three);
    l2_one.next   = Some(l2_two);
    h2_one.next   = Some(l2_one);

    let h2_two   = Box::new(ListNode::new(9));

    let my_time = Instant::now();
    let my_result = my_add_two_numbers( Some(h_one), Some(h_two) );
    println!("Result: {:?}, Time: {:?}", my_result, my_time.elapsed());

    let other1_time = Instant::now();
    let other1_result = other1_add_two_numbers( Some(h1_one), Some(h1_two));
    println!("Result: {:?}, Time: {:?}", other1_result, other1_time.elapsed());

    let other2_time = Instant::now();
    let other2_result = other2_add_two_numbers( Some(h2_one), Some(h2_two), 0 );
    println!("Result: {:?}, Time: {:?}", other2_result, other2_time.elapsed());
}
