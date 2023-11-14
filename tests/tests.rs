#[cfg(test)]
use additive_merkle_tree::{Hasher, Tree};
use sha2::{Digest, Sha256};

struct TestHasher {
    h: Sha256,
}

impl<'a> Hasher for TestHasher {
    fn update(&mut self, a: &[u8]) {
        self.h.update(a);
    }
    fn finalize(&mut self) -> Vec<u8> {
        self.h.finalize_reset().to_vec()
    }
}

#[test]
fn testhash() {
    let mut t = Tree {
        peaks: Vec::new(),
        hasher: Box::new(TestHasher { h: Sha256::new() }),
    };
    t.add(b"a");
    t.add(b"b");
    t.add(b"c");
    t.add(b"d");

    let h = &mut TestHasher { h: Sha256::new() };
    let a = h.hash1(b"a");
    let b = h.hash1(b"b");
    let c = h.hash1(b"c");
    let d = h.hash1(b"d");

    let ab = h.hash2(&a, &b);
    let cd = h.hash2(&c, &d);
    let abcd = h.hash2(&ab, &cd);

    assert_eq!(t.root(), abcd)
}
