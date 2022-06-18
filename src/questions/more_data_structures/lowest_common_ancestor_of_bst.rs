use crate::*;

fn traverse(root: &BSTNode, p: &BSTNode, q: &BSTNode) -> BSTNode {
    if let (Some(root_node), Some(p_node), Some(q_node)) = (root, p, q) {
        let (root_, p_, q_) = (root_node.borrow(), p_node.borrow(), q_node.borrow());
        match (
            root_.val > p_.val && root_.val > q_.val,
            root_.val < p_.val && root_.val < q_.val,
        ) {
            (true, _) => traverse(&root_.left, p, q),
            (_, true) => traverse(&root_.right, p, q),
            _ => root.clone(),
        }
    } else {
        root.clone()
    }
}

pub fn lowest_common_ancestor(root: BSTNode, p: BSTNode, q: BSTNode) -> BSTNode {
    traverse(&root, &p, &q)
}

test! {
    test_1: lowest_common_ancestor(btree![6,2,8,0,4,7,9,null,null],btree![3],btree![5]), btree![6],
    test_2: lowest_common_ancestor(btree![6,2,8,0,4,7,9,null,null,3,5], btree![2], btree![4]), btree![2],
    test_3: lowest_common_ancestor(btree![2, 1], btree![2], btree![1]), btree![2],
    test_4: lowest_common_ancestor(btree![5,3,6,2,4,null,null,1], btree![1], btree![3]), btree![3],
}
