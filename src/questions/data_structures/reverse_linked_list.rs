use crate::*;
use std::mem;

test! {
    test_1: reverse_list(slist![1, 2, 3]), slist![3,2,1],
    test_2: reverse_list(slist![]), slist![],
    test_3: reverse_list(slist![1]), slist![1],
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut box_node: Option<Box<ListNode>> = head;
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(mut node) = box_node {
        box_node = mem::replace(&mut node.next, prev);
        prev = Some(node);
    }

    prev
}
