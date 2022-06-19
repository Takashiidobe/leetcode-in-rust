## Group Anagrams

Example 1:

```
Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
```

Example 2:

```
Input: strs = [""]
Output: [[""]]
```

Example 3:

```
Input: strs = ["a"]
Output: [["a"]]
```

### Problem

Given an array of strings `strs`, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

### Intuition

Let's recycle what we learned from the question about anagrams. Two
words are anagrams of each other if they contain the same letters.

We have a `Vec` of `String`s. We need to find a way to represent our
`String`s as characters.

We could, as we did previously, create a `HashMap` out of the chars of the
`String`, and then check if two words are anagrams by checking if their
`HashMap` representations are equal.

Next, we'd like to use each `HashMap` as a key to a larger `HashMap`,
allowing us to group anagrams easily.

Unfortunately, this doesn't work, because `HashMap` doesn't implement
`Hash`, so a `HashMap` cannot be used as a key to a `HashMap`.

Darn.

Luckily for us, though, `Vec` can be used as a key to a `HashMap`.

We need to find a way to see if two `String`s are equal when
represented as `Vec`s. To do that, we could grab its `chars`
representation and then sort the keys.

If we do that, since `chars` are sortable, we can use them to represent
anagrams. Hurray.

So we do this for each `String` in `strs`, and then check the values of
the HashMap, which is our grouping.

### Test Cases

```{.rs include=src/questions/sequences/group_anagrams.rs startLine=16 endLine=20}

```

### Answer

```{.rs include=src/questions/sequences/group_anagrams.rs startLine=4 endLine=14}

```
