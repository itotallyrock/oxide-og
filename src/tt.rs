
// Local imports
use crate::chess_move::ChessMove;
#[cfg(feature = "low_memory")]
use crate::chess_move::CompressedChessMove;
use crate::eval::ScoreType;
// Std imports
#[cfg(not(feature = "low_memory"))]
use std::time::Duration;
use std::time::Instant;
use std::collections::HashMap;
use std::sync::RwLock;
use std::ops::Index;
// External imports
use lazy_static::*;
use log::{trace, warn};

// Max capacity of TT
#[cfg(not(feature = "low_memory"))]
const TT_SIZE: usize = 10000019;
#[cfg(feature = "low_memory")]
const TT_SIZE: usize = 100003;
// TT Entries can be overwritten after 30 seconds
#[cfg(not(feature = "low_memory"))]
const MAX_AGE: Duration = Duration::from_secs(30);


#[derive(Debug)]
pub(crate) struct TranspositionTable {
    len: usize,
    entries: Box<[Option<TranspositionEntry>; TT_SIZE]>,
}

impl TranspositionTable {
    #[inline]
    pub const fn capacity(&self) -> usize {
        TT_SIZE
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub const fn contains_key(&self, key: u64) -> bool {
        self.entries[key as usize % self.capacity()].is_some()
    }
    pub fn insert(&mut self, key: u64, entry: TranspositionEntry) {
        // Try to get an entry at the key given to see if we are overwriting
        let entry = if let Some(existing_entry) = self.get(key) {
            // No need to increment length since we have an entry at this slot already
            // Choose the highest depth to keep
            if existing_entry.depth > entry.depth {
                // Always use the higher depth unless it exceeds the max age
                #[cfg(not(feature = "low_memory"))]
                if existing_entry.created_at.elapsed() < MAX_AGE {
                    *existing_entry
                } else {
                    trace!("overwriting tt entry as it exceeds max age {:>5.3} / {:<5.3}", existing_entry.created_at.elapsed().as_secs_f64(), MAX_AGE.as_secs_f64());
                    entry
                }
                // Since we don't use age on low memory just use the higher depth without age
                #[cfg(feature = "low_memory")] {
                    *existing_entry
                }
            } else {
                trace!("overwriting tt entry depth {} with higher depth {}", existing_entry.depth, entry.depth);
                entry
            }
        } else {
            // Since we are adding a new entry increment length
            self.len += 1;
            entry
        };
        // Set the slot to the entry we chose
        self.entries[key as usize % self.capacity()] = Some(entry);
    }
    // Debug only because its slow and only used in dump
    #[cfg(debug_assertions)]
    pub fn entries(&self) -> impl Iterator<Item=&TranspositionEntry> {
        self.entries.iter().filter(|e| e.is_some()).map(|e| e.as_ref().unwrap())
    }
    #[inline]
    pub fn get(&self, key: u64) -> Option<&TranspositionEntry> {
        // This is safe as long as entries has the same length as capacity
        unsafe { self.entries.get_unchecked(key as usize % self.capacity()) }.as_ref()
    }
}

lazy_static! {
    pub(crate) static ref TRANSPOSITION_TABLE: RwLock<TranspositionTable> = RwLock::new(TranspositionTable {
        len: 0,
        entries: box [None; TT_SIZE],
    });
}

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
    pub fn key(&self) -> u64 {
        self.key
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
