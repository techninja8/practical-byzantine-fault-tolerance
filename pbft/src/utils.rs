use sha2::{Sha256, Digest};

pub fn hash_data(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

// TODO: Implement digital signatures using secp256k1
