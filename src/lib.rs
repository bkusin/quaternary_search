use std::arch::x86_64::*;

// returns index of match, or None if not found
pub fn quaternary_search(data: &[i32], x: i32) -> Option<usize> {
   let mut base: usize = 0;
   let mut quarter = data.len();

    while quarter > 4 {

        quarter = quarter / 4;
   
        unsafe {
            // _mm_i32gather_epi32 uses byte offsets, so scale by the i32 size.
            // repeat 3*offset because we don't want 4*offset
            let offsets = _mm_setr_epi32(quarter as i32, 2*quarter as i32, 3*quarter as i32, 3*quarter as i32);
            let points = _mm_i32gather_epi32::<4>(data.as_ptr().add(base), offsets);
            let mut cmps: __mmask8 = _mm_cmp_epi32_mask( points, _mm_set1_epi32(x), _MM_CMPINT_LE);
            // zero out the fourth comparison's bit
            // note the order - lanes 0,1,2,3 will have bit results in the opposite order in the mask from right
            let mask: __mmask8 = 0b00001000;
            cmps = _kandn_mask8(mask, cmps);

            // don't need to explicitly convert with _cvtmask8_u32(cmps) or even use SIMD intrinsic - the compiler does it
            let matches = cmps.count_ones();

            base = base + (matches as usize * quarter);
        }
    }

    for i in 0..4 {
        if base + i < data.len() && data[base + i] == x { return Some(base + i) }
    }

    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn simd_ops() {
        let data = [1,2,3,4,5,6,7,8,9,10];
        unsafe {
            let quarter = 2;
            let offsets = _mm_setr_epi32(quarter as i32, 2*quarter as i32, 3*quarter as i32, 3*quarter as i32);
            let mut lanes = [0_i32; 4];

            let points = _mm_i32gather_epi32::<4>(data.as_ptr(), offsets);
            _mm_storeu_si128(lanes.as_mut_ptr().cast(), points);
            assert_eq!(lanes, [3, 5, 7, 7]);

            let mut cmps: __mmask8 = _mm_cmp_epi32_mask( points, _mm_set1_epi32(6), _MM_CMPINT_LE);
            assert_eq!(cmps, 0b00000011);
        }
    }

    #[test]
    fn small_list() {
        let data = [1,2];
        let result = quaternary_search(&data, 2);

        assert_eq!(result, Some(1));
    }

    #[test]
    fn out_of_range_left() {
        let data = [1,2,3,4,5,6,7,8,9,10];
        let result = quaternary_search(&data, -1);

        assert_eq!(result, None);
    }

    #[test]
    fn out_of_range_right() {
        let data = [1,2,3,4,5,6,7,8,9,10];
        let result = quaternary_search(&data, 11);

        assert_eq!(result, None);
    }

    #[test]
    fn out_of_range_len_20() {
        let data: [i32; 20] = std::array::from_fn(|i| i as i32);
        let result = quaternary_search(&data, 21);

        assert_eq!(result, None);
    }

    #[test]
    fn target_is_first() {
        let data = [1,2,3,4,5,6,7,8,9,10];
        let result = quaternary_search(&data, 1);

        assert_eq!(result, Some(0));
    }

    #[test]
    fn target_is_last() {
        let data = [1,2,3,4,5,6,7,8,9,10];
        let result = quaternary_search(&data, 10);

        assert_eq!(result, Some(9));
    }

    #[test]
    fn find_everything() {
        let data = [1,2,3,4,5,6,7,8,9,10];
        for (idx, n) in data.iter().enumerate() {
            let result = quaternary_search(&data, *n);
            assert_eq!(result, Some(idx));
        }
    }
}

