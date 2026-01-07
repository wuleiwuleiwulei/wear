use core::fmt;
use core::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum BorrowError {
    AlreadyBorrow,
    AlreadyBorrowMut,
}

impl Display for BorrowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BorrowError::AlreadyBorrow => f.pad("already borrowed"),
            BorrowError::AlreadyBorrowMut => f.pad("already mutably borrowed"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BorrowError {}
