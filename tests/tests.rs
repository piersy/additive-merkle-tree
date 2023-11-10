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
    assert_eq!(t.root().len(), 32)
    // println!("root {:?}", t.root());
}

// }
//
// let mut hasher = Sha256::new();
// let data = b"Hello world!";
// hasher.update(data);
// // `input` can be called repeatedly and is generic over `AsRef<[u8]>`
// hasher.update("String data");
// // Note that calling `finalize()` consumes hasher
// let hash = hasher.finalize();
// println!("Result: {:x}", hash);
