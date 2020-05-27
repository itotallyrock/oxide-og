
use std::convert::TryFrom;
use bitflags::bitflags;
use super::errors;

bitflags! {
    pub struct CastlePermissions: u8 {
        const NONE = 0;
        const WHITE_KING = 1;
        const WHITE_QUEEN = 2;
        const WHITE_ALL = 3;
        const BLACK_KING = 4;
        const BOTH_KINGS = 5;
        const WHITE_QUEEN_BLACK_KING = 6;
        const WHITE_ALL_BLACK_KING = 7;
        const BLACK_QUEEN = 8;
        const WHITE_KING_BLACK_QUEEN = 9;
        const BOTH_QUEENS = 10;
        const WHITE_ALL_BLACK_QUEEN = 11;
        const BLACK_ALL = 12;
        const BLACK_ALL_WHITE_KING = 13;
        const BLACK_ALL_WHITE_QUEEN = 14;
        const ALL = 15;
    }
}

impl Default for CastlePermissions {
    fn default() -> Self {
        CastlePermissions::NONE
    }
}

impl ToString for CastlePermissions {
    fn to_string(&self) -> String {
        match self.clone() {
            CastlePermissions::WHITE_KING => "K".to_string(),
            CastlePermissions::WHITE_QUEEN => "Q".to_string(),
            CastlePermissions::WHITE_ALL => "KQ".to_string(),
            CastlePermissions::BLACK_KING => "k".to_string(),
            CastlePermissions::BOTH_KINGS => "Kk".to_string(),
            CastlePermissions::WHITE_QUEEN_BLACK_KING => "Qk".to_string(),
            CastlePermissions::WHITE_ALL_BLACK_KING => "KQk".to_string(),
            CastlePermissions::BLACK_QUEEN => "q".to_string(),
            CastlePermissions::WHITE_KING_BLACK_QUEEN => "Kq".to_string(),
            CastlePermissions::BOTH_QUEENS => "Qq".to_string(),
            CastlePermissions::WHITE_ALL_BLACK_QUEEN => "KQq".to_string(),
            CastlePermissions::BLACK_ALL => "kq".to_string(),
            CastlePermissions::BLACK_ALL_WHITE_KING => "Kkq".to_string(),
            CastlePermissions::BLACK_ALL_WHITE_QUEEN => "Qkq".to_string(),
            CastlePermissions::ALL => "KQkq".to_string(),
            CastlePermissions::NONE | _ => "-".to_string(),
        }
    }
}

impl TryFrom<String> for CastlePermissions {
    type Error = errors::InvalidCastlesError;
    fn try_from(castle_string: String) -> Result<Self, Self::Error> {
        match castle_string.as_str() {
            "KQkq" => Ok(CastlePermissions::ALL),

            "KQk" => Ok(CastlePermissions::WHITE_ALL_BLACK_KING),
            "KQq" => Ok(CastlePermissions::WHITE_ALL_BLACK_QUEEN),
            "Kkq" => Ok(CastlePermissions::BLACK_ALL_WHITE_KING),
            "Qkq" => Ok(CastlePermissions::BLACK_ALL_WHITE_QUEEN),

            "KQ" => Ok(CastlePermissions::WHITE_ALL),
            "kq" => Ok(CastlePermissions::BLACK_ALL),
            "Kk" => Ok(CastlePermissions::BOTH_KINGS),
            "Qq" => Ok(CastlePermissions::BOTH_QUEENS),
            "Qk" => Ok(CastlePermissions::WHITE_QUEEN_BLACK_KING),

            "K" => Ok(CastlePermissions::WHITE_KING),
            "Q" => Ok(CastlePermissions::WHITE_QUEEN),
            "k" => Ok(CastlePermissions::BLACK_KING),
            "q" => Ok(CastlePermissions::BLACK_QUEEN),

            "-" => Ok(CastlePermissions::NONE),
            _ => Err(errors::InvalidCastlesError),
        }
    }
}
