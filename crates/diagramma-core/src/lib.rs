//! Core type definitions for diagramma.
//!
//! Defines nodes, edges, containers, and diagram spec types
//! used across the diagramma rendering pipeline.

/// Library version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }
}
