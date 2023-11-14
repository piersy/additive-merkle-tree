#[cfg(test)]
use additive_merkle_tree::{Hasher, Tree};
use sha2::{Digest, Sha256};

struct TestHasher {
    h: Sha256,
}

impl<'a> Hasher for TestHasher {
    fn add(&mut self, a: &[u8]) {
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
    h.add(b"a");
    let a = h.finalize();
    h.add(b"b");
    let b = h.finalize();
    h.add(b"c");
    let c = h.finalize();
    h.add(b"d");
    let d = h.finalize();

    h.add(&a);
    h.add(&b);
    let ab = h.finalize();
    h.add(&c);
    h.add(&d);
    let cd = h.finalize();
    h.add(&ab);
    h.add(&cd);
    let abcd = h.finalize();

    assert_eq!(t.root(), abcd)
}
