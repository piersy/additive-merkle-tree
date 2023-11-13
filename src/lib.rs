/// Tree contains the peaks of the tree currently under construction.
pub struct Tree<'a> {
    pub peaks: Vec<Vec<u8>>,
    pub hasher: &'a dyn Hasher,
}

pub trait Hasher {
    fn hash2(&self, a: &[u8], b: &[u8]) -> Vec<u8>;
    fn hash1(&self, a: &[u8]) -> Vec<u8>;
}

const EMPTY_LEAF: &[u8; 9] = b"emptyLeaf";

impl<'a> Tree<'a> {
    pub fn add(&mut self, leaf: &[u8]) {
        let mut node = self.hasher.hash1(leaf);
        // Hash upwards
        for ele in self.peaks.iter_mut() {
            if ele.len() == 0 {
                *ele = node;
                return;
            } else {
                node = self.hasher.hash2(ele, &node);
                *ele = vec![];
            }
        }
        // If we didn't break out early then we are adding a new higher peak
        self.peaks.push(node);
    }

    pub fn root(&self) -> Vec<u8> {
        let mut node: Vec<u8> = Vec::new();
        let empty_leaf_hash = self.hasher.hash1(EMPTY_LEAF);
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
                    node = self.hasher.hash2(&ele, &empty_leaf_hash);
                    continue;
                }
            }

            // If we've set node and we have an empty peak, mix node with the empty node
            if ele.len() == 0 {
                node = self.hasher.hash2(&node, &empty_leaf_hash);
            // If we've set node and we have a non empty peak, mix the peak with the node
            } else {
                node = self.hasher.hash2(ele, &node);
            }
        }
        node
    }
}
