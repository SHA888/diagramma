//! Auto-layout algorithms for diagramma.
//!
//! Provides hierarchical layout (flowcharts), tree packing (structural diagrams),
//! and arrow routing with obstacle avoidance.

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

/// Flowchart layout (hierarchical/layered algorithm).
pub mod flowchart {
    use crate::{LayoutEdge, LayoutNode, LayoutResult, Point};
    use diagramma_core::{Direction, FlowchartSpec, NodeId};
    use std::collections::{HashMap, VecDeque};

    /// Compute flowchart layout from a validated spec.
    ///
    /// # Arguments
    /// * `spec` - Validated flowchart specification
    /// * `inter_layer_spacing` - Vertical spacing between layers (default 60px)
    /// * `intra_layer_spacing` - Horizontal spacing within a layer (default 40px)
    /// * `node_width` - Fixed node width (default 100px)
    /// * `node_height` - Fixed node height (default 60px)
    ///
    /// # Returns
    /// Positioned layout with nodes, edges, and viewBox.
    ///
    /// # Panics
    /// Panics if a path vector is empty (should not occur with valid specs).
    #[must_use]
    pub fn layout(
        spec: &FlowchartSpec,
        inter_layer_spacing: f64,
        intra_layer_spacing: f64,
        node_width: f64,
        node_height: f64,
    ) -> LayoutResult {
        let mut result = LayoutResult::new();

        if spec.nodes.is_empty() {
            result.set_viewbox(40.0, 0.0, 640.0, 100.0);
            return result;
        }

        let mut layers = assign_layers(spec);
        reorder_layers(&mut layers, spec);
        let base_positions = assign_coordinates(
            &layers,
            inter_layer_spacing,
            intra_layer_spacing,
            node_width,
            node_height,
        );
        let (positions, viewbox_height, max_x) =
            transform_positions(&base_positions, node_width, node_height, spec.direction);

        for (node_id, (x, y)) in &positions {
            result.nodes.insert(
                node_id.clone(),
                LayoutNode {
                    id: node_id.clone(),
                    x: *x,
                    y: *y,
                    width: node_width,
                    height: node_height,
                },
            );
        }

        for edge in &spec.edges {
            let from_pos = positions.get(&edge.from).copied().unwrap_or((0.0, 0.0));
            let to_pos = positions.get(&edge.to).copied().unwrap_or((0.0, 0.0));

            let from_center = (
                from_pos.0 + node_width / 2.0,
                from_pos.1 + node_height / 2.0,
            );
            let to_center = (to_pos.0 + node_width / 2.0, to_pos.1 + node_height / 2.0);

            let path = vec![
                Point::new(from_center.0, from_center.1),
                Point::new(to_center.0, to_center.1),
            ];
            let arrow_pos = *path.last().unwrap();

            result.edges.push(LayoutEdge {
                id: format!("{}-{}", edge.from, edge.to),
                path,
                arrow_pos,
            });
        }

        let height = (viewbox_height + 40.0).max(100.0);
        let width = (max_x - 40.0).max(600.0);
        result.set_viewbox(40.0, 0.0, width, height);
        result
    }

    fn assign_layers(spec: &FlowchartSpec) -> Vec<Vec<NodeId>> {
        let mut layers: Vec<Vec<NodeId>> = vec![Vec::new()];
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();

        let start_nodes: Vec<_> = spec
            .nodes
            .iter()
            .filter(|n| !spec.edges.iter().any(|e| e.to == n.id))
            .map(|n| n.id.clone())
            .collect();

        for node_id in start_nodes {
            queue.push_back((node_id.clone(), 0));
            visited.insert(node_id);
        }

        while let Some((node_id, layer)) = queue.pop_front() {
            while layers.len() <= layer {
                layers.push(Vec::new());
            }
            layers[layer].push(node_id.clone());

            for edge in spec.edges.iter().filter(|e| e.from == node_id) {
                if !visited.contains(&edge.to) {
                    visited.insert(edge.to.clone());
                    queue.push_back((edge.to.clone(), layer + 1));
                }
            }
        }

        layers
    }

