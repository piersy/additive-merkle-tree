#[cfg(test)]
// mod tests {
use additive_merkle_tree::{Hasher, Tree};
use sha2::{Digest, Sha256};
// #[test]
// fn tree_generates_correct_root() {
//     assert_eq!(2 + 2, 4);
// }

struct TestHasher {}
impl Hasher for TestHasher {
    fn hash2(&self, a: &[u8], b: &[u8]) -> Vec<u8> {
        Sha256::new()
            .chain_update(a)
            .chain_update(b)
            .finalize()
            .to_vec()
    }
    fn hash1(&self, a: &[u8]) -> Vec<u8> {
        Sha256::new().chain_update(a).finalize().to_vec()
    }
}

#[test]
fn testhash() {
    let mut t = Tree {
        peaks: Vec::new(),
        hasher: &TestHasher {},
    };
    t.add(b"a");
    t.add(b"b");
    t.add(b"c");
    t.add(b"d");

    let h = &TestHasher {};
    let a = h.hash1(b"a");
    let b = h.hash1(b"b");
    let c = h.hash1(b"c");
    let d = h.hash1(b"d");

    let ab = h.hash2(&a, &b);
    let cd = h.hash2(&c, &d);
    let abcd = h.hash2(&ab, &cd);

    assert_eq!(t.root(), abcd)
}
