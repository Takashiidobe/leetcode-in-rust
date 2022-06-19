## Valid Anagram

### Problem

> Given two strings `s` and `t`, return true if `t` is an anagram of `s`,
> and false otherwise.

### Intuition

What constitutes an anagram? Two strings are anagrams if both strings
contain the same number of characters. For example, "abc" and "cba" are
anagrams, since they both have one a, one b, one c.

### Test Cases

```{.rs include=src/questions/sequences/valid_anagram.rs startLine=4 endLine=9}

```

### Using HashMaps

Anagrams have all the same characters with the same occurrence count.
In this case, we can create two HashMaps of the occurrence count of `s`
and `t`, and check if they are equal. If they are equal, then the
strings are anagrams, otherwise, they aren't anagrams.

### Complexity

O(n) time, O(n) space. We take O(n) time to convert the s and t into
HashMaps, and the HashMaps takes O(n) space as well.

### Answer

```{.rs include=src/questions/sequences/valid_anagram.rs startLine=11 endLine=24}

```