    #[allow(clippy::cast_precision_loss)]
    fn reorder_layers(layers: &mut [Vec<NodeId>], spec: &FlowchartSpec) {
        if layers.is_empty() {
            return;
        }

        let mut order_map: HashMap<NodeId, usize> = layers[0]
            .iter()
            .cloned()
            .enumerate()
            .map(|(idx, id)| (id, idx))
            .collect();

        for layer in layers.iter_mut().skip(1) {
            let mut entries: Vec<(NodeId, Option<f64>, usize)> = layer
                .iter()
                .cloned()
                .enumerate()
                .map(|(original_idx, node_id)| {
                    let predecessors: Vec<f64> = spec
                        .edges
                        .iter()
                        .filter(|edge| edge.to == node_id)
                        .filter_map(|edge| order_map.get(&edge.from).map(|&pos| pos as f64))
                        .collect();
                    let barycenter = if predecessors.is_empty() {
                        None
                    } else {
                        Some(predecessors.iter().sum::<f64>() / predecessors.len() as f64)
                    };
                    (node_id, barycenter, original_idx)
                })
                .collect();

            entries.sort_by(|a, b| match (a.1, b.1) {
                (Some(a_val), Some(b_val)) => a_val
                    .partial_cmp(&b_val)
                    .unwrap_or(std::cmp::Ordering::Equal),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.2.cmp(&b.2),
            });

            layer.clear();
            for (idx, (node_id, _, _)) in entries.into_iter().enumerate() {
                layer.push(node_id.clone());
                order_map.insert(node_id, idx);
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn assign_coordinates(
        layers: &[Vec<NodeId>],
        inter_layer_spacing: f64,
        intra_layer_spacing: f64,
        node_width: f64,
        node_height: f64,
    ) -> HashMap<NodeId, (f64, f64)> {
        let mut positions = HashMap::new();
        let safe_width = 640.0;
        let mut current_y = 40.0;
        for layer in layers {
            if layer.is_empty() {
                continue;
            }
            let chunks: Vec<&[NodeId]> = layer.chunks(4).collect();
            for (chunk_idx, chunk) in chunks.iter().enumerate() {
                let nodes_in_row = chunk.len();
                let row_width = if nodes_in_row > 1 {
                    nodes_in_row as f64 * node_width
                        + (nodes_in_row - 1) as f64 * intra_layer_spacing
                } else {
                    node_width
                };
                let start_x = ((safe_width - row_width) / 2.0).max(40.0);
                let row_y = current_y + chunk_idx as f64 * (node_height + intra_layer_spacing);
                for (node_idx, node_id) in chunk.iter().enumerate() {
                    let x = start_x + (node_idx as f64) * (node_width + intra_layer_spacing);
                    positions.insert((*node_id).clone(), (x, row_y));
                }
            }
            let rows_in_layer = chunks.len().max(1);
            let layer_height = if rows_in_layer > 1 {
                rows_in_layer as f64 * node_height
                    + (rows_in_layer - 1) as f64 * intra_layer_spacing
            } else {
                node_height
            };
            current_y += layer_height + inter_layer_spacing;
        }
        positions
    }

    fn transform_positions(
        positions: &HashMap<NodeId, (f64, f64)>,
        node_width: f64,
        node_height: f64,
        direction: Direction,
    ) -> (HashMap<NodeId, (f64, f64)>, f64, f64) {
        let mut transformed = HashMap::new();
        let (min_x, _max_x, min_y, max_y) = bounds(positions, node_width, node_height);

        match direction {
            Direction::TopDown => {
                transformed.extend(positions.iter().map(|(id, pos)| (id.clone(), *pos)));
            }
            Direction::BottomUp => {
                for (id, (x, y)) in positions {
                    let new_y = (min_y + max_y) - (*y + node_height);
                    transformed.insert(id.clone(), (*x, new_y));
                }
            }
            Direction::LeftRight => {
                for (id, (x, y)) in positions {
                    let new_x = min_x + (*y - min_y);
                    let new_y = min_y + (*x - min_x);
                    transformed.insert(id.clone(), (new_x, new_y));
                }
            }
            Direction::RightLeft => {
                for (id, (x, y)) in positions {
                    let new_x = min_x + (max_y - min_y) - (*y - min_y) - node_height;
                    let new_y = min_y + (*x - min_x);
                    transformed.insert(id.clone(), (new_x, new_y));
                }
            }
        }

        // Normalize positions to maintain top-left origin at (40, 0).
        let (t_min_x, _, t_min_y, _) = bounds(&transformed, node_width, node_height);
        let dx = 40.0 - t_min_x;
        let dy = 0.0 - t_min_y;
        if dx.abs() > f64::EPSILON || dy.abs() > f64::EPSILON {
            for value in transformed.values_mut() {
                value.0 += dx;
                value.1 += dy;
            }
        }

        let (_, t_max_x, t_min_y, t_max_y) = bounds(&transformed, node_width, node_height);

        (transformed, t_max_y - t_min_y, t_max_x)
    }

    fn bounds(
        positions: &HashMap<NodeId, (f64, f64)>,
        node_width: f64,
        node_height: f64,
    ) -> (f64, f64, f64, f64) {
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        for (x, y) in positions.values() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x + node_width);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y + node_height);
        }
        (min_x, max_x, min_y, max_y)
    }
}

