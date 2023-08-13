#import "template.typ": *
#show: ieee.with(
  title: "Implementation of LOUDS: Level-Ordered Unary Degree Sequence",
  authors: (
    (
      name: "Yuze Fu",
      department: [University of Tokyo],
      email: "fu-yuze@g.ecc.u-tokyo.ac.jp",
    ),
  ),
  bibliography-file: "report.bib",
)

= LOUDS

LOUDS (Level-Ordered Unary Degree Sequence) is a data structure for efficient (constant time complexity) tree operations. It uses a succinct index for binary vectors which has a constant time complexity for _rank_ and _select_ operations.

== Bit Vectors

Bit vectors could be represented efficiently in memory with one bit for each element in the vector, like in @fig:bv. It takes 12 bits for representing a bit vector with 12 elements.

#figure(caption: "bit vector in memory")[#image("bit-vector.png", height: 4em)]<fig:bv>

When operating on bit vectors, _rank_ and _select_ are the two most frequent operations. _rank_ operation computes the number of `1`s or `0`s from the beginning of the vector to the position specified. For example, assuming the beginning position is `0`, in the bit vector in @fig:rank, the `rank(5)` returns $3$, which is the count of $1$ from $V_0$ to $V_5$.

#figure(caption: "rank and select operations")[#image("rank.png", height: 4em)]<fig:rank>

In contrast, _select_ operation returns the $i$-th `1` or `0` from the beginning of the vector. In @fig:rank, `select(3)` returns $5$, which is the position of the third $1$. Therefore, we could see rank and select operations as dual, which means it always has

$ "rank"("select"(i)) = "rank"("select"(i)) = i $

== FID Index for Bit Vectors

A kind of succinct index for bit vectors is called FID (Fully Indexable Dictionary). This kind of index provides constant time complexity for rank and select operations. In #cite("DBLP:journals/talg/RamanRS07"), they proposed a possible data structure for FID.

// TODO:

== LOUDS

With the power of FID indices, LOUDS #cite("DBLP:conf/ic-nc/KitamuraIMS15") is proposed. The tree structure is first represented by a series of bits, where each $1$ means a tree node, and each $0$ means the separation between tree nodes. For example, for the tree in @fig:tree, it is represented as the bit sequence on the bottom of the figure.

#figure(caption: "tree")[#image("tree.png", height: 10em)]<fig:tree>

The nodes are encoded in a bread-first order. The direct children (green) of the root node (red) is encoded at the beginning of the bit vector. As there are two children, it is two $1$ and an ending $0$ as `110`. Then, for node 1, its children are encoded is `110`, and for node 2, its children are encoded directly follows as `10`. Then, for node 3, 4 and 5, they do not have children, so the three nodes are encoded as `000`.

This data structure supports efficient querys between the identifier of a node and the index of the location of corresponding $1$ in the bit vector. These two operations are called $x = "bfs_select"(i)$ and $i = "bfs_rank"(x)$ respectively. Here, $i$ stands for the index of the bit vector, and $x$ stands for the identifier of the node. Therefore, $"bfs_select"(6) = 5$, and $"bfs_rank"(4) = 4$.

We know that each $1$ represents for a node, and the nodes are encoded in a bread-first order, so the $x$-th $1$ is the node $x$. Therefore, there is

$ "bfs_rank"(x) = "rank"_1(L, x) $

where $L$ is the bit vector. Similarily, there is

$ "bfs_select"(i) = "select"_1(L, i) $

as rank and select are dual just as bfs\_select and bfs\_rank. Since rank and select are $O(1)$ with FID, bfs\_select and bfs\_rank are also $O(1)$.

The next pair of operations are getting the position $i$ of a position $i_0$ ($"first_child_select"(i_0)$), and getting the identifier $x$ of the parent of a node $x_0$ ($"parent_rank"$). We know that each $0$ stands for the end of a children sequence, so the $i$-th $0$ is the end of children of the node $i-1$ (recall node $0$ is the phantom root, and physical node identifiers begin from $1$). Also, since the nodes are encoded bread-first, so the next position in $L$ is the first children of node $x$. Therefore, there is

$ "first_child_select"(i_0) = "select"_0(L, i_0) + 1 $

In addition, to get the parent position of a child identifier, we just need to count for $0$ to the left of the current position to get the parent identifier, so there is

$ "parent_rank"(x_0) = "rank"_0(L, x_0 - 1) $

Since rank and select are $O(1)$ with FID, first_child_select and parent_rank are also $O(1)$.

Additionally, one could get the starting and end position of child of a node $x$. The two operations are called $"firstchild"$ and $"lastchild"$. firstchild is straightforward as it only need to convert $x$ to position $i_0$ and use first_child_select to get the position $i$.

$ "firstchild"(i_0) = "first_child_select"("bfs_rank"(i_0)) $

