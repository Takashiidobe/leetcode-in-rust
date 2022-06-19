## Invert Binary Tree

### Problem

> Given the `root` of a binary tree, invert the tree, and return its root.

### Intuition

This question tests your knowledge of recursion. To do so, start off
with the base case:

- What happens when left is None and right has a value? Return false.
- What happens when left has a value and right is None? Return false.
- What happens when both left and right are None? Return true.
- What happens when left and right have different values? Return false.
- What happens when left and right have the same values? Test their left
  and right nodes for equality as well.

### Test Cases

```{.rs include=src/questions/invert_binary_tree.rs startLine=3 endLine=8}

```

### Answer

```{.rs include=src/questions/invert_binary_tree.rs startLine=10 endLine=40}

```
