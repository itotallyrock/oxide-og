
// Local imports
use crate::chess_move::ChessMove;
use crate::eval::ScoreType;

// Max capacity of TT
#[cfg(not(feature = "low_memory"))]
pub const TT_DEFAULT_SIZE: usize = 100_000_019;
#[cfg(feature = "low_memory")]
pub const TT_DEFAULT_SIZE: usize = 100003;


#[derive(Debug)]
pub struct TranspositionTable {
    len: usize,
    capacity: usize,
    entries: Vec<Option<TranspositionEntry>>,
    #[cfg(debug_assertions)]
    hits: usize,
}

impl TranspositionTable {
    #[inline]
    pub fn new(tt_size: usize) -> Self {
        Self {
            len: 0,
            capacity: tt_size,
            entries: vec![None; tt_size],
            #[cfg(debug_assertions)]
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
    pub fn insert(&mut self, key: u64, entry: TranspositionEntry) {
        // Try to get an entry at the key given to see if we are overwriting
        let entry = if let Some(existing_entry) = self.get(key) {
            // No need to increment length since we have an entry at this slot already
            // Choose the highest depth to keep
            if existing_entry.depth > entry.depth {
                // Always use the higher depth
                existing_entry
            } else {
                // trace!("overwriting tt entry depth {} with higher depth {}", existing_entry.depth, entry.depth);
                entry
            }
        } else {
            // Since we are adding a new entry increment length
            self.len += 1;
            entry
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
    #[cfg(not(debug_assertions))]
    pub fn get(&self, key: u64) -> Option<TranspositionEntry> {
        // This is safe as long as entries has the same length as capacity
        unsafe { self.entries.get_unchecked(key as usize % self.capacity()) }.filter(|tt| {
            tt.key == key
        })
    }
    #[inline]
    #[cfg(debug_assertions)]
    pub fn get(&mut self, key: u64) -> Option<TranspositionEntry> {
        // This is safe as long as entries has the same length (or less than) capacity
        let t = &self.entries[key as usize % self.capacity()];
        // unsafe { self.entries.get_unchecked(key as usize % self.capacity()) }.filter(|tt| tt.key == key).as_ref()
        t.filter(|tt| if tt.key == key {
            // Update hits since we are in debug
            self.hits += 1;
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
    key: u64,
    best_move: ChessMove,
    depth: u8,
    score: ScoreType,
    node_type: PVType,
}

// Getters
impl TranspositionEntry {
    pub const fn best_move(&self) -> ChessMove {
        self.best_move
    }
    pub const fn key(&self) -> u64 {
        self.key
    }
    pub const fn depth(&self) -> u8 {
        self.depth
    }
    pub const fn score(&self) -> i32 {
        self.score
    }
    pub const fn node_type(&self) -> PVType {
        self.node_type
    }
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
