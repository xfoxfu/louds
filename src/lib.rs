mod fid;
mod louds;

#[cfg(test)]
mod louds_test;

pub use crate::louds::{BitIndex, Louds, NodeId};

pub struct TreeNode {
    pub children: Vec<TreeNode>,
}

impl Into<Louds> for TreeNode {
    fn into(self) -> Louds {
        Louds::from_bit_vec(&*Into::<Vec<bool>>::into(self))
    }
}

impl Into<Vec<bool>> for TreeNode {
    fn into(self) -> Vec<bool> {
        let mut bits = Vec::with_capacity(self.children.len());
        // encode current node
        bits.resize(self.children.len(), true);
        bits.push(false);
        // encode children
        bits.extend(
            self.children
                .into_iter()
                .flat_map(|child| Into::<Vec<bool>>::into(child)),
        );
        bits
    }
}
