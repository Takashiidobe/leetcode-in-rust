## Contains Duplicate

### Problem

> Given an integer array nums, return true if any value appears at least
> twice in the array, and return false if every element is distinct.

### Intuition

### Test Cases

```{.rs include=src/questions/contains_duplicate.rs startLine=4 endLine=8}

```

### Using Sets

If a slice of numbers is the same length as the set of its numbers, we
know that the slice **only contains** unique numbers. With this, we can
find the solution to the problem:

### Complexity

O(n) time, O(n) space. We take O(n) time to convert the slice into the
HashSet, and the HashSet takes O(n) space as well.

### Answer

```{.rs include=src/questions/contains_duplicate.rs startLine=10 endLine=14}

```
