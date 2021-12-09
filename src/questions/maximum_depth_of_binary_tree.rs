use crate::*;
use std::cmp::max;

test! {
    test_1: max_depth(btree![]), 0,
    test_2: max_depth(btree![1, 2, 3, 4, 5, 6]), 3,
    test_3: max_depth(btree![1, 2, 3, 4, 5, 6, 7, 8]), 4,
}

/// A function that finds the maximum depth of a binary tree.
/// It does this recursively:
/// If the current node is `None`, it returns a depth of 0.
/// For a node that exists, it takes the max depth of the left and right subtree and adds 1 to it.
/// It then returns that value.
pub fn max_depth(root: BSTNode) -> i32 {
    fn traverse(node: &BSTNode) -> i32 {
        match node {
            Some(n) => max(
                traverse(&n.borrow().left) + 1,
                traverse(&n.borrow().right) + 1,
            ),
            None => 0,
        }
    }
    traverse(&root)
}
