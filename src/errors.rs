
use err_derive::Error;
use super::square;


#[derive(Clone, Debug, Error)]
#[error(display = "invalid square string was expected to be 2 characters long")]
pub struct SquareLengthError(pub String);

#[derive(Clone, Debug, Error)]
#[error(display = "invalid file character expecting 1 of {:?}'", square::FILE_CHARS)]
pub struct SquareFileError;

#[derive(Clone, Debug, Error)]
#[error(display = "invalid rank character expecting 1 of {:?}'", square::RANK_CHARS)]
pub struct SquareRankError;

#[derive(Clone, Debug, Error)]
pub enum SquareParseError {
    #[error(display = "invalid square string '{:?}' was expected to be 2 characters long", _0)]
    SquareLengthError(String),

    #[error(display = "invalid file character '{:?} expecting 1 of {:?}'", _0, square::FILE_CHARS)]
    SquareFileError(char),

    #[error(display = "invalid rank character '{:?} expecting 1 of {:?}'", _0, square::RANK_CHARS)]
    SquareRankError(char),
}

#[derive(Clone, Debug, Error)]
#[error(display = "invalid side string (make sure lowercase)")]
pub struct InvalidSideError;

#[derive(Clone, Debug, Error)]
#[error(display = "invalid castle string (consisting of [K][Q][k][q]|-)")]
pub struct InvalidCastlesError;


#[derive(Clone, Debug, Error)]
#[error(display = "failed to parse fen string '{}'", _0)]
pub struct InvalidFenString(pub String);


#[derive(Clone, Debug, Error)]
pub enum FenParseError {
    #[error(display = "could not parse square '{}' in fen", _1)]
    SquareParseError(#[error(source)] SquareParseError, String),
    #[error(display = "could not parse fen chunks")]
    InvalidFenString(#[error(source)] InvalidFenString),
    #[error(display = "could not parse side from '{}'", _1)]
    InvalidSideError(#[error(source)] InvalidSideError, String),
    #[error(display = "could not parse castles from '{}'", _1)]
    InvalidCastlesError(#[error(source)] InvalidCastlesError, String),
}