If the result returns a position of value $0$, it means the node does not have any child, so the function should return $-1$ for _not found_.

similarly, lastchild could be implemented as getting the first child position of the next node and seek backward to get the resulting position. This is

$ "lastchild"(i_0) = "select"_0("bfs_rank"(i_0) + 1) - 1 $

Finally, there are some trivial operations with $O(1)$ to implement:

- $"is_leaf"(x) -> bold("bool")$ can be implemented as checking the first child position is $0$ or not.
- $"parent"(x) -> i$ can be implemented as getting the parent_rank and then apply bfs_select to get the position.
- $"sibling"(i) -> i$ can be implemented as increasing the position once if the current position is not $0$.
- $"degree"(i) -> bold("int")$ can be implemented as count the elements between firstchild and lastchild.
- $"child"(x, k) -> i$ for getting the $k$-th child of $x$ could be implemented as increasing the firstchild position by $k - 1$.
- $"childrank"(x) -> bold("int")$ for getting the rank for a child of $x$ could be implemented as substracting the position by the firstchild position of the parent and then add $1$.

There are some operations with greater time complexity:

- $"depth"(x) -> bold("int")$ could be implemented as resursively adding $1$ until reaching the root node.
- $"lca"(x, y) -> x'$ could be implemented as resursively choosing the lca of the node with smaller index in ${x, y}$ and the parent of the node with larger index.

= Implementation

I conducted the implementation with Rust. First, I have found a useful bit vector library with FID called `fid`, so I uses it for my base. It supports the $"rank"_0$, $"rank"_1$, $"select"_1$ and $"select"_1$ operations, and it could be constructed by inserting bit-by-bit.

== Construction

I defines a `TreeNode` for a user-friendly specification of tree nodes.

```rs
pub struct TreeNode {
    pub children: Vec<TreeNode>,
}
```

The child of each node is stored in the field `children`. For example, the tree in @fig:tree could be coded as

```rs
let root = TreeNode { // 0
  children: vec![
    TreeNode { // 1
      children: vec![
        TreeNode { children: vec![] }, // 3
        TreeNode { children: vec![] }, // 4
      ]
    },
    TreeNode { // 2
      children: vec![
        TreeNode { children: vec![] }, // 5
      ]
    },
  ]
};
```

Then, I wrote code to convert the tree into a bit vector. It is implemented just as the definition of LOUDS. The struct `Louds` represents the LOUDS data structure, and only contains the underlying bit vector with FID.

```rs
impl Into<Louds> for TreeNode {
    fn into(self) -> Louds {
        Louds::from_bit_vec(&*Into::<Vec<bool>>::into(self))
    }
}

impl Into<Vec<bool>> for TreeNode {
  fn into(self) -> Vec<bool> {
    let mut bits = Vec::new();
    // encode current node
    bits.resize(self.children.len(), true);
    bits.push(false);
    // encode children
    bits.extend(self.children.into_iter().flat_map(|child| Into::<Vec<bool>>::into(child)));
    bits
  }
}

#[derive(Debug)]
pub struct Louds {
    lbs: BitVector,
}

impl Louds {
  pub fn from_bit_vec(s: &[bool]) -> Self {
    let mut lbs = BitVector::new();
    for c in s.iter() {
      lbs.push(*c);
    }
    Self { lbs }
  }
}
```

== Utilities

As the `fid` library's index definition is different from the one described before, I wrote wrapper over the `fid` library to make the index consistent.

```rs
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
```

Since both the bit vector index and the node identifier is typed as `u64`, I created type wrappers with _New Type_ pattern to avoid confusion in the code. In this way, one cannot accidentally assign node identifier to bit vector index.

```rs
#[derive(NewType, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct NodeId(pub u64);

#[derive(NewType, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct BitIndex(pub u64);
```

== Operations

I have implemented the operations just as the LOUDS data structure defines.

```rs
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
  pub fn child_rank(&self, x: BitIndex) -> Option<usize> {
    self.parent(x)
      .and_then(|x| self.first_child(x))
      .map(|y| (*x - *y + 1) as usize)
  }

  /// Get the depth of a node.
  pub fn depth(&self, x: NodeId) -> usize {
    if *x == 1 {
      return 0;
    }
    1 + self.depth(
      self.parent_rank(self.bfs_select(x).unwrap()))
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
```

= Evaluation

I have used the test cases from #link("https://github.com/laysakura/louds-rs") for checking the correctness of the implementation. However, `louds-rs` has different operations naming and some operations are not implemented, so I borrowed the tree defined and manually created some test cases. The test cases could be run with the Rust cargo's builtin testing as `cargo test`, and has passed shown in @fig:test.

#figure(caption: "test cases")[#image("test.png")]<fig:test>

The source code for my implementation is available at #link("https://github.com/xfoxfu/louds").
