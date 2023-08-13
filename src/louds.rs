use fid::{BitVector, FID};
use newtype::NewType;

#[derive(Debug)]
pub struct Louds {
    lbs: BitVector,
}

impl Louds {
    pub fn from_str(s: &str) -> Self {
        let mut lbs = BitVector::new();
        for c in s.chars() {
            if c == '_' {
                continue;
            }
            lbs.push(c == '1');
        }
        Self { lbs }
    }

    pub fn from_bit_vec(s: &[bool]) -> Self {
        let mut lbs = BitVector::new();
        for c in s.iter() {
            lbs.push(*c);
        }
        Self { lbs }
    }
}

#[derive(NewType, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct NodeId(pub u64);

#[derive(NewType, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BitIndex(pub u64);

impl Louds {
    fn select0(&self, i: u64) -> u64 {
        self.lbs.select0(i - 1)
    }
    fn select1(&self, i: u64) -> u64 {
        self.lbs.select1(i - 1)
    }
    fn rank0(&self, i: u64) -> u64 {
        self.lbs.rank0(i + 1)
    }
    fn rank1(&self, i: u64) -> u64 {
        self.lbs.rank1(i + 1)
    }
}

impl Louds {
    /// Convert the position x into the node i. Dual to `bfs_select`.
    pub fn bfs_rank(&self, x: BitIndex) -> NodeId {
        self.rank1(*x).into()
    }

    /// Convert the node i to position x. Dual to `bfs_rank`.
    /// If the node i is out of range, return `None`.
    pub fn bfs_select(&self, i: NodeId) -> Option<BitIndex> {
        Some(self.select1(*i))
            .filter(|i| *i < self.lbs.len())
            .map(BitIndex)
    }

    /// Get the parent node i of the position x.
    pub fn parent_rank(&self, x: BitIndex) -> NodeId {
        self.rank0(*x - 1).into()
    }

    /// Get the position of the first child of node i.
    pub fn first_child_select(&self, i: NodeId) -> Option<BitIndex> {
        Some(self.select0(*i))
            .map(|x| x + 1)
            .filter(|i| *i < self.lbs.len())
            .map(BitIndex)
    }

    /// Check if the position is a leaf node.
    pub fn is_leaf(&self, x: BitIndex) -> bool {
        self.lbs.get(*self.bfs_rank(x)) == false
    }

    /// Get the parent node position of the position x.
    pub fn parent(&self, x: BitIndex) -> Option<BitIndex> {
        self.bfs_select(self.parent_rank(x))
    }

    /// Get the position of the first child of the position x.
    /// If position x is not a valid node, the behavior is undefined.
    pub fn first_child(&self, x: BitIndex) -> Option<BitIndex> {
        let y = self.first_child_select(self.bfs_rank(x));
        y.and_then(|y| {
            if self.lbs.get(*y) == false {
                None
            } else {
                Some(y)
            }
        })
    }

    /// Get the position of the last child of the position x.
    /// If position x is not a valid node, the behavior is undefined.
    pub fn last_child(&self, x: BitIndex) -> Option<BitIndex> {
        let y = Some(self.select0(*self.bfs_rank(x) + 1)).map(|y| y - 1);
        y.and_then(|y| {
            if self.lbs.get(y) == false {
                None
            } else {
                Some(y)
            }
        })
        .map(BitIndex)
    }

    /// Get the position of the sibling of the position x.
    /// If position x is not a valid node, the behavior is undefined.
    pub fn sibling(&self, x: BitIndex) -> Option<BitIndex> {
        Some(*x + 1)
            .filter(|x| self.lbs.get(*x) != false)
            .map(BitIndex)
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
    pub fn child_rank(&self, i: BitIndex) -> Option<usize> {
        self.parent(i)
            .and_then(|x| self.first_child(x))
            .map(|y| (*i - *y + 1) as usize)
    }

    /// Get the depth of a node.
    pub fn depth(&self, x: NodeId) -> usize {
        if *x == 1 {
            return 0;
        }
        1 + self.depth(self.parent_rank(self.bfs_select(x).unwrap()))
    }

    /// Get the lease common ancestor of two nodes.
    pub fn lca(&self, x: NodeId, y: NodeId) -> NodeId {
        if x == y {
            return x;
        }
        self.lca(
            std::cmp::min(x, y),
            self.parent_rank(self.bfs_select(std::cmp::max(x, y)).unwrap()),
        )
    }
}
