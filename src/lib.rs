#[inline]
pub fn sum_up(n: i64) -> i64 {
    // This structure matches how the SIMD function works and is not optimized away by the compiler
    (0..=n).into_iter().fold(0, |a, x| sum(a, x))
}

fn sum(a: i64, x: i64) -> i64 {
    a + x
}

use std::arch::x86_64::{_mm256_add_epi64, _mm256_set_epi64x};

#[inline]
#[allow(unsafe_code)]
pub fn sum_up_simd(slice: &[i64]) -> i64 {
    // SAFETY: I am running this on an x86_64 chip that I know has AVX2
    match slice {
        // use simd if we have 16 values
        // SAFETY: We know all of these values will be defined
        &[v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16] => unsafe {
            let vec1 = _mm256_set_epi64x(v1, v2, v3, v4);
            let vec2 = _mm256_set_epi64x(v5, v6, v7, v8);
            let vec3 = _mm256_set_epi64x(v9, v10, v11, v12);
            let vec4 = _mm256_set_epi64x(v13, v14, v15, v16);

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
        assert_eq!(sum_up(5), 15);
    }

    #[test]
    fn simd_works() {
        let mut nums: Vec<i64> = Vec::new();
        for i in 1..=20 {
            nums.push(i);
        }
        assert_eq!(
            nums.chunks(16).map(|c| sum_up_simd(c)).sum::<i64>(),
            210_i64
        );
    }
}
