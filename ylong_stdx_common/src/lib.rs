// Provide default trait implementation of SafeClone trait on structs that implements Clone trait
#![feature(specialization)]
// Mask the unstable alarm of specialization.
#![allow(incomplete_features)]
// Temporarily need this feature, because we use hashbrown from std.
#![feature(rustc_private)]

use core::alloc::LayoutError;
use std::alloc::Layout;
use std::collections::hash_map::TryReserveError;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Container Error for safe crates in the project of STD enhancement.
#[derive(Debug)]
pub enum ContainerError {
    AllocFailure(Layout),
    LayoutError,
    CapacityOverflow,
    Full,
    IndexInvalid,
    Custom(Box<dyn Error>),
    Unlikely,
}

impl Display for ContainerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ContainerError {}

// Temporarily need this mask, because we use hashbrown from std.
/// Convert from errors of RawTable in hashbrown crate. RawTable is the inner part of
/// FixedCapMap.
#[allow(exported_private_dependencies)]
impl From<TryReserveError> for ContainerError {
    fn from(value: TryReserveError) -> Self {
        match value {
            TryReserveError::CapacityOverflow => ContainerError::CapacityOverflow,
            TryReserveError::AllocError { layout } => ContainerError::AllocFailure(layout),
        }
    }
}

impl From<LayoutError> for ContainerError {
    fn from(_value: LayoutError) -> Self {
        ContainerError::LayoutError
    }
}

/// Clone trait safely returns recoverable error, compared to Clone trait in STD.
pub trait SafeClone: Sized {
    /// Safe clone self, and return Err if failing in allocation or other operations.
    fn safe_clone(&self) -> Result<Self, ContainerError>;

    /// Safe clone self from an other instance, and return Err if failing in allocation or other
    /// operations.
    fn safe_clone_from(&mut self, other: &Self) -> Result<(), ContainerError>;
}

/// Default implementation of SafeClone trait for sturcts that implement Clone. Users can
/// override this if they want.
///
/// # Attention
/// If users want to override these implementations, users need to use the unstable feature
/// `#[feature(specialization)]`. There'll be a warning because of the instability in rustc of the
/// community's version, if users want to mask this warning, please specify
/// `#[allow(incomplete_features)]`.
impl<T> SafeClone for T
where
    T: Clone,
{
    default fn safe_clone(&self) -> Result<Self, ContainerError> {
        Ok(self.clone())
    }

    default fn safe_clone_from(&mut self, other: &Self) -> Result<(), ContainerError> {
        self.clone_from(other);
        Ok(())
    }
}
