use bitvec::vec::BitVec;

struct Fid {
    data: BitVec,
    rank_index: (),
    select_index: (),
}

impl Fid {
    pub fn new(data: BitVec) -> Self {
        Self {
            data,
            rank_index: (),
            select_index: (),
        }
    }
}
