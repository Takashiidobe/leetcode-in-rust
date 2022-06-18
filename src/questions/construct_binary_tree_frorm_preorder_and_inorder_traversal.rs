use crate::*;

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> BSTNode {
    if !preorder.is_empty() && !inorder.is_empty() {
        if let Some(pos) = inorder.iter().position(|&x| x == preorder[0]) {
            let mut root = TreeNode::new(preorder[0]);
            root.left = build_tree(preorder[1..].to_vec(), inorder[..pos].to_vec());
            root.right = build_tree(preorder[pos + 1..].to_vec(), inorder[pos + 1..].to_vec());
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    } else {
        None
    }
}

test! {
    test_1: build_tree(vec![3,9,20,15,7], vec![9,3,15,20,7]), btree![3,9,20,null,null,15,7],
    test_2: build_tree(vec![-1], vec![-1]), btree![-1],
    test_3: build_tree(vec![], vec![]), btree![],
}
