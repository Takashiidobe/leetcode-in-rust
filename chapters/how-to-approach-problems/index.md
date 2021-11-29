# How to Approach Problems

Much has been said about how to become a better problem solver
[@citeulike:679515]. Here we'll go over some tips and tricks to solve a
hard problem by using some of these techniques.

## A Plan of Attack

1. Build an intuition about the problem. What should the code return?
2. Write some test cases. Note any edge cases your code should take
   care of.
3. Start writing out the code, being wary of edge cases.
4. Refactor your code. How can it be improved?

Let's go over our plan step by step, using an example problem called
`Jewels and Stones`:

> You're given strings jewels representing the types of stones that are
> jewels, and stones representing the stones you have. Each character in
> stones is a type of stone you have. You want to know how many of the
> stones you have are also jewels.
>
> Letters are case sensitive, so "a" is considered a different type of
> stone from "A".

> Example 1:
>
> Input: jewels = "aA", stones = "aAAbbbb"
> Output: 3
>
> Example 2:
>
> Input: jewels = "z", stones = "ZZ"
> Output: 0

### Build Intuition

First, let's jot down some notes about the problem.
It says: `We want to know how many of the stones you have are also jewels`.
This means that we want to return a count of our jewels.
A count is going to be a unsigned integer. We can imagine that our
return type would be some unsigned integer type, like `u32`.

We can start with that:

```rs
fn jewels_and_stones(/* TODO */) -> u32 { /* TODO */ }
```

Next, let's take note of the two inputs, which are `given [as] strings`.
We can assume we are given one string for the jewels and one for the
stones.

```rs
fn jewels_and_stones(jewels: String, stones: String) -> u32 { /* TODO */ }
```

The problem also notes that we want to return the number of jewels in
our collection of stones, and that every character of jewels is a jewel,
and every character of stones is a stone.

Let's reduce the problem to something easier. Let's say that instead of
having a collection of stone(s), we have just one stone and one jewel.
Does this make the problem easier?

It should. We now only need to check if the stone is a jewel, and return
our counter at the end.

```rs
fn jewels_and_stones(jewel: char, stone: char) -> u32 {
  let mut count = 0;
  if jewel == stone {
    count += 1;
  }
  count
}
```

What if we make it so we have one stone but many jewels? What would we
do?

Well for our one stone, we would want to check every jewel to make sure
that it is a jewel, and return the count of jewels we have.

```rs
fn jewels_and_stones(jewels: String, stone: char) -> u32 {
  let mut count = 0;
  for jewel in jewels.chars() {
    if jewel == stone {
      count += 1;
    }
  }
  count
}
```

What happens if we have many stones but one jewel? We do the opposite,
where every stone that counts as a jewel increments our count by one.

```rs
fn jewels_and_stones(jewel: char, stones: String) -> u32 {
  let mut count = 0;
  for stone in stones.chars() {
    if jewel == stone {
      count += 1;
    }
  }
  count
}
```

Now that we have some intuition about how to solve simpler problems,
we'll start by writing test cases for this problem:

## Writing test cases

We'll start off by writing test cases for our simplified problems:

If the jewels and stones have a length of one, either they are the same
or not. If they are the same, this function should return 1. If not,
this function should return 0.

```rs
assert_eq!(jewels_and_stones("a".to_string(), "a".to_string()), 1);
assert_eq!(jewels_and_stones("a".to_string(), "b".to_string()), 0);
```

If there is one jewel, we iterate through our stones and increment our
count every time we find a jewel.

```rs
assert_eq!(jewels_and_stones("a".to_string(), "aac".to_string()), 2);
assert_eq!(jewels_and_stones("a".to_string(), "xyz".to_string()), 0);
```

Otherwise, if there's one stone, then we check every jewel to see if our
stone is a jewel.

```rs
assert_eq!(jewels_and_stones("abc".to_string(), "a".to_string()), 1);
assert_eq!(jewels_and_stones("xyz".to_string(), "a".to_string()), 0);
```

Finally, if there's more than one jewel and more than one stone, for
each stone, we check if it is a jewel in our set of jewels.

```rs
assert_eq!(jewels_and_stones("abc".to_string(), "cxx".to_string()), 1);
assert_eq!(jewels_and_stones("xyz".to_string(), "xxa".to_string()), 2);
```

## Writing Code

Now we can begin writing some code to tackle our original problem:

We have an intuition that for every stone, we want to check if it is a
jewel. To do this, we have to iterate through all the jewels, and
compare our stone to it. If they're the same, we can increment the
count.

```rs
fn jewels_and_stones(jewels: String, stones: String) -> u32 {
  let mut count = 0;
  for stone in stones.chars() {
    for jewel in jewels.chars() {
      if stone == jewel {
        count += 1;
        break;
      }
    }
  }
  count
}
```

This turns out to pass the tests outlined above, but it has some
problems. Time to refactor!

## Refactoring

When refactoring, let's discuss some things we can do to improve our
code:

Our code is very concise. There's not much that can be done to improve
its readability, which is a good thing. That being said, it can have a
slow runtime. If we say the length of stones is N and the length of
jewels is M, the runtime of our code grows in O(N\*M) (polynomial time). We
should be able to do better. But how?

```rs
// O(N*M)
fn jewels_and_stones(jewels: String, stones: String) -> u32 {
  let mut count = 0;
  for stone in stones.chars() { // O(N)
    for jewel in jewels.chars() { // O(M): Wouldn't it be nice if this was O(1)?
      if stone == jewel {
        count += 1;
        break;
      }
    }
  }
  count
}
```

We have a nested for loop, which contributes the slow runtime. Maybe we
could represent either jewels or stones in a different fashion, and get
rid of a for loop? Would there be a different way of representing jewels
that would make this easier? Maybe a data structure that has O(1) time
for if it contains an item?

We can use a set for this:

So our solution turns into this:

```rs
// O(N)
fn jewels_and_stones(jewels: String, stones: String) -> u32 {
  let mut count = 0;
  let jewels_set: HashSet<char> = HashSet::from_iter(jewels.chars());
  for stone in stones.chars() { // O(N)
    if jewels_set.contains(&stone) {
      count += 1;
    }
  }
  count
}
```

And we get down from O(N\*M) to O(N) time.
