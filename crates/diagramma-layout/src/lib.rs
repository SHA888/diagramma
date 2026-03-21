//! Auto-layout algorithms for diagramma.
//!
//! Provides hierarchical layout (flowcharts), tree packing (structural diagrams),
//! and arrow routing with obstacle avoidance.

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
