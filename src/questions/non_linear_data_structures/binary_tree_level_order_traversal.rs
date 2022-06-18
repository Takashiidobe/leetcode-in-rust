use crate::*;
use std::collections::BTreeMap;

test! {
    test_1: level_order(btree![1,2,3]), vec![vec![1], vec![2, 3]],
    test_2: level_order(btree![3,9,20,null,null,15,7]), vec![vec![3],vec![9,20],vec![15,7]],
}

pub fn level_order(root: BSTNode) -> Vec<Vec<i32>> {
    let mut hm: BTreeMap<u32, Vec<i32>> = BTreeMap::new();
    fn traverse(node: &BSTNode, level: u32, hm: &mut BTreeMap<u32, Vec<i32>>) {
        if let Some(node) = node {
            let node = node.borrow();
            hm.entry(level).or_insert(vec![]).push(node.val);
            traverse(&node.left, level + 1, hm);
            traverse(&node.right, level + 1, hm);
        }
    }

    traverse(&root, 0, &mut hm);

    hm.values().cloned().collect()
}
