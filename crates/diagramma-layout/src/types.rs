use diagramma_core::NodeId;
use std::collections::HashMap;

/// A positioned node in the layout result.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    /// Node identifier.
    pub id: NodeId,
    /// X coordinate (top-left).
    pub x: f64,
    /// Y coordinate (top-left).
    pub y: f64,
    /// Width of the node.
    pub width: f64,
    /// Height of the node.
    pub height: f64,
}

/// A point in 2D space (used for arrow paths).
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

impl Point {
    /// Create a new point.
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A positioned edge with routing path.
#[derive(Debug, Clone)]
pub struct LayoutEdge {
    /// Edge identifier (from-to pair).
    pub id: String,
    /// Path points for the edge (routing).
    pub path: Vec<Point>,
    /// Arrow head position (typically the last point).
    pub arrow_pos: Point,
}

/// A positioned container with recursive layout.
#[derive(Debug, Clone)]
pub struct LayoutContainer {
    /// Container identifier.
    pub id: NodeId,
    /// X coordinate (top-left).
    pub x: f64,
    /// Y coordinate (top-left).
    pub y: f64,
    /// Width of the container.
    pub width: f64,
    /// Height of the container.
    pub height: f64,
    /// Positioned children (nodes and containers).
    pub children: Vec<LayoutElement>,
}

/// A positioned element (node or container).
#[derive(Debug, Clone)]
pub enum LayoutElement {
    /// A positioned node.
    Node(LayoutNode),
    /// A positioned container.
    Container(LayoutContainer),
}

/// Complete layout result for a diagram.
#[derive(Debug, Clone)]
pub struct LayoutResult {
    /// All positioned nodes (flat).
    pub nodes: HashMap<NodeId, LayoutNode>,
    /// All positioned edges.
    pub edges: Vec<LayoutEdge>,
    /// All positioned containers (flat).
    pub containers: HashMap<NodeId, LayoutContainer>,
    /// `ViewBox` dimensions: (x, y, width, height).
    pub viewbox: (f64, f64, f64, f64),
}

impl LayoutResult {
    /// Create a new empty layout result.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            containers: HashMap::new(),
            viewbox: (0.0, 0.0, 680.0, 0.0),
        }
    }

    /// Set the viewBox dimensions.
    pub fn set_viewbox(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.viewbox = (x, y, width, height);
    }
}

impl Default for LayoutResult {
    fn default() -> Self {
        Self::new()
    }
}
