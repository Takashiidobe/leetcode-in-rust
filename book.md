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

## A macro for testing

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

!include chapters/introductory/index.md

# Trees

## Maximum Path through a Binary Tree

```{.rs include=src/questions/binary_tree_maximum_path_sum.rs}

```

## Validate Binary Search Tree

```{.rs include=src/questions/valid_binary_search_tree.rs}

```

!include chapters/trees/same_tree.md
