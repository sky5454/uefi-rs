//! Standard UEFI tables.

/// Common trait implemented by all standard UEFI tables.
pub trait Table {
    /// A unique number assigned by the UEFI specification
    /// to the standard tables.
    const SIGNATURE: u64;
}

mod header;
pub use header::Header;

mod system;
pub use system::{Boot, Runtime, SystemTable};

pub mod boot;
pub mod runtime;

pub mod cfg;

pub use uefi_raw::table::Revision;
