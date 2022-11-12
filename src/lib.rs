// Standard
#[inline]
pub fn sum_up(nums: Vec<i64>) -> i64 {
    // This structure matches how the SIMD function works and is not optimized away by the compiler
    nums.iter().fold(0, |a, x| regular_math(a, *x))
}

fn regular_math(a: i64, x: i64) -> i64 {
    // Normally, you'd do more complicated math than an addition - for example, calculate a
    // squared error
    a + x
}

// SIMD
use std::arch::x86_64::{_mm256_add_epi64, _mm256_set_epi64x};

#[inline]
pub fn simd_sum_up(nums: Vec<i64>) -> i64 {
    // Iterate over 16 values at a time
    // Each time we do `.map(|c| simd_math(c))`, it results in a vec that is 1/16 the length of the
    // original.
    // Chaining results in worse performance
    nums.chunks(16).map(|c| simd_math(c)).sum::<i64>()
}

#[allow(unsafe_code)]
fn simd_math(slice: &[i64]) -> i64 {
    // SAFETY: I am running this on an x86_64 chip that I know has AVX2
    match slice {
        // Use simd if we have 16 values
        &[v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16] => unsafe {
            // SAFETY: We know all of these values will be defined
            // Set four packed 64-bit vectors
            let vec1 = _mm256_set_epi64x(v1, v2, v3, v4);
            let vec2 = _mm256_set_epi64x(v5, v6, v7, v8);
            let vec3 = _mm256_set_epi64x(v9, v10, v11, v12);
            let vec4 = _mm256_set_epi64x(v13, v14, v15, v16);

            // `transmute()` unpacks the 256-bit register into 4 i64 values
            // `_mm256_add_epi64 performs the pair-wise sum across the vectors
            let (r1, r2, r3, r4): (i64, i64, i64, i64) = std::mem::transmute(_mm256_add_epi64(
                _mm256_add_epi64(vec1, vec2),
                _mm256_add_epi64(vec3, vec4),
            ));

            r1 + r2 + r3 + r4
        },
        // If we don't have 16 values, just sum them
        other => other.into_iter().sum::<i64>(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum_up((0..=5).collect::<Vec<i64>>()), 15);
    }

    #[test]
    fn simd_works() {
        assert_eq!(simd_sum_up((0..=20).collect::<Vec<i64>>()), 210);
    }
}
