use sha3::Digest;

fn hash_slice<H: Digest>(input: &[u8]) -> Vec<u8> {
    let mut hasher = H::new();
    hasher.update(input);
    hasher.finalize().as_slice().to_owned()
}
