use crate::*;

pub fn build_tree(preorder: &[i32], inorder: &[i32]) -> BSTNode {
    inorder.iter().position(|&v| v == preorder[0]).map(|pivot| {
        Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: build_tree(&preorder[1..(1 + pivot)], &inorder[..pivot]),
            right: build_tree(&preorder[(1 + pivot)..], &inorder[(pivot + 1)..]),
        }))
    })
}

test! {
    test_1: build_tree(&[3,9,20,15,7], &[9,3,15,20,7]), btree![3,9,20,null,null,15,7],
    test_2: build_tree(&[-1], &[-1]), btree![-1],
    test_3: build_tree(&[], &[]), btree![],
}
