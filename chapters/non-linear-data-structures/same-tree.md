## Same Tree

### Problem

> Given the roots of two binary trees p and q, write a function to check if they are the same or not.
> Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

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

```{.rs include=src/questions/non_linear_data_structures/same_tree.rs startLine=3 endLine=8}

```

### Answer

```{.rs include=src/questions/non_linear_data_structures/same_tree.rs startLine=10 endLine=40}

```

|                   Left                   |                  Right                   |
| :--------------------------------------: | :--------------------------------------: |
| ![](../../figures/same_tree/example.svg) | ![](../../figures/same_tree/example.svg) |
