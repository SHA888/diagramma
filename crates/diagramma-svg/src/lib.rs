//! SVG generation for diagramma.
//!
//! Takes layout results and produces themed, interactive SVG strings
//! with dark/light mode support and clickable elements.

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
