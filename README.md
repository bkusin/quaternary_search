This is a little performance experiment.
I thought that a branchless SIMD quaternary search might beat binary search.
In reality, it loses to Rust's slice::binary_search by 60% slower on my machine.

I didn't profile to find out why, but it's probably the extra reads it must do.
In short, it does fewer iterations than binary_search but more work per iteration.

That's not to say that quaternary search is bad, it just isn't good for this workload and CPU.
It might outperform binary_search if, for example, the comparisons were very compute-heavy and the CPU had more load ports so it could do all the reads simultaneously (mine has 2).
But you don't know until you test.
