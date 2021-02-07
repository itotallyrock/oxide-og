use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicBool;

use crate::board::Board;
use crate::search::SearchParameters;

// Easily defined types for evaluation (just to identify what type you're working with)
pub type PieceCountType = u8;
pub type ScoreType = i32;
pub type WeightType = ScoreType;
#[cfg(not(feature = "low_memory"))]
pub type MaterialKeyType = i32;
pub type GamePhase = ScoreType;

// Search types
// Type for easy switching for various members
pub type NodeCount = u64;
pub type PlyCount = u8;
// Thread search types
pub type QuitFlag = Arc<AtomicBool>;
pub type GoCV = Arc<(Mutex<(bool, SearchParameters, Board)>, Condvar)>;

// Score type for moves
pub type MoveScore = ScoreType;

