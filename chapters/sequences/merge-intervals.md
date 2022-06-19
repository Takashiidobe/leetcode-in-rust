## Merge Intervals

### Problem

> Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

### Intuition

In this problem, we are given a list of intervals where the first item
in the interval is the start interval, and the second item is the end
interval. We are then given the task to merge all overlapping intervals.

First, since we need to merge overlapping intervals, we need to sort the
input by their start time.

Then, we want to iterate through our sorted array in pairs: if the start
time of the second item is before the end time of the first item, we can
merge the intervals. To merge an interval, we take the earliest start
time and the latest end time.

Afterwards, we increment both pointers by one and move onto the next
interval, repeating until we hit the end.

### Test Cases

```{.rs include=src/questions/sequences/merge_intervals.rs startLine=3 endLine=27}

```

### Answer

```{.rs include=src/questions/sequences/merge_intervals.rs startLine=29 endLine=36}

```
