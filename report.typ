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

I conducted the implementation with Rust. First, I have found a useful bit vector library with FID called `fid`, so I uses it for my base.

= Evaluation
