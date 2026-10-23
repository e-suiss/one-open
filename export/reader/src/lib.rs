//! Reader for the One export package (`app.suiss.one.export`, defined in `export/export.cddl`).
//!
//! The package is JSON. [`from_str`] and [`from_slice`] decode it into typed structs,
//! check every value against its CDDL rule, refuse a document with a missing required
//! field, and skip fields the reader does not know at every level, so a version 1 reader
//! reads later versions, which only add fields.
//!
//! ```
//! let text = std::fs::read_to_string(concat!(
//!     env!("CARGO_MANIFEST_DIR"),
//!     "/../vectors/round-trip.json"
//! ))
//! .map_err(|e| e.to_string())?;
//! let vectors: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
//! let document = vectors
//!     .get(0)
//!     .and_then(|v| v.get("input"))
//!     .and_then(|v| v.get("document"))
//!     .ok_or("vector")?;
//! let package = one_export_reader::from_str(&document.to_string()).map_err(|e| e.to_string())?;
//! assert_eq!(package.schema_version.get(), 1);
//! # Ok::<(), String>(())
//! ```

pub mod document;
pub mod record;
pub mod reference;
pub mod scalar;

use core::fmt;

pub use document::ExportDocument;

/// A document that is not a readable export package.
#[derive(Debug)]
pub struct ReadError(serde_json::Error);

impl ReadError {
    /// Line of the error in the input, from 1; 0 when unknown.
    #[must_use]
    pub fn line(&self) -> usize {
        self.0.line()
    }

    /// Column of the error in the input, from 1; 0 when unknown.
    #[must_use]
    pub fn column(&self) -> usize {
        self.0.column()
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not a valid export package: {}", self.0)
    }
}

impl core::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.0)
    }
}

/// Reads an export package from JSON text.
///
/// # Errors
/// [`ReadError`] when the text is not JSON, a required field is missing, or a value breaks its rule.
pub fn from_str(json: &str) -> Result<ExportDocument, ReadError> {
    serde_json::from_str(json).map_err(ReadError)
}

/// Reads an export package from UTF-8 JSON bytes.
///
/// # Errors
/// [`ReadError`] when the bytes are not JSON, a required field is missing, or a value breaks its rule.
pub fn from_slice(json: &[u8]) -> Result<ExportDocument, ReadError> {
    serde_json::from_slice(json).map_err(ReadError)
}

/// Reads an export package from a parsed JSON value.
///
/// # Errors
/// [`ReadError`] when a required field is missing or a value breaks its rule.
pub fn from_value(json: serde_json::Value) -> Result<ExportDocument, ReadError> {
    serde_json::from_value(json).map_err(ReadError)
}

/// Writes the package as JSON with only the fields this reader knows.
///
/// # Errors
/// [`ReadError`] if serialisation fails.
pub fn to_value(document: &ExportDocument) -> Result<serde_json::Value, ReadError> {
    serde_json::to_value(document).map_err(ReadError)
}
