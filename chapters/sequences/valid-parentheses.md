## Valid Parentheses

### Problem

> Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
>
> An input string is valid if:
>
> Open brackets must be closed by the same type of brackets.
> Open brackets must be closed in the correct order.

### Intuition

The input string has three possible opening characters and three
possible closing characters.

In order for the string to be valid, we must fulfill two conditions:

1. Every opening char must be correctly matched to a closing char.
2. All opening chars must come first in the string.

From the second condition we know the following:

1. If we encounter a closing char and we did not previously encounter an
   opening char, our string cannot be valid.
2. If we encounter a closing char and we did not encounter the correct
   opening char just before, our string cannot be valid.
3. An input string is not valid if there are more opening or closing
   characters than the other.

So, we need a data structure where we can remember the last opening
character we encountered, and remove it. To do so, we need to use a
stack.

We will use a vector as a stack. The stack keeps track of all the
opening characters we encountered in reverse order.

If at any point we find a closing character, we check the top of the
stack. If we don't find the correct matching character or the stack is
empty, we return false. We do this for every character in the string.

At the end, we make sure the stack is empty. If not, there were more
opening characters than closing characters, which makes the string
invalid.

### Test Cases

```{.rs include=src/questions/sequences/valid_parentheses.rs startLine=3 endLine=27}

```

### Answer

```{.rs include=src/questions/sequences/valid_parentheses.rs startLine=29 endLine=36}

```
