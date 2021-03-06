# Rust in a Nutshell

## Why Rust?

### Rust vs. C, C++, Objective-C

### Rust vs. Java, Kotlin, C\#, Swift

### Rust vs. JavaScript, Python, Ruby

### Rust vs. Haskell, OCaml, F\#, Elixir

!include ownership-and-references.md

## Cargo

## Cargo Doc

## Testing

## Crates

## Basic Types

### Bool

### Char

### Floats

### Integers

### Saturating Operations

### Unsigned Integers

!include aggregate-types.md

## Pattern Matching

## Error Handling at Compile Time

### Option

### Error

## Impl

## Traits

## Iterators

## Data Structures

### Vec

Rust's growable array type is called `Vec`, short for `vector` (which
comes from C++). In python any many functional languages it's called a
list, and in other languages, an array.

To create a vector, one can use this syntax:

```rs
let mut v = Vec::new();
```

The main operation of a vector is `push`, which appends one element to
the end of the vector.

```rs
let mut v = Vec::new();
v.push(5); // The vector now looks like this: [5].
```

Pushing has an amortized time complexity of O(1). Pushing to a vector
has a worst case time complexity of O(n), because vectors dynamically
grow.

If a vector is full, and you push back to a vector, the vector must do
the following:

1. Allocate a buffer that is twice the size of its previous buffer
2. Copy over its current items to the new buffer
3. Add the new element to the buffer.
4. Free the previous buffer.

The first, third, and fourth step all take constant time, but copying
over every item from the previous to the new buffer takes linear (O(n))
time. That being said, as long as you avoid this case as much as
possible (which you can mitigate by doubling the buffer size every time)
the time complexity for pushing to a vector is constant time on average.

If you want to check the contents of a vector at any given time, you can
print it:

```rs
let mut v = Vec::new();
v.push(5);
println!("{:?}", v);
```

### VecDeque

A VecDeque is a Doubly-Ended Queue implemented as a Vector. A VecDeque
allows for O(1) appends and pops from either end of the queue, which
basically makes it a stack and a queue in one data structure.

### LinkedList

A LinkedList is a doubly linked list.

There are two Key-Value data structures in the Rust Standard Library.

### HashMap

A HashMap is an Unordered Map. That means that getting, inserting,
updating, or deleting a value from this data structure is done in O(1)
time.

### BTreeMap

A BTreeMap is an Ordered Map. That means that getting, inserting,
updating, or deleting a value from this data structure is done in O(log
n) time.

### HashSet

### BTreeSet

### BinaryHeap

## Algorithms

### Binary Search

## Counting in O(1) space with slices

!include regex.md

## Derive Macros

## Memory

### Swap

Swap does what it says: swaps values at two mutable locations.
This is particularly useful for linked list problems, where you may need
to swap values.

```rs
pub fn swap<T>(x: &mut T, y: &mut T)
```

```rs
use std::mem;

let mut x = 5;
let mut y = 42;

mem::swap(&mut x, &mut y);

assert_eq!(42, x);
assert_eq!(5, y);
```

### Take

Take grabs a value from a location, and then replaces that with the
default value of `T`, while returning the value that was previously at
that location.

```rs
pub fn take<T>(dest: &mut T) -> T where
    T: Default,
```

```rs
use std::mem;

let mut v: Vec<i32> = vec![1, 2];

let old_v = mem::take(&mut v);
assert_eq!(vec![1, 2], old_v);
assert!(v.is_empty());
```

### Replace

Replace moves a value into a mutable reference, and returns the value
that was previously there.

```rs
pub fn replace<T>(dest: &mut T, src: T) -> T
```

```rs
use std::mem;

let mut v: Vec<i32> = vec![1, 2];

let old_v = mem::replace(&mut v, vec![3, 4, 5]);
assert_eq!(vec![1, 2], old_v);
assert_eq!(vec![3, 4, 5], v);
```

## Smart Pointers

### Box

Box is a smart pointer that points to data on the heap. Box is analogous
to C++'s `std::unique_ptr` in that it is a pointer for data on the heap.

```rs
let five = Box::new(5); // create a new pointer
```

This is useful for data that doesn't have a size that is known at
compile time:

```rs
enum List {
  Cons(i32, Box<List>),
  Nil,
} // this compiles because data on the heap can be unsized

enum List {
  Cons(i32, List),
  Nil,
} // the compiler doesn't accept unsized values on the stack
```
