use crate::*;

/// Inverts a Binary Tree.
/// This is done by taking each node's children and swapping them.
pub fn invert_binary_tree(
    mut root: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(node: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            let mut n = n.borrow_mut();

            match (n.left.take(), n.right.take()) {
                (None, None) => {}
                (l, r) => {
                    n.left = r;
                    n.right = l;
                }
            }
            helper(&mut n.left);
            helper(&mut n.right);
        }
    }
    helper(&mut root);

    root
}

test! {
    test_1: invert_binary_tree(btree![4, 2, 7, 1, 3, 6, 9]), btree![4, 7, 2, 9, 6, 3, 1],
    test_2: invert_binary_tree(btree![1, 2, 3, 4]), btree![1, 3, 2, null, null, null, 4],
}
