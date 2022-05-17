use num_bigint::BigUint;
use std::ops::Range;

/// Constructs a range starting at `start` and spanning `len` integers.
/// If `start` + `len` overflows BigUint, the len is truncated to the largest value that doesn't overflow
/// BigUint.
pub fn make_range(start: BigUint, len: usize) -> Range<BigUint> {
    Range {
        start: start.clone(),
        end: start + len as u64,
    }
}

/// Constructs an intersection of two ranges.
pub fn intersect(l: &Range<BigUint>, r: &Range<BigUint>) -> Range<BigUint> {
    Range {
        start: l.start.clone().max(r.start.clone()),
        end: l.end.clone().min(r.end.clone()),
    }
}

/// Returns true iff `r` contains each point of `l`.
pub fn is_subrange(l: &Range<BigUint>, r: &Range<BigUint>) -> bool {
    r.start <= l.start && l.end <= r.end
}

/// Returns the total number of elements in range `r`.
pub fn range_len(r: &Range<BigUint>) -> BigUint {
    r.end.clone() - r.start.clone()
}

/// Returns the prefix of the range `r` that contains at most `n` elements.
pub fn head(r: &Range<BigUint>, n: usize) -> Range<BigUint> {
    Range {
        start: r.start.clone(),
        end: r.end.clone().min(r.start.clone() + n as u64),
    }
}

/// Constructs an interval by dropping at most `n` first elements of range `r`.
pub fn behead(r: &Range<BigUint>, n: usize) -> Range<BigUint> {
    Range {
        start: r.end.clone().min(r.start.clone() + n as u64),
        end: r.end.clone(),
    }
}

/// Constructs an interval by removing at most `n` last elements of range `r`.
pub fn curtail(r: &Range<BigUint>, n: usize) -> Range<BigUint> {
    Range {
        start: r.start.clone(),
        end: r.start.clone() + range_len(r) - (n as u64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_range() {
        assert_eq!(
            make_range(BigUint::from(0u32), 0),
            Range {
                start: BigUint::from(0u32),
                end: BigUint::from(0u32),
            }
        );
        assert_eq!(
            make_range(BigUint::from(0u32), 1),
            Range {
                start: BigUint::from(0u32),
                end: BigUint::from(1u32),
            }
        );
        assert_eq!(
            make_range(BigUint::from(10u32), 15),
            Range {
                start: BigUint::from(10u32),
                end: BigUint::from(25u32),
            }
        );
    }

    #[test]
    fn test_intersect() {
        let range1 = make_range(BigUint::from(10u32), 15);
        let range2 = make_range(BigUint::from(20u32), 25);
        let range3 = make_range(BigUint::from(20u32), 5);
        assert_eq!(intersect(&range1, &range2), range3);

        let range1 = make_range(BigUint::from(0u32), 1000);
        let range2 = make_range(BigUint::from(2000u32), 1000);
        assert_eq!(intersect(&range1, &range2), Range { start: BigUint::from(2000u32), end: BigUint::from(1000u32) });

        assert_eq!(head(&Range { start: BigUint::from(2000u32), end: BigUint::from(1000u32) }, 100), Range { start: BigUint::from(2000u32), end: BigUint::from(1000u32) });
    }

    #[test]
    fn test_is_subrange() {
        let range1 = make_range(BigUint::from(10u32), 15);
        let range2 = make_range(BigUint::from(10u32), 25);
        let range3 = make_range(BigUint::from(20u32), 25);
        assert!(is_subrange(&range1, &range2));
        assert!(!is_subrange(&range1, &range3));
        assert!(!is_subrange(&range2, &range3));
    }

    #[test]
    fn test_range_len() {
        let range1 = make_range(BigUint::from(10u32), 15);
        let range2 = make_range(BigUint::from(10u32), 25);
        let range3 = make_range(BigUint::from(20u32), 5);
        assert_eq!(range_len(&range1), BigUint::from(15u32));
        assert_eq!(range_len(&range2), BigUint::from(25u32));
        assert_eq!(range_len(&range3), BigUint::from(5u32));
    }

    #[test]
    fn test_head() {
        let range1 = make_range(BigUint::from(10u32), 15);
        let range2 = make_range(BigUint::from(10u32), 25);
        assert_eq!(head(&range2, 15), range1);
    }

    #[test]
    fn test_behead() {
        let range1 = make_range(BigUint::from(10u32), 15);
        let range2 = make_range(BigUint::from(20u32), 5);
        assert_eq!(behead(&range1, 10), range2);
    }

    #[test]
    fn test_curtail() {
        let range1 = make_range(BigUint::from(10u32), 15);
        let range2 = make_range(BigUint::from(10u32), 25);
        assert_eq!(curtail(&range2, 10), range1);
    }
}
