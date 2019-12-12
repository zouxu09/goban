use crate::pieces::stones::Color;
use crate::pieces::util::coord::Point;
use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::ops::Index;

const SEED: u64 = 172_147_124;

#[derive(Debug, Clone)]
pub struct ZobristTable {
    hashes: Vec<Vec<u64>>,
    n: usize,
}

impl ZobristTable {
    fn new(n: usize) -> Self {
        let mut rng = XorShiftRng::seed_from_u64(SEED);
        let mut hashes = vec![vec![0; 2]; 19 * 19];
        for i in 0..n * n {
            for j in 0..2 {
                hashes[i][j] = rng.next_u64();
            }
        }
        ZobristTable { hashes, n }
    }
}

impl Index<(Point, Color)> for ZobristTable {
    type Output = u64;

    fn index(&self, (x, color): (Point, Color)) -> &Self::Output {
        &self.hashes[x.0 * self.n + x.1][(color as u8 - 1) as usize]
    }
}

lazy_static! {
    pub static ref ZOBRIST: ZobristTable = ZobristTable::new(19);
}
