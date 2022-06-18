use crate::*;
use std::collections::BTreeMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut h = BTreeMap::new();

    for s in strs {
        let mut key: Vec<char> = s.chars().collect();
        key.sort_unstable();
        h.entry(key).or_insert(vec![]).push(s);
    }

    h.values().cloned().collect()
}

test! {
    test_1: group_anagrams(vec_string!["eat","tea","tan","ate","nat","bat"]),vec![vec_string!["bat"], vec_string!["eat","tea","ate"], vec_string!["tan","nat"]],
    test_2: group_anagrams(vec_string![""]), vec![vec_string![""]],
    test_3: group_anagrams(vec_string!["a"]), vec![vec_string!["a"]],
}
