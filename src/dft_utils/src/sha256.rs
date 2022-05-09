use sha2::{Digest, Sha256};

pub fn compute_hash(data: &[u8]) -> [u8; 32] {
    let mut sha = Sha256::new();
    sha.update(data);
    sha.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_hash() {
        let data = b"Hello, world!";
        let hash = compute_hash(data);
        assert_eq!(
            hash,
            [
                49, 95, 91, 219, 118, 208, 120, 196, 59, 138, 192, 6, 78, 74, 1, 100, 97, 43, 31,
                206, 119, 200, 105, 52, 91, 252, 148, 199, 88, 148, 237, 211
            ]
        );
    }
}
