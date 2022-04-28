use sha2::{Digest, Sha256};

pub fn compute_hash(data: &[u8]) -> [u8; 32] {
    let mut sha = Sha256::new();
    sha.update(data);
    sha.finalize().into()
}
