
// Local imports
use crate::pieces::Piece;
use crate::square::Square;
#[cfg(not(feature = "low_memory"))]
use crate::attacks::sliding::init_sliding_attacks;

// Modules
mod sliding;
mod king;
mod knight;
mod pawn;

// Re-export attack getters
pub use sliding::bishop_attacks;
pub use sliding::rook_attacks;
pub use sliding::queen_attacks;
pub use king::king_attacks;
pub use knight::knight_attacks;
pub use pawn::pawn_attacks;

pub fn pseudo_attacks(piece: Piece, from_square: Square, occupied: u64) -> u64 {
    match piece {
        Piece::Pawn => panic!("pawn attacks unsupported in pseudo attacks"),
        Piece::Bishop => bishop_attacks(from_square, occupied),
        Piece::Rook => rook_attacks(from_square, occupied),
        Piece::King => panic!("king attacks unsupported in pseudo attacks"),
        Piece::Knight => knight_attacks(from_square.mask()),
        Piece::Queen => queen_attacks(from_square, occupied),
        Piece::None => panic!("cannot get pseudo attacks for none piece"),
    }
}

// Initialize attacks
pub fn init_attacks() {
    // Initialize sliding attack magic tables and pseudo attack tables
    #[cfg(not(feature = "low_memory"))]
    init_sliding_attacks();
}