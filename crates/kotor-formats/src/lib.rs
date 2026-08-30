//! Round-trip readers and writers for BioWare Aurora/Odyssey data formats.
//!
//! Loading a file and saving it back without edits produces the same bytes,
//! which is what a patcher needs to be trustworthy: it must reproduce
//! everything it does not touch, exactly.

pub mod error;
pub mod fsutil;
pub mod latin1;
pub mod text;

pub mod erf;
pub mod gff;
pub mod ssf;
pub mod strtok;
pub mod tlk;
pub mod twoda;