/// Arrow routing utilities.
pub mod routing {
    use crate::Point;

    /// Route an edge with L-bend (horizontal-then-vertical).
    ///
    /// # Arguments
    /// * `from` - Starting point
    /// * `to` - Ending point
    ///
    /// # Returns
    /// Vector of points forming the path.
    #[must_use]
    pub fn l_bend_h_then_v(from: Point, to: Point) -> Vec<Point> {
        let mid_x = f64::midpoint(from.x, to.x);
        vec![from, Point::new(mid_x, from.y), Point::new(mid_x, to.y), to]
    }

    /// Route an edge with L-bend (vertical-then-horizontal).
    ///
    /// # Arguments
    /// * `from` - Starting point
    /// * `to` - Ending point
    ///
    /// # Returns
    /// Vector of points forming the path.
    #[must_use]
    pub fn l_bend_v_then_h(from: Point, to: Point) -> Vec<Point> {
        let mid_y = f64::midpoint(from.y, to.y);
        vec![from, Point::new(from.x, mid_y), Point::new(to.x, mid_y), to]
    }

    /// Route an edge with direct straight line.
    ///
    /// # Arguments
    /// * `from` - Starting point
    /// * `to` - Ending point
    ///
    /// # Returns
    /// Vector of points forming the path.
    #[must_use]
    pub fn direct(from: Point, to: Point) -> Vec<Point> {
        vec![from, to]
    }
}

/// Structural layout (tree packing for containers).
pub mod structural {
    use crate::{LayoutContainer, LayoutElement, LayoutNode, LayoutResult};
    use diagramma_core::{Container, Element, Node, StructuralSpec};

    const NODE_WIDTH: f64 = 140.0;
    const NODE_HEIGHT: f64 = 56.0;

    /// Compute structural layout from a validated spec.
    ///
    /// # Arguments
    /// * `spec` - Validated structural specification
    /// * `inner_padding` - Padding inside containers (default 24px)
    /// * `text_padding` - Padding from text to edge (default 12px)
    ///
    /// # Returns
    /// Positioned layout with containers and edges.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn layout(spec: &StructuralSpec, inner_padding: f64, text_padding: f64) -> LayoutResult {
        let mut result = LayoutResult::new();

        if spec.containers.is_empty() {
            result.set_viewbox(40.0, 0.0, 640.0, 100.0);
            return result;
        }

        let mut max_x: f64 = 640.0;
        let mut max_y: f64 = 40.0;
        let mut y_offset = 40.0;

        for container in &spec.containers {
            let (layout_container, mut nodes, container_max_x, container_bottom) =
                layout_container_tree(container, 40.0, y_offset, inner_padding, text_padding);

            max_x = max_x.max(container_max_x + 40.0);
            max_y = max_y.max(container_bottom + inner_padding);
            y_offset = container_bottom + inner_padding;

            for node in nodes.drain(..) {
                result.nodes.insert(node.id.clone(), node);
            }
            result
                .containers
                .insert(layout_container.id.clone(), layout_container);
        }

