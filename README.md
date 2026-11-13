This is a little performance experiment.
I thought that a branchless SIMD quaternary search might beat binary search.
In reality, it loses to Rust's slice::binary_search by 60% slower on my machine.

I didn't profile to find out why, but it's probably the extra reads it must do.
In short, it does fewer iterations than binary_search but more work per iteration.

That's not to say that quaternary search is bad, it just isn't good for this workload and CPU.

### Algorithm

Quaternary search uses three decision points to split the range into four subranges.

This implementation:
- performs a SIMD gather to collect those points
- uses an AVX-512 instruction to simultaneously compare them all to the target values
- the resulting __mmask8 goes in a mask register.
  Because the data is sorted, the mask will have a partition of 1's where the comparison tested True, followed by 0's.
  Those 1's represent how many subranges to skip the base pointer.
- Advance the base pointer and quarter the subrange size for the next iteration.
- When the subrange size is smaller than 5, the loop ends and the algorithm linearly searches over remaining values.
  Although it's a linear search, the cost should be low because there are only a few of them and they are packed contiguously.
