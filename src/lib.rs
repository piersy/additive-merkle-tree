/// Tree contains the peaks of the tree currently under construction.
pub struct Tree {
    pub peaks: Vec<Vec<u8>>,
    pub hasher: Box<dyn Hasher>,
}

pub trait Hasher {
    // update adds the provided bytes to the current hash instance.
    fn update(&mut self, a: &[u8]);
    // finalize computes the hash, returns it and resets the state ready to compute a new hash.
    fn finalize(&mut self) -> Vec<u8>;
    // hash1 is a convenience function that returns the hash of the given byte slice.
    fn hash1(&mut self, a: &[u8]) -> Vec<u8> {
        self.update(a);
        self.finalize()
    }
    // hash2 is a convenience function that returns the hash of the two given byte slices.
    fn hash2(&mut self, a: &[u8], b: &[u8]) -> Vec<u8> {
        self.update(a);
        self.update(b);
        self.finalize()
    }
}

const EMPTY_LEAF: &[u8; 9] = b"emptyLeaf";

impl Tree {
    pub fn add(&mut self, leaf: &[u8]) {
        self.hasher.update(leaf);
        let mut node = self.hasher.finalize();
        // Hash upwards
        for ele in self.peaks.iter_mut() {
            if ele.len() == 0 {
                *ele = node;
                return;
            } else {
                self.hasher.update(ele);
                self.hasher.update(&node);
                node = self.hasher.finalize();
                *ele = vec![];
            }
        }
        // If we didn't break out early then we are adding a new higher peak
        self.peaks.push(node);
    }

    pub fn root(&mut self) -> Vec<u8> {
        let mut node: Vec<u8> = Vec::new();
        self.hasher.update(EMPTY_LEAF);
        let empty_leaf_hash = self.hasher.finalize();
        for i in 0..self.peaks.len() {
            let ele = &self.peaks[i];
            // If we have no node and there is no peak then go to next peak
            if node.len() == 0 {
                // We are at the last peak and we did not set node, then the last peak is the root.
                if i == self.peaks.len() - 1 {
                    return self.peaks.last().unwrap().clone();
                }
                if ele.len() == 0 {
                    continue;
                } else {
                    self.hasher.update(&ele);
                    self.hasher.update(&empty_leaf_hash);
                    node = self.hasher.finalize();
                    continue;
                }
            }

            // If we've set node and we have an empty peak, mix node with the empty node
            if ele.len() == 0 {
                self.hasher.update(&node);
                self.hasher.update(&empty_leaf_hash);
                node = self.hasher.finalize();
            // If we've set node and we have a non empty peak, mix the peak with the node
            } else {
                self.hasher.update(ele);
                self.hasher.update(&node);
                node = self.hasher.finalize();
            }
        }
        node
    }
}
