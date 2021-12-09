use crate::*;

test! {
    test_1: level_order(btree![1,2,3]), vec![vec![1], vec![2, 3]],
    test_2: level_order(btree![3,9,20,null,null,15,7]), vec![vec![3],vec![9,20],vec![15,7]],
}

pub fn level_order(root: BSTNode) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    fn traversal(root: &BSTNode, res: &mut Vec<Vec<i32>>, level: usize) {
        if let Some(r) = root {
            if res.len() == level {
                res.push(Vec::new());
            }
            res[level].push(r.borrow().val);
            traversal(&r.borrow().left, res, level + 1);
            traversal(&r.borrow().right, res, level + 1);
        }
    }
    traversal(&root, &mut res, 0);

    res
}
