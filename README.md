# A simple SIMD example in Rust

## SAFETY

__This code is only safe to run on x86_64 chips that support AVX2. Absolutely no attempt is made to
check if AVX2 intrinsics are available at runtime. This code assumes that they are.__

## Overview

This is a simple example of using SIMD (single-instruction multiple-data, a.k.a. vector) operations
in Rust on x86_64 architecture with AVX2. I use a trivial case of summing the integers from 1 to
1,000,000 to keep the focus on the operations.

Below I discuss the performance difference of a standard summation vs. summation using SIMD
operations.

## Notes on the code

The code uses the `_mm256_add_epi64` intrinsic, which adds `i64` values pair-wise across two
vectors of four packed `i64` values (i.e. summing two vectors of length four results in one vector
of length four). To take advantage of more SIMD operations, this code takes 16 values at a time,
sums two pairs of vetors together with `_mm256_add_epi64`, sums the two resulting vectors with the
same operation, unpacks the resulting vector into four `i64` values, and finally sums those `i64`s
into one. Of course, we could write this in a similar way to take any multiple-of-four values,
creating something of a summation tree, but that's outside the scope of this demo.

If 16 values aren't available to sum (like at the end of the vector), the code simply sums whatever
values are left.

SIMD intrinsics require unsafe Rust because they are architecture- ,and really chip-, specific.
They commonly provide access to a single CPU instruction that may not be available on every chip.

### Use of intrinsics

There are a handful of crates out there that try to make SIMD operations easier to incorporate or
portable across architectures. I haven't used these personally, so I can't opine on their use. I
use intrinsics here because I think it's helpful to see how these things really work first-hand.

The documentation on SIMD intrinsics in Rust is
[here](https://doc.rust-lang.org/core/arch/index.html).

## Results

Below are the results of micro-benchmarking the difference between the standard summation and SIMD
summation using the [`criterion` crate](https://docs.rs/criterion/latest/criterion/).

Having run these micro-benchmarks many times, there is generally a 16-17% performance with the SIMD
operations, with a slight improvement in outliers.

__Using `sum_up`__
```
Benchmarking sum up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. [...]
Benchmarking sum up: Collecting 100 samples in estimated 6.7887 s (5050 iterations)

sum up                  time:   [1.3366 ms 1.3384 ms 1.3406 ms]
Found 18 outliers among 100 measurements (18.00%)
  6 (6.00%) high mild
  12 (12.00%) high severe
```

Here I used the same benchmark but changed the summation function to the SIMD version. This shows
the % improvement over the standard summation.

__Using `simd_sum_up`__
```
Benchmarking sum up: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. [...]
Benchmarking sum up: Collecting 100 samples in estimated 5.6438 s (5050 iterations)

sum up                  time:   [1.1133 ms 1.1139 ms 1.1149 ms]
                        change: [-16.972% -16.829% -16.677%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) low mild
  4 (4.00%) high mild
  4 (4.00%) high severe
```

## Summary

The take-away from this example is that we can achieve _some_ performance improvement by
carefully using SIMD operations in large calculations. The extent of the improvement will likely
vary considerably based on the math we are doing (probably much more than a simple sum), the extent
to which we can translate our math into SIMD operations and work directly in the vectors, and how
much explicit code we want to write (see "Notes on the code" above).
