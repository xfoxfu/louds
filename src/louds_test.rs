//! The test cases are from https://github.com/laysakura/louds-rs.
//! Licensed under MIT OR Apache-2.0.

macro_rules! parameterized_tests {
    ($fn: ident, $($name:ident: ($tree:expr, $arg:expr, $expect:expr),)*) => {
        #[cfg(test)]
        mod $fn {
            use crate::*;

            $(
                #[test]
                fn $name() {
                    let louds = dbg!(Louds::from_str($tree));
                    let index = louds.$fn($arg);
                    pretty_assertions::assert_eq!($expect, index);
                }
            )*
        }
    }
}

parameterized_tests!(bfs_select,
    t1_1: ("10_0", NodeId(1), Some(BitIndex(0))),
    t2_1: ("10_10_0", NodeId(1), Some(BitIndex(0))),
    t2_2: ("10_10_0", NodeId(2), Some(BitIndex(2))),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(1), Some(BitIndex(0))),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(2), Some(BitIndex(2))),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(3), Some(BitIndex(3))),
    t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(4), Some(BitIndex(4))),
    t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(5), Some(BitIndex(6))),
    t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(6), Some(BitIndex(9))),
    t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(7), Some(BitIndex(10))),
    t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(8), Some(BitIndex(11))),
    t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(9), Some(BitIndex(15))),
    t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(10), Some(BitIndex(17))),
    t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(11), Some(BitIndex(18))),
);

parameterized_tests!(bfs_rank,
    t1_1: ("10_0", BitIndex(0), NodeId(1)),
    t2_1: ("10_10_0", BitIndex(0), NodeId(1)),
    t2_2: ("10_10_0", BitIndex(2), NodeId(2)),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(0), NodeId(1)),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(2), NodeId(2)),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(3), NodeId(3)),
    t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(4), NodeId(4)),
    t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(6), NodeId(5)),
    t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(9), NodeId(6)),
    t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(10), NodeId(7)),
    t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(11), NodeId(8)),
    t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(15), NodeId(9)),
    t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(17), NodeId(10)),
    t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(18), NodeId(11)),
);

parameterized_tests!(parent_rank,
    t2_1: ("10_10_0", BitIndex(2), NodeId(1)),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(2), NodeId(1)),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(3), NodeId(1)),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(4), NodeId(1)),
    t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(6), NodeId(2)),
    t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(9), NodeId(4)),
    t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(10), NodeId(4)),
    t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(11), NodeId(4)),
    t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(15), NodeId(7)),
    t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(17), NodeId(8)),
    t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(18), NodeId(8)),
);

parameterized_tests!(first_child_select,
    t1_1: ("10_0", NodeId(1), Some(BitIndex(2))),
    t1_2: ("10_0", NodeId(2), None),
    t2_1: ("10_10_0", NodeId(1), Some(BitIndex(2))),
    t2_2: ("10_10_0", NodeId(2), Some(BitIndex(4))),
    t2_3: ("10_10_0", NodeId(3), None),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(1), Some(BitIndex(2))),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(2), Some(BitIndex(6))),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(3), Some(BitIndex(8))),
    t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(4), Some(BitIndex(9))),
    t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(5), Some(BitIndex(13))),
    t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(6), Some(BitIndex(14))),
    t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(7), Some(BitIndex(15))),
    t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(8), Some(BitIndex(17))),
    t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(9), Some(BitIndex(20))),
    t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(10), Some(BitIndex(21))),
    t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(11), Some(BitIndex(22))),
    t3_12: ("10_1110_10_0_1110_0_0_10_110_0_0_0", NodeId(12), None),
);

parameterized_tests!(first_child,
    t1_1: ("10_0", BitIndex(0), None),
    t2_0: ("10_10_0", BitIndex(0), Some(BitIndex(2))),
    t2_2: ("10_10_0", BitIndex(2), None),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(0), Some(BitIndex(2))),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(2), Some(BitIndex(6))),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(3), None),
    t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(4), Some(BitIndex(9))),
    t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(6), None),
    t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(9), None),
    t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(10), Some(BitIndex(15))),
    t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(11), Some(BitIndex(17))),
    t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(15), None),
    t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(17), None),
    t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(18), None),
);

parameterized_tests!(last_child,
    t1_1: ("10_0", BitIndex(0), None),
    t2_0: ("10_10_0", BitIndex(0), Some(BitIndex(2))),
    t2_2: ("10_10_0", BitIndex(2), None),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(0), Some(BitIndex(4))),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(2), Some(BitIndex(6))),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(3), None),
    t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(4), Some(BitIndex(11))),
    t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(6), None),
    t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(9), None),
    t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(10), Some(BitIndex(15))),
    t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(11), Some(BitIndex(18))),
    t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(15), None),
    t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(17), None),
    t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(18), None),
);

parameterized_tests!(sibling,
    t1_1: ("10_0", BitIndex(0), None),
    t2_0: ("10_10_0", BitIndex(0), None),
    t2_2: ("10_10_0", BitIndex(2), None),
    t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(0), None),
    t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(2), Some(BitIndex(3))),
    t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", BitIndex(3), Some(BitIndex(4))),
);
