use std::ops::Range;

use candid::Nat;

/// Constructs a range starting at `start` and spanning `len` integers.
/// If `start` + `len` overflows Nat, the len is truncated to the largest value that doesn't overflow
/// Nat.
pub fn make_range(start: Nat, len: usize) -> Range<Nat> {
    Range {
        start: start.clone(),
        end: start + len as u64,
    }
}

/// Constructs an intersection of two ranges.
pub fn intersect(l: &Range<Nat>, r: &Range<Nat>) -> Range<Nat> {
    Range {
        start: l.start.clone().max(r.start.clone()),
        end: l.end.clone().min(r.end.clone()),
    }
}

/// Returns true iff `r` contains each point of `l`.
pub fn is_subrange(l: &Range<Nat>, r: &Range<Nat>) -> bool {
    r.start <= l.start && l.end <= r.end
}

/// Returns the total number of elements in range `r`.
pub fn range_len(r: &Range<Nat>) -> Nat {
    r.end.clone() - r.start.clone()
}

/// Returns the prefix of the range `r` that contains at most `n` elements.
pub fn head(r: &Range<Nat>, n: usize) -> Range<Nat> {
    Range {
        start: r.start.clone(),
        end: r.end.clone().min(r.start.clone() + n as u64),
    }
}

/// Constructs an interval by dropping at most `n` first elements of range `r`.
pub fn behead(r: &Range<Nat>, n: usize) -> Range<Nat> {
    Range {
        start: r.end.clone().min(r.start.clone() + n as u64),
        end: r.end.clone(),
    }
}

/// Constructs an interval by removing at most `n` last elements of range `r`.
pub fn curtail(r: &Range<Nat>, n: usize) -> Range<Nat> {
    Range {
        start: r.start.clone(),
        end: r.start.clone() + range_len(r) - (n as u64),
    }
}
