use crate::*;

pub fn lowest_common_ancestor(mut root: BSTNode, p: BSTNode, q: BSTNode) -> BSTNode {
    let (pnode, qnode) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
    while let Some(node) = root.clone() {
        let node = node.borrow();
        if pnode > node.val && qnode > node.val {
            root = node.right.clone()
        } else if pnode < node.val && qnode < node.val {
            root = node.left.clone()
        } else {
            return root;
        }
    }
    root
}

test! {
    test_1: lowest_common_ancestor(btree![6,2,8,0,4,7,9,null,null],btree![2],btree![8]), btree![6,2,8,0,4,7,9,null,null],
    test_2: lowest_common_ancestor(btree![6,2,8,0,4,7,9,null,null,3,5], btree![2], btree![4]), btree![2, 0, 4, null, null, 3, 5],
    test_3: lowest_common_ancestor(btree![2, 1], btree![2], btree![1]), btree![2, 1],
}
