## Maximum Depth of Binary Tree

### Problem

> Given the root of a binary tree, return its maximum depth.

> A binary tree's maximum depth is the number of nodes along the longest
> path from the root node down to the farthest leaf node.

### Intuition

To Find the maximum depth of a binary tree, assume that every node adds
to the level of the tree by 1.

Assume that there are four possible states a node can be in:

1. The Node does not exist (is None). In this case, return 0.
2. The Node exists, but has no children. In this case, return 1.
3. The Node exists, and has either a left or right child. In this case,
   return 1 + the depth of the left or right child.
4. The Node exists, and has both a left and right child. In this case,
   return 1 + the maximum depth of the right or left child.

### Test Cases

```{.rs include=src/questions/maximum_depth_of_binary_tree.rs startLine=4 endLine=8}

```

### Answer

```{.rs include=src/questions/maximum_depth_of_binary_tree.rs startLine=11 endLine=50}

```
