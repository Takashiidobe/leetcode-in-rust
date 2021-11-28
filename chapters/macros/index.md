# Macros for Rust

## A macro for testing

Unlike C and C++, a testing framework is built into rust. We can create
our own tests by creating a `mod` block and letting cargo know that we
want to test it.

Let's say we create this function:

```{.rs include=src/add.rs startLine=1 endLine=3}

```

We can test it at the bottom of the file:

```{.rs include=src/add.rs startLine=5 endLine=18}

```

Macros let us reduce most of the boilerplate:

```{.rs include=src/lib.rs startLine=27 endLine=41}

```

Our tests can then be rewritten like so:

```{.rs include=src/add.rs startLine=20 endLine=23}

```

And running them gives us this result:

```sh
$ cargo test
running 2 tests
test test::add_one_and_one ... ok
test test::add_one_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