        result.set_viewbox(40.0, 0.0, max_x.max(640.0), (max_y + 40.0).max(100.0));
        result
    }

    fn layout_container_tree(
        container: &Container,
        origin_x: f64,
        origin_y: f64,
        inner_padding: f64,
        text_padding: f64,
    ) -> (LayoutContainer, Vec<LayoutNode>, f64, f64) {
        let mut nodes = Vec::new();
        let label_width = label_pixel_width(&container.label, text_padding);
        let header_height = 40.0;
        let mut current_y = origin_y + header_height + inner_padding;
        let mut max_width = origin_x + label_width;
        let mut children = Vec::new();

        for element in &container.children {
            match element {
                Element::Node(node) => {
                    let layout_node =
                        layout_structural_node(node, origin_x + inner_padding, current_y);
                    max_width = max_width.max(layout_node.x + layout_node.width + inner_padding);
                    current_y += layout_node.height + inner_padding;
                    nodes.push(layout_node.clone());
                    children.push(LayoutElement::Node(layout_node));
                }
                Element::Container(child_container) => {
                    let (layout_child, mut child_nodes, child_max_x, child_bottom) =
                        layout_container_tree(
                            child_container,
                            origin_x + inner_padding,
                            current_y,
                            inner_padding,
                            text_padding,
                        );
                    max_width = max_width.max(child_max_x + inner_padding);
                    current_y = child_bottom + inner_padding;
                    nodes.append(&mut child_nodes);
                    children.push(LayoutElement::Container(layout_child));
                }
            }
        }

        let content_height = if current_y > origin_y + header_height + inner_padding {
            current_y - origin_y
        } else {
            header_height + inner_padding * 2.0
        };
        let width = max_width - origin_x;
        let layout_container = LayoutContainer {
            id: container.id.clone(),
            x: origin_x,
            y: origin_y,
            width: width.max(label_width + inner_padding * 2.0),
            height: content_height,
            children,
        };

        let bottom = origin_y + layout_container.height;
        let max_x = layout_container.x + layout_container.width;
        (layout_container, nodes, max_x, bottom)
    }

    #[allow(clippy::cast_precision_loss)]
    fn label_pixel_width(label: &str, text_padding: f64) -> f64 {
        label.len() as f64 * 8.0 + 2.0 * text_padding
    }

    fn layout_structural_node(node: &Node, x: f64, y: f64) -> LayoutNode {
        LayoutNode {
            id: node.id.clone(),
            x,
            y,
            width: NODE_WIDTH,
            height: NODE_HEIGHT,
        }
    }
}

/// Text measurement and box sizing utilities.
pub mod text {
    /// Font metrics for text measurement.
    #[derive(Debug, Clone, Copy)]
    pub struct FontMetrics {
        /// Character width at 14px font size (pixels).
        pub char_width_14px: f64,
        /// Character width at 12px font size (pixels).
        pub char_width_12px: f64,
        /// Horizontal padding inside box (pixels).
        pub h_padding: f64,
        /// Vertical padding inside box (pixels).
        pub v_padding: f64,
    }

    impl FontMetrics {
        /// Default monospace metrics: ~8px per char at 14px, ~7px at 12px.
        #[must_use]
        pub fn default_monospace() -> Self {
            Self {
                char_width_14px: 8.0,
                char_width_12px: 7.0,
                h_padding: 12.0,
                v_padding: 8.0,
            }
        }

        /// Estimate width for a string at 14px font.
        #[must_use]
        #[allow(clippy::cast_precision_loss)]
        pub fn estimate_width_14px(&self, text: &str) -> f64 {
            text.len() as f64 * self.char_width_14px
        }

        /// Estimate width for a string at 12px font.
        #[must_use]
        #[allow(clippy::cast_precision_loss)]
        pub fn estimate_width_12px(&self, text: &str) -> f64 {
            text.len() as f64 * self.char_width_12px
        }
    }

    /// Calculate box dimensions from title and optional subtitle.
    ///
    /// # Arguments
    /// * `title` - Primary text (14px font)
    /// * `subtitle` - Optional secondary text (12px font)
    /// * `metrics` - Font metrics for measurement
    ///
    /// # Returns
    /// `(width, height)` in pixels.
    #[must_use]
    pub fn box_size(title: &str, subtitle: Option<&str>, metrics: &FontMetrics) -> (f64, f64) {
        let title_width = metrics.estimate_width_14px(title) + 2.0 * metrics.h_padding;
        let subtitle_width = subtitle.map_or(0.0, |s| {
            metrics.estimate_width_12px(s) + 2.0 * metrics.h_padding
        });

        let width = title_width.max(subtitle_width);
        let height = if subtitle.is_some() {
            2.0 * metrics.v_padding + 14.0 + 4.0 + 12.0
        } else {
            2.0 * metrics.v_padding + 14.0
        };

        (width, height)
    }
}

