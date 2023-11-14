/// Tree contains the peaks of the tree currently under construction.
pub struct Tree {
    pub peaks: Vec<Vec<u8>>,
    pub hasher: Box<dyn Hasher>,
}

pub trait Hasher {
    fn add(&mut self, a: &[u8]);
    fn finalize(&mut self) -> Vec<u8>;
}

const EMPTY_LEAF: &[u8; 9] = b"emptyLeaf";

impl Tree {
    pub fn add(&mut self, leaf: &[u8]) {
        self.hasher.add(leaf);
        let mut node = self.hasher.finalize();
        // Hash upwards
        for ele in self.peaks.iter_mut() {
            if ele.len() == 0 {
                *ele = node;
                return;
            } else {
                self.hasher.add(ele);
                self.hasher.add(&node);
                node = self.hasher.finalize();
                *ele = vec![];
            }
        }
        // If we didn't break out early then we are adding a new higher peak
        self.peaks.push(node);
    }

    pub fn root(&mut self) -> Vec<u8> {
        let mut node: Vec<u8> = Vec::new();
        self.hasher.add(EMPTY_LEAF);
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
                    self.hasher.add(&ele);
                    self.hasher.add(&empty_leaf_hash);
                    node = self.hasher.finalize();
                    continue;
                }
            }

            // If we've set node and we have an empty peak, mix node with the empty node
            if ele.len() == 0 {
                self.hasher.add(&node);
                self.hasher.add(&empty_leaf_hash);
                node = self.hasher.finalize();
            // If we've set node and we have a non empty peak, mix the peak with the node
            } else {
                self.hasher.add(ele);
                self.hasher.add(&node);
                node = self.hasher.finalize();
            }
        }
        node
    }
}
