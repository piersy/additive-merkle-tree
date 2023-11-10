/// Tree contains the peaks of the tree currently under construction.
pub struct Tree<'a> {
    pub peaks: Vec<Vec<u8>>,
    pub hasher: &'a dyn Hasher,
}

pub trait Hasher {
    fn hash2(&self, a: &[u8], b: &[u8]) -> Vec<u8>;
    fn hash1(&self, a: &[u8]) -> Vec<u8>;
}

impl<'a> Tree<'a> {
    pub fn add(&mut self, leaf: &[u8]) {
        let mut node = self.hasher.hash1(leaf);
        for ele in self.peaks.iter_mut() {
            if ele.len() == 0 {
                *ele = node;
                return;
            } else {
                node = self.hasher.hash2(&node, ele);
            }
        }
        self.peaks.push(node);
    }
    pub fn root(&self) -> &[u8] {
        self.peaks.last().unwrap().as_ref()
        // self.peaks.get(0).unwrap().as_ref()
    }
}
