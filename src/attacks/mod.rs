
mod sliding;
mod king;
mod knight;
mod pawn;

#[cfg(not(feature = "low_memory"))]
use crate::attacks::sliding::init_sliding_attacks;

// Re-export attack getters
pub use sliding::bishop_attacks;
pub use sliding::rook_attacks;
pub use sliding::queen_attacks;
pub use king::king_attacks;
pub use knight::knight_attacks;
pub use pawn::pawn_attacks;

// Initialize attacks
pub fn init_attacks() {
    // Initialize sliding attack magic tables and pseudo attack tables
    #[cfg(not(feature = "low_memory"))]
    init_sliding_attacks();
}