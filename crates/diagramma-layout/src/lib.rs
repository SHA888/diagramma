//! Auto-layout algorithms for diagramma.
//!
//! Provides hierarchical layout (flowcharts), tree packing (structural diagrams),
//! and arrow routing with obstacle avoidance.

pub mod flowchart;
pub mod routing;
pub mod structural;
pub mod text;
pub mod types;

pub use text::{FontMetrics, box_size};
pub use types::{LayoutContainer, LayoutEdge, LayoutElement, LayoutNode, LayoutResult, Point};

/// Library version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests;
