
// Local imports
use crate::chess_move::ChessMove;
use crate::eval::ScoreType;
use crate::board::Board;
use std::sync::RwLock;
use crate::search::SearchError;

// Max capacity of TT
#[cfg(not(any(feature = "low_memory", debug_assertions)))]
pub const TT_DEFAULT_SIZE: usize = 100_000_019;
#[cfg(all(debug_assertions, not(feature = "low_memory")))]
pub const TT_DEFAULT_SIZE: usize = 1_000_019;
#[cfg(feature = "low_memory")]
pub const TT_DEFAULT_SIZE: usize = 100_003;

#[inline]
pub fn probe_tt(tt: &RwLock<TranspositionTable>, board: &Board) -> Result<Option<TranspositionEntry>, SearchError> {
    probe_tt_by_key(tt, board.state().key())
}

#[inline]
pub fn probe_tt_by_key(tt: &RwLock<TranspositionTable>, key: u64) -> Result<Option<TranspositionEntry>, SearchError> {
    #[cfg(feature = "diagnostics")] {
        return Ok(tt.write().map_err(|_| SearchError::UnableToWriteTT)?.get(key));
    }
    #[cfg(not(feature = "diagnostics"))] {
        return Ok(tt.read().map_err(|_| SearchError::UnableToReadTT)?.get(key));
    }
}

#[inline]
pub fn insert_tt(tt: &RwLock<TranspositionTable>, new_entry: TranspositionEntry) -> Result<(), SearchError> {
    tt.write().map_err(|_| SearchError::UnableToWriteTT)?.insert(new_entry.key, new_entry);

    Ok(())
}


#[derive(Debug)]
pub struct TranspositionTable {
    len: usize,
    capacity: usize,
    entries: Vec<Option<TranspositionEntry>>,
    #[cfg(feature = "diagnostics")]
    hits: usize,
}

impl TranspositionTable {
    #[inline]
    pub fn new(tt_size: usize) -> Self {
        Self {
            len: 0,
            capacity: tt_size,
            entries: vec![None; tt_size],
            #[cfg(feature = "diagnostics")]
            hits: 0,
        }
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub const fn is_empty(&self) -> bool { self.len == 0 }
    pub fn insert(&mut self, key: u64, new_entry: TranspositionEntry) {
        // Try to get an entry at the key given to see if we are overwriting
        let entry = if let Some(existing_entry) = self.get(key) {
            // No need to increment length since we have an entry at this slot already
            // Choose which entry to keep, prefer PV, to prevent key collisions overwrite if keys differ, and keep the highest depth
            if new_entry.node_type == PVType::PV || new_entry.key != existing_entry.key || new_entry.depth > existing_entry.depth {
                new_entry
            } else {
                existing_entry
            }
        } else {
            // Since we are adding a new entry increment length
            self.len += 1;
            new_entry
        };
        // Set the slot to the entry we chose
        self.entries[key as usize % self.capacity] = Some(entry);
    }
    // Debug only because its slow and only used in dump
    #[cfg(debug_assertions)]
    pub fn entries(&self) -> impl Iterator<Item=&TranspositionEntry> {
        self.entries.iter().filter(|e| e.is_some()).map(|e| e.as_ref().unwrap())
    }
    #[inline]
    #[cfg(not(feature = "diagnostics"))]
    pub fn get(&self, key: u64) -> Option<TranspositionEntry> {
        // This is safe as long as entries has the same length as capacity
        debug_assert!(self.entries.capacity() >= self.capacity, "Underlying capacity is not high enough");
        unsafe { self.entries.get_unchecked(key as usize % self.capacity()) }.filter(|tt| {
            tt.key == key
        })
    }
    #[inline]
    #[cfg(feature = "diagnostics")]
    pub fn get(&mut self, key: u64) -> Option<TranspositionEntry> {
        // This is safe as long as entries has the same length (or less than) capacity
        let entry = self.entries[key as usize % self.capacity()];
        // unsafe { self.entries.get_unchecked(key as usize % self.capacity()) }.filter(|tt| tt.key == key).as_ref()
        entry.filter(|e| if e.key == key {
            // Update hits since we are in debug
            #[cfg(feature = "diagnostics")] {
                self.hits += 1;
            }
            true
        } else {
            false
        })
    }
    // Clear is only used with clear tt which is debug only
    #[cfg(debug_assertions)]
    pub fn clear(&mut self) -> usize {
        use log::info;

        let entry_count = self.len;
        self.entries.fill(None);
        self.len = 0;
        info!("cleared {} elements from tt", entry_count);

        entry_count
    }
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
    // TODO: Consider only storing truncated key as u16 (if key is uniformly distributed we can still match on lower 16bits mostly safely (at least on low_memory))
    key: u64,
    best_move: ChessMove,
    depth: u8,
    score: ScoreType,
    node_type: PVType,
}

// Getters
impl TranspositionEntry {
    #[inline]
    pub const fn best_move(&self) -> ChessMove {
        self.best_move
    }
    #[inline]
    pub const fn key(&self) -> u64 {
        self.key
    }
    #[inline]
    pub const fn depth(&self) -> u8 {
        self.depth
    }
    #[inline]
    pub const fn score(&self) -> i32 {
        self.score
    }
    #[inline]
    pub const fn node_type(&self) -> PVType {
        self.node_type
    }
    #[inline]
    pub const fn new(key: u64, best_move: ChessMove, depth: u8, score: i32, node_type: PVType) -> Self {
        Self {
            key,
            best_move,
            depth,
            score,
            node_type,
        }
    }
}