/// Library version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::flowchart;
    use super::text::{FontMetrics, box_size};
    use super::*;
    use diagramma_core::{Direction, Edge, FlowchartSpec, Node, Theme};

    #[test]
    fn test_layout_node_creation() {
        let node = LayoutNode {
            id: "node1".into(),
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 50.0,
        };
        assert_eq!(node.x, 10.0);
        assert_eq!(node.y, 20.0);
        assert_eq!(node.width, 100.0);
        assert_eq!(node.height, 50.0);
    }

    #[test]
    fn test_point_creation() {
        let p = Point::new(5.0, 15.0);
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 15.0);
    }

    #[test]
    fn test_layout_edge_creation() {
        let edge = LayoutEdge {
            id: "edge1".to_string(),
            path: vec![Point::new(0.0, 0.0), Point::new(100.0, 100.0)],
            arrow_pos: Point::new(100.0, 100.0),
        };
        assert_eq!(edge.path.len(), 2);
        assert_eq!(edge.arrow_pos.x, 100.0);
    }

    #[test]
    fn test_layout_result_default() {
        let result = LayoutResult::default();
        assert!(result.nodes.is_empty());
        assert!(result.edges.is_empty());
        assert!(result.containers.is_empty());
        assert_eq!(result.viewbox, (0.0, 0.0, 680.0, 0.0));
    }

    #[test]
    fn test_layout_result_set_viewbox() {
        let mut result = LayoutResult::new();
        result.set_viewbox(40.0, 0.0, 640.0, 500.0);
        assert_eq!(result.viewbox, (40.0, 0.0, 640.0, 500.0));
    }

    #[test]
    fn test_font_metrics_default() {
        let metrics = FontMetrics::default_monospace();
        assert_eq!(metrics.char_width_14px, 8.0);
        assert_eq!(metrics.char_width_12px, 7.0);
        assert_eq!(metrics.h_padding, 12.0);
        assert_eq!(metrics.v_padding, 8.0);
    }

    #[test]
    fn test_estimate_width_14px() {
        let metrics = FontMetrics::default_monospace();
        let width = metrics.estimate_width_14px("hello");
        assert_eq!(width, 5.0 * 8.0);
    }

    #[test]
    fn test_estimate_width_12px() {
        let metrics = FontMetrics::default_monospace();
        let width = metrics.estimate_width_12px("hello");
        assert_eq!(width, 5.0 * 7.0);
    }

    #[test]
    fn test_box_size_title_only() {
        let metrics = FontMetrics::default_monospace();
        let (width, height) = box_size("hello", None, &metrics);
        let expected_width = 5.0 * 8.0 + 2.0 * 12.0;
        let expected_height = 2.0 * 8.0 + 14.0;
        assert_eq!(width, expected_width);
        assert_eq!(height, expected_height);
    }

    #[test]
    fn test_box_size_with_subtitle() {
        let metrics = FontMetrics::default_monospace();
        let (width, height) = box_size("hello", Some("world"), &metrics);
        let title_width: f64 = 5.0 * 8.0 + 2.0 * 12.0;
        let subtitle_width: f64 = 5.0 * 7.0 + 2.0 * 12.0;
        let expected_width = title_width.max(subtitle_width);
        let expected_height = 2.0 * 8.0 + 14.0 + 4.0 + 12.0;
        assert_eq!(width, expected_width);
        assert_eq!(height, expected_height);
    }

    #[test]
    fn test_flowchart_layout_empty() {
        let spec = FlowchartSpec {
            direction: Direction::TopDown,
            nodes: vec![],
            edges: vec![],
            theme: Theme::Light,
        };
        let result = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        assert!(result.nodes.is_empty());
        assert!(result.edges.is_empty());
        assert_eq!(result.viewbox, (40.0, 0.0, 640.0, 100.0));
    }

    #[test]
    fn test_flowchart_layout_single_node() {
        let spec = FlowchartSpec {
            direction: Direction::TopDown,
            nodes: vec![Node {
                id: "n1".into(),
                label: "Start".into(),
                subtitle: None,
                color: diagramma_core::ColorRamp::Blue,
                shape: diagramma_core::NodeShape::Rect,
            }],
            edges: vec![],
            theme: Theme::Light,
        };
        let result = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        assert_eq!(result.nodes.len(), 1);
        assert!(result.nodes.contains_key(&"n1".into()));
        assert!(result.edges.is_empty());
    }

    #[test]
    fn test_flowchart_layout_two_nodes_connected() {
        let spec = FlowchartSpec {
            direction: Direction::TopDown,
            nodes: vec![
                Node {
                    id: "n1".into(),
                    label: "Start".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Blue,
                    shape: diagramma_core::NodeShape::Rect,
                },
                Node {
                    id: "n2".into(),
                    label: "End".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Green,
                    shape: diagramma_core::NodeShape::Rect,
                },
            ],
            edges: vec![Edge {
                from: "n1".into(),
                to: "n2".into(),
                label: None,
                style: diagramma_core::EdgeStyle::Solid,
                arrow: diagramma_core::ArrowStyle::Closed,
            }],
            theme: Theme::Light,
        };
        let result = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        assert_eq!(result.nodes.len(), 2);
        assert!(!result.edges.is_empty());
        let edge = &result.edges[0];
        assert_eq!(edge.id, "n1-n2");
        assert!(edge.path.len() >= 2);
    }

    #[test]
    fn test_flowchart_layout_respects_barycenter_ordering() {
        let spec = FlowchartSpec {
            direction: Direction::TopDown,
            nodes: vec![
                Node {
                    id: "n1".into(),
                    label: "Start".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Blue,
                    shape: diagramma_core::NodeShape::Rect,
                },
                Node {
                    id: "n2".into(),
                    label: "Alt".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Green,
                    shape: diagramma_core::NodeShape::Rect,
                },
                Node {
                    id: "n3".into(),
                    label: "Left".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Purple,
                    shape: diagramma_core::NodeShape::Rect,
                },
                Node {
                    id: "n4".into(),
                    label: "Right".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Coral,
                    shape: diagramma_core::NodeShape::Rect,
                },
            ],
            edges: vec![
                Edge {
                    from: "n1".into(),
                    to: "n4".into(),
                    label: None,
                    style: diagramma_core::EdgeStyle::Solid,
                    arrow: diagramma_core::ArrowStyle::Closed,
                },
                Edge {
                    from: "n2".into(),
                    to: "n3".into(),
                    label: None,
                    style: diagramma_core::EdgeStyle::Solid,
                    arrow: diagramma_core::ArrowStyle::Closed,
                },
            ],
            theme: Theme::Light,
        };

        let result = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        let left = result.nodes.get(&"n4".into()).unwrap().x;
        let right = result.nodes.get(&"n3".into()).unwrap().x;
        assert!(left < right);
    }

    #[test]
    fn test_flowchart_layout_left_right_direction() {
        let spec = FlowchartSpec {
            direction: Direction::LeftRight,
            nodes: vec![
                Node {
                    id: "n1".into(),
                    label: "Start".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Blue,
                    shape: diagramma_core::NodeShape::Rect,
                },
                Node {
                    id: "n2".into(),
                    label: "Next".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Green,
                    shape: diagramma_core::NodeShape::Rect,
                },
            ],
            edges: vec![Edge {
                from: "n1".into(),
                to: "n2".into(),
                label: None,
                style: diagramma_core::EdgeStyle::Solid,
                arrow: diagramma_core::ArrowStyle::Closed,
            }],
            theme: Theme::Light,
        };

        let result = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        let start = result.nodes.get(&"n1".into()).unwrap();
        let next = result.nodes.get(&"n2".into()).unwrap();
        assert!(next.x > start.x);
        assert!((start.y - next.y).abs() < f64::EPSILON);
    }

    #[test]
    fn test_flowchart_layout_tier_cap_wraps_rows() {
        let mut nodes = Vec::new();
        for idx in 0..6 {
            nodes.push(Node {
                id: format!("n{idx}").into(),
                label: format!("Node {idx}").into(),
                subtitle: None,
                color: diagramma_core::ColorRamp::Blue,
                shape: diagramma_core::NodeShape::Rect,
            });
        }
        let edges: Vec<Edge> = nodes
            .iter()
            .skip(1)
            .map(|node| Edge {
                from: "n0".into(),
                to: node.id.clone(),
                label: None,
                style: diagramma_core::EdgeStyle::Solid,
                arrow: diagramma_core::ArrowStyle::Closed,
            })
            .collect();

        let spec = FlowchartSpec {
            direction: Direction::TopDown,
            nodes,
            edges,
            theme: Theme::Light,
        };

        let result = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        let first_row_y = result.nodes.get(&"n1".into()).unwrap().y;
        let second_row_y = result.nodes.get(&"n5".into()).unwrap().y;
        assert!(second_row_y > first_row_y);
        assert!(second_row_y - first_row_y < 2.0 * (60.0 + 40.0));
    }
}
