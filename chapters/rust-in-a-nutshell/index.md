# Rust in a Nutshell

## Why Rust?

### Rust vs. C, C++, Objective-C

### Rust vs. Java, Kotlin, C\#, Swift

### Rust vs. JavaScript, Python, Ruby

### Rust vs. Haskell, OCaml, F\#, Elixir

## Ownership and References

Rust's most unique feature is its ownership system, which can be summed
up thusly:

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Any data can be referred to by either:

1. A single mutable reference
2. Many immutable references

This is useful because having more than one _active_ mutable reference
can cause issues.

To demonstrate that, in this code below, we create a vector with one
element, 5. Then we take an immutable reference to it, but mutably
append the reference to it.

```cpp
#include <vector>
using namespace std;

int main() {
  vector<int> vec = {5};
  const int &first = vec.front();
  for (int i = 0; i < 10; ++i) vec.push_back(first);
  for (const int item: vec) cout << item << '\n';
}
```

This causes iterator invalidation in C++, where `first` will eventually
point to unowned memory, and will be pushed to vec. Vec will then
point to unowned memory. This causes `Undefined Behavior`, where the
program will behave non-deterministically.

Rust actually stops this in its tracks. First, we convert the above
program to rust:

```rs
fn main() {
  let mut v = vec![5];
  let first = &v[0];
  for _ in 1..10 { v.push(*first); }
  for p in v { println!("{}", p); }
}
```

And if compiled, the compiler gives this error:

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> <source>:4:20
  |
3 |   let first = &v[0];
  |                - immutable borrow occurs here
4 |   for _ in 1..10 { v.push(*first); }
  |                    ^^^^^^^------^
  |                    |      |
  |                    |      immutable borrow later used here
  |                    mutable borrow occurs here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
```

The error points us to the exact line and the error, which says that a
mutable reference on `v` cannot exist at the same time as an immutable
borrow on `v`. This could allow for iterator invalidation, and the
compiler catches this for us.

If you're more used to a language with garbage collection, you might
recall that there is a distinction between `value types` and `reference types`.

In JavaScript:

Numbers are value types so mutating the variable passed into a function
doesn't have an effect on the variable.

```js
> a = 10;
10
> b = 20;
20
> const add = (a, b) => a + b;
undefined
> add(a, b);
30
> a
10 // a is still 10
```

But for arrays, which are objects, and thus reference types:

```js
> x = [];
[]
> x
[]
> const append = (array, item) => array.push(item);
undefined
> append(x, 10);
1
> x
[ 10 ] // x is now [ 10 ]
```

This is meant to ease programming, but it can be hard to remember which
types are value types. (For example, strings in JavaScript are heap
allocated, but are treated as value types).

In Rust, functions can take ownership of values by moving values into
the function:

```rs
fn main() {
  let v = vec![5];
  fn push_and_print(mut v: Vec<i32>) {
    v.push(10);
    println!("{:?}", v);
  }
  push_and_print(v);
  // println!("{:?}", v); // This now causes a compiler error, as v is not in scope
}
```

If the last line is uncommented:

```
error[E0382]: borrow of moved value: `v`
 --> <source>:8:20
  |
2 |   let v = vec![5];
  |       - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
...
7 |   push_and_print(v);
  |                  - value moved here
8 |   println!("{:?}", v); // This now causes a compiler error, since v has been moved into `push_and_print`.
  |                    ^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
```

Functions can take a mutable reference, which does not transfer
ownership:

```rs
fn main() {
  let mut v = vec![5];
  fn push_and_print(v: &mut Vec<i32>) {
    v.push(10);
    println!("{:?}", v);
  } // the mut reference is returned to the outer scope here
  push_and_print(&mut v);
  println!("{:?}", v); // this no longer causes a compiler error
}
```

Or they can take an immutable reference, which does not transfer
ownership and disallows mutating the data behind the reference:

```rs
fn main() {
  let v = vec![5];
  fn push_and_print(v: &Vec<i32>) {
    // v.push(10); // mutating v is no longer allowed
    println!("{:?}", v);
  } // the reference is returned to the outer scope here
  push_and_print(&v);
  println!("{:?}", v); // no compiler error. V is in scope.
}
```

Value types (which in Rust implement a trait called `Copy`) can be
treated similarly, except they are not moved into functions.

They are copied into a function by default:

```rs
fn main() {
  let num = 10;
  fn add_and_print(mut num: i32) {
    num += 10;
    println!("{}", num);
  }
  add_and_print(num); // 20
  println!("{}", num); // 10
}
```

They can be treated as a mutable reference:

```rs
fn main() {
  let mut num = 10;
  fn add_and_print(num: &mut i32) {
    *num += 10;
    println!("{}", num);
  } // the reference is returned to the outer scope here
  add_and_print(&mut num); // 20
  println!("{}", num); // 20 // no compiler error. num is in scope
}
```

They can be treated as an immutable reference:

```rs
fn main() {
  let num = 10;
  fn add_and_print(num: &i32) {
    // *num += 10; // mutation is no longer allowed
    println!("{}", num);
  } // the reference is returned to the outer scope here
  add_and_print(&num); // 20
  println!("{}", num); // 20 // no compiler error. num is in scope
}
```

All in all, Rust allows finer grained control of the lifetimes of
variables, and functions show if they take ownership of variables that are
passed in, or if they take a reference, or if they mutate the underlying
data.

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

## Regex

## Derive Macros

## Smart Pointers
