use crate::*;
use std::mem;

test! {
    test_1: reverse_list(slist![1, 2, 3]), slist![3,2,1],
    test_2: reverse_list(slist![]), slist![],
    test_3: reverse_list(slist![1]), slist![1],
}

type MaybeListNode = Option<Box<ListNode>>;

pub fn reverse_list(head: MaybeListNode) -> MaybeListNode {
    let mut box_node: MaybeListNode = head;
    let mut prev: MaybeListNode = None;
    while let Some(mut node) = box_node {
        box_node = mem::replace(&mut node.next, prev);
        prev = Some(node);
    }

    prev
}
