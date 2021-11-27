use crate::*;
use std::cmp::max;

/// Finds the maximum path sum through a binary tree.
pub fn max_path_sum(root: BSTNode) -> i32 {
  let mut max_so_far = i32::MIN;
  fn helper(node: &BSTNode, max_so_far: &mut i32) -> i32 {
    match node {
      Some(n) => {
        let val = n.borrow().val;
        let l = max(0, helper(&n.borrow().left, max_so_far));
        let r = max(0, helper(&n.borrow().right, max_so_far));
        *max_so_far = max(*max_so_far, val + l + r);
        val + max(l, r)
      }
      None => 0,
    }
  }
  helper(&root, &mut max_so_far);
  max_so_far
}

test! {
    test_1: max_path_sum(btree![1,2,3]), 6,
    test_2: max_path_sum(btree![-10, 9, 20, null, null, 15, 7]), 42,
}
