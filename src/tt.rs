
// Local imports
use crate::chess_move::ChessMove;
#[cfg(feature = "low_memory")]
use crate::chess_move::CompressedChessMove;
use crate::eval::ScoreType;
// Std imports
#[cfg(not(feature = "low_memory"))]
use std::time::Duration;
use std::time::Instant;
use std::mem::size_of;
use std::collections::HashMap;
use std::sync::RwLock;
// External imports
use lazy_static::*;
use log::{trace, warn};

const TT_ENTRY_BYTES: usize = size_of::<TranspositionEntry>();
// Or we can just in the command
#[cfg(feature = "low_memory")]
// 256 KiB
const HASHMAP_BASE_SIZE: usize = (256 * 1024) / TT_ENTRY_BYTES;
#[cfg(not(feature = "low_memory"))]
// 512 MiB
const HASHMAP_BASE_SIZE: usize = (512 * 1024 * 1024) / TT_ENTRY_BYTES;

lazy_static! {
    // NOTE: We may want a fixed size hashmap that never allocates
    pub static ref TRANSPOSITION_TABLE: RwLock<HashMap<u64, TranspositionEntry>> = RwLock::new(HashMap::with_capacity(HASHMAP_BASE_SIZE));
}

pub fn clear_tt() {
    let start = Instant::now();
    let mut tt = TRANSPOSITION_TABLE.write().unwrap();
    let len = tt.len();
    tt.clear();
    trace!(
        "cleared {} element(s) from tt in {}ms",
        len,
        start.elapsed().as_millis()
    );
}

pub fn get_tt_entry(key: u64) -> Option<TranspositionEntry> {
    TRANSPOSITION_TABLE.read().unwrap().get(&key).copied()
}

pub fn insert_tt_entry(key: u64, entry: TranspositionEntry) -> Option<TranspositionEntry> {
    if let Some(old_entry) = TRANSPOSITION_TABLE.write().unwrap().insert(key, entry) {
        // Warning on hash table insertion overwrite
        warn!("hash table collision at {:#X} old entry {:?} replaced by {:?}", key, old_entry, entry);

        Some(entry)
    } else {
        None
    }
}

pub fn tt_contains_key(key: u64) -> bool {
    TRANSPOSITION_TABLE.read().unwrap().contains_key(&key)
}

// TODO: Add functions to cleanup/recycle old entries

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PVType {
    PV,
    Upper,
    Lower,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
// NOTE: Packed to save memory, but might slow down access operations or lead to undefined behavior https://github.com/rust-lang/rust/issues/27060
#[cfg_attr(feature = "low_memory", repr(packed))]
pub struct TranspositionEntry {
    key: u64,
    #[cfg(not(feature = "low_memory"))]
    best_move: ChessMove,
    #[cfg(feature = "low_memory")]
    compressed_best_move: CompressedChessMove,
    // Plies from zero
    depth: u8,
    score: ScoreType,
    node_type: PVType,
    // Age (not included to save memory on low memory builds)
    #[cfg(not(feature = "low_memory"))]
    created_at: Instant,
}

// Getters
impl TranspositionEntry {
    #[cfg(not(feature = "low_memory"))]
    pub fn best_move(&self) -> ChessMove {
        self.best_move
    }
    #[cfg(feature = "low_memory")]
    pub fn best_move(&self) -> ChessMove {
        ChessMove::decompress(self.compressed_best_move)
    }
    pub fn depth(&self) -> u8 {
        self.depth
    }
    pub fn score(&self) -> i32 {
        self.score
    }
    pub fn node_type(&self) -> PVType {
        self.node_type
    }
    pub fn new(key: u64, best_move: ChessMove, depth: u8, score: i32, node_type: PVType) -> Self {
        Self {
            key,
            #[cfg(not(feature = "low_memory"))]
            best_move,
            #[cfg(feature = "low_memory")]
            compressed_best_move: best_move.compress(),
            depth,
            score,
            node_type,
            #[cfg(not(feature = "low_memory"))]
            created_at: Instant::now(),
        }
    }
}
