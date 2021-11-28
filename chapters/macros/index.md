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

Test can then be called like so:

```{.rs include=src/add.rs startLine=20 endLine=23}

```
