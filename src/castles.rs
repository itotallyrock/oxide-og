
use std::convert::TryFrom;
use bitflags::bitflags;
use super::errors;

bitflags! {
    pub struct CastlePermissions: u8 {
        const NONE = 0;
        const WHITE_KING_CASTLE = 1;
        const WHITE_QUEEN_CASTLE = 2;
        const BLACK_KING_CASTLE = 4;
        const BLACK_QUEEN_CASTLE = 8;
        const WHITE_ALL = 3;
        const BLACK_ALL = 12;
        const ALL = 15;
    }
}

impl Default for CastlePermissions {
    fn default() -> Self {
        CastlePermissions::NONE
    }
}

impl TryFrom<String> for CastlePermissions {
    type Error = errors::InvalidCastlesError;
    fn try_from(castle_string: String) -> Result<Self, Self::Error> {
        match castle_string.as_str() {
            "KQkq" => Ok(CastlePermissions::ALL),

            "KQk" => Ok(CastlePermissions::WHITE_ALL | CastlePermissions::BLACK_KING_CASTLE),
            "KQq" => Ok(CastlePermissions::WHITE_ALL | CastlePermissions::BLACK_QUEEN_CASTLE),
            "Kkq" => Ok(CastlePermissions::WHITE_KING_CASTLE | CastlePermissions::BLACK_ALL),
            "Qkq" => Ok(CastlePermissions::WHITE_QUEEN_CASTLE | CastlePermissions::BLACK_ALL),

            "KQ" => Ok(CastlePermissions::WHITE_ALL),
            "kq" => Ok(CastlePermissions::BLACK_ALL),
            "Kk" => Ok(CastlePermissions::WHITE_KING_CASTLE | CastlePermissions::BLACK_KING_CASTLE),
            "Qq" => Ok(CastlePermissions::WHITE_QUEEN_CASTLE | CastlePermissions::BLACK_QUEEN_CASTLE),

            "K" => Ok(CastlePermissions::WHITE_KING_CASTLE),
            "Q" => Ok(CastlePermissions::WHITE_QUEEN_CASTLE),
            "k" => Ok(CastlePermissions::BLACK_KING_CASTLE),
            "q" => Ok(CastlePermissions::BLACK_QUEEN_CASTLE),

            "-" => Ok(CastlePermissions::NONE),
            _ => Err(errors::InvalidCastlesError),
        }
    }
}
