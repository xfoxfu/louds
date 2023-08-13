use fid_rs::Fid;
use newtype::NewType;

pub struct Louds {
    lbs: Fid,
}

impl Louds {
    pub fn from_str(s: &str) -> Self {
        let lbs = Fid::from(s);
        Self { lbs }
    }

    pub fn from_bit_vec(s: impl Into<Fid>) -> Self {
        Self { lbs: s.into() }
    }
}

#[derive(NewType, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct NodeId(pub u64);

#[derive(NewType, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BitIndex(pub u64);

impl Louds {
    /// Convert the position x into the node i. Dual to `bfs_select`.
    pub fn bfs_rank(&self, x: BitIndex) -> NodeId {
        self.lbs.rank(*x).into()
    }

    /// Convert the node i to position x. Dual to `bfs_rank`.
    /// If the node i is out of range, return `None`.
    pub fn bfs_select(&self, i: NodeId) -> Option<BitIndex> {
        self.lbs
            .select(*i)
            .filter(|i| *i < self.lbs.len())
            .map(BitIndex)
    }

    /// Get the parent node i of the position x.
    pub fn parent_rank(&self, x: BitIndex) -> NodeId {
        self.lbs.rank0(*x - 1).into()
    }

    /// Get the position of the first child of node i.
    pub fn first_child_select(&self, i: NodeId) -> Option<BitIndex> {
        self.lbs
            .select0(*i)
            .map(|x| x + 1)
            .filter(|i| *i < self.lbs.len())
            .map(BitIndex)
    }

    /// Check if the position is a leaf node.
    pub fn is_leaf(&self, x: BitIndex) -> bool {
        self.lbs[*self.bfs_rank(x)] == false
    }

    /// Get the parent node position of the position x.
    pub fn parent(&self, x: BitIndex) -> Option<BitIndex> {
        self.bfs_select(self.parent_rank(x))
    }

    /// Get the position of the first child of the position x.
    /// If position x is not a valid node, the behavior is undefined.
    pub fn first_child(&self, x: BitIndex) -> Option<BitIndex> {
        let y = self.first_child_select(self.bfs_rank(x));
        y.and_then(|y| if self.lbs[*y] == false { None } else { Some(y) })
    }

    /// Get the position of the last child of the position x.
    /// If position x is not a valid node, the behavior is undefined.
    pub fn last_child(&self, x: BitIndex) -> Option<BitIndex> {
        let y = self.lbs.select0(*self.bfs_rank(x) + 1).map(|y| y - 1);
        y.and_then(|y| if self.lbs[y] == false { None } else { Some(y) })
            .map(BitIndex)
    }

    /// Get the position of the sibling of the position x.
    /// If position x is not a valid node, the behavior is undefined.
    pub fn sibling(&self, x: BitIndex) -> Option<BitIndex> {
        Some(*x + 1).filter(|x| self.lbs[*x] != false).map(BitIndex)
    }

    /// Get the degree of the node of the position x.
    pub fn degree(&self, x: BitIndex) -> u64 {
        if self.is_leaf(x) {
            0
        } else {
            *self.last_child(x).unwrap() - *self.first_child(x).unwrap() + 1
        }
    }

    /// Get the i-th child of the node at position x. Dual to `child_rank`.
    pub fn child(&self, x: BitIndex, i: usize) -> Option<BitIndex> {
        if i as u64 >= self.degree(x) {
            return None;
        }
        self.first_child(x).map(|x| *x + i as u64).map(BitIndex)
    }

    /// Get the rank of the current node at position x. Dual to `child`.
    /// Returns `None` if x is not a node or does not have a parent.
    pub fn child_rank(&self, x: BitIndex) -> Option<usize> {
        self.parent(x)
            .and_then(|x| self.first_child(x))
            .map(|y| (*x - *y + 1) as usize)
    }
}
