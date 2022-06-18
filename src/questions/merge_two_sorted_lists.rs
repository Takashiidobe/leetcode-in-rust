use crate::*;

pub fn merge_two_sorted_lists(list1: LLNode, list2: LLNode) -> LLNode {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) | (None, Some(l)) => Some(l),
        (Some(mut l1), Some(mut l2)) => {
            if l1.val < l2.val {
                l1.next = merge_two_sorted_lists(l1.next, Some(l2));
                Some(l1)
            } else {
                l2.next = merge_two_sorted_lists(l2.next, Some(l1));
                Some(l2)
            }
        }
    }
}

test! {
    test_1: merge_two_sorted_lists(slist![1,2,4], slist![1,3,4]), slist![1,1,2,3,4,4],
    test_2: merge_two_sorted_lists(slist![], slist![]), slist![],
    test_3: merge_two_sorted_lists(slist![], slist![0]), slist![0],
}
