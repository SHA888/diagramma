//! SVG generation for diagramma.
//!
//! Takes layout results and produces themed, interactive SVG strings
//! with dark/light mode support and clickable elements.

pub mod elements;
pub mod tokens;

/// Library version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
