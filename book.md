---
title: "Leetcode in Rust"
header-includes: |
  \renewcommand{\chapterheadstartvskip}{}
---

# Rust in a Nutshell

## Why Rust?

## Cargo

## Cargo Doc

## Crates

## Basic Data Structures

### Sequences

#### Vec

#### VecDeque

#### LinkedList

### Maps

#### HashMap

#### BTreeMap

### Sets

#### HashSet

#### BTreeSet

### Other

#### BinaryHeap

## Basic Algorithms

## Other Useful things

## Regex

## Derive Macros

## Counting in O(1) space with slices

# Macros for Rust

## `test!`

Unlike C and C++, a testing framework is built into rust. We can create
our own tests by creating a `mod` block and letting cargo know that we
want to test it.

Let's say we create this function:

`src/add.rs`
```rs
fn add(a: i32, b: i32) -> i32 {
  a + b
}
```

We can test it at the bottom of the file:

`src/add.rs`
```rs
...
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn add_one_and_one() {
    assert_eq!(add(1, 1), 2);
  }

  #[test]
  fn add_one_and_two() {
    assert_eq!(add(1, 2), 3);
  }
}
```

Macros let us reduce most of the boilerplate:

`src/lib.rs`
```rs
#[macro_export]
macro_rules! test {
  ($($name:ident: $left:expr, $right:expr,)*) => {
    #[cfg(test)]
    mod test {
      use super::*;
      $(
          #[test]
          fn $name() {
            assert_eq!($left, $right);
          }
       )*
    }
  }
}
```

Test can then be called like so:

`src/add.rs`
```rs
test! {
  add_one_to_one: add(1, 1), 2,
  add_one_to_two: add(1, 2), 3,
}
```

# Introductory

## Contains Duplicate

### Problem

> Given an integer array nums, return true if any value appears at least
> twice in the array, and return false if every element is distinct.

### Intuition



### Test Cases

```rs
[] == false
[1] == false
[1,1] == true
[1,2,3] == false
[1,2,1] == true
```

### Using Sets

If a slice of numbers is the same length as the set of its numbers, we
know that the slice **only contains** unique numbers. With this, we can
find the solution to the problem:

### Complexity

O(n) time, O(n) space. We take O(n) time to convert the slice into the
HashSet, and the HashSet takes O(n) space as well.

### Answer

```{.rs include=src/questions/contains_duplicate.rs}
```

# Trees

## Maximum Path through a Binary Tree

```{.rs include=src/questions/binary_tree_maximum_path_sum.rs}
```

## Validate Binary Search Tree

```{.rs include=src/questions/valid_binary_search_tree.rs}
```

## Same Tree

### Test Cases

```{.rs include=src/questions/same_tree.rs startLine=3 endLine=6}
```

```{.rs include=src/questions/same_tree.rs}
```

![](./figures/same_tree/example.svg)

