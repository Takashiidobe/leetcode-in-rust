# Rust in a Nutshell

## Why Rust?

## Ownership

## Cargo

## Cargo Doc

## Testing

## Crates

## Basic Types

### Bool

### Char

### Floats

### Integers

### Saturing Operations

### Unsigned Integers

### Tuples

### Structs

### Enums

### Unit Type

## References

## Pattern Matching

## Error Handling at Compile Time

### Option

### Error

## Impl

## Traits

## Iterators

## Basic Data Structures

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

## Basic Algorithms

### Binary Search

## Counting in O(1) space with slices

## Regex

## Derive Macros

## Smart Pointers
