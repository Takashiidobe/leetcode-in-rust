use crate::*;

test! {
    test_1: is_same_tree(btree![], btree![]), true,
    test_2: is_same_tree(btree![1], btree![]), false,
    test_3: is_same_tree(btree![1,2,3], btree![1,2,3]), true,
    test_4: is_same_tree(btree![1,2,3,4], btree![1,2,3]), false,
}

/// Calculates if two binary search trees have the same values.
/// In this question, there are four possible cases:
/// 1. Both left and right point to a `None` node. In this case, return true.
/// 2. Both left and right point to nodes with the same value. Continue recursing through both
///    trees left and right subtrees.
/// 3. For any other case, return false.
pub fn is_same_tree(p: BSTNode, q: BSTNode) -> bool {
    fn same(p: &BSTNode, q: &BSTNode) -> bool {
        match (p, q) {
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && same(&left.left, &right.left)
                    && same(&left.right, &right.right)
            }
            (None, None) => true,
            _ => false,
        }
    }
    same(&p, &q)
}
