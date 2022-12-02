//! A Compact ENDF (ACE) format module.
//!
//! # References
//!
//! Conlin, J. L., & Romano, P. (2019). *A compact ENDF (ACE) format specification*
//! (No. LA-UR-19-29016). Los Alamos National Lab.(LANL), Los Alamos, NM (United States).

mod error;
pub use error::AceError;

mod table;
pub use table::Table;

mod parse;
pub use parse::parse_ace_table;
