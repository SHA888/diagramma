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
    use crate::{LayoutEdge, LayoutNode, LayoutResult, Point, routing};
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

        let mut node_layouts: HashMap<NodeId, LayoutNode> = HashMap::new();
        for (node_id, (x, y)) in &positions {
            let layout_node = LayoutNode {
                id: node_id.clone(),
                x: *x,
                y: *y,
                width: node_width,
                height: node_height,
            };
            node_layouts.insert(node_id.clone(), layout_node.clone());
            result.nodes.insert(node_id.clone(), layout_node);
        }

        let mut sorted_edges = spec.edges.clone();
        sorted_edges.sort_by(|a, b| {
            let ax = positions.get(&a.from).map_or(0.0, |pos| pos.0);
            let bx = positions.get(&b.from).map_or(0.0, |pos| pos.0);
            ax.partial_cmp(&bx).unwrap_or(std::cmp::Ordering::Equal)
        });

        let all_nodes: Vec<LayoutNode> = node_layouts.values().cloned().collect();

        for edge in sorted_edges {
            if let (Some(from_node), Some(to_node)) =
                (node_layouts.get(&edge.from), node_layouts.get(&edge.to))
            {
                let path = routing::route_edge(from_node, to_node, spec.direction, &all_nodes);
                let arrow_pos = *path.last().unwrap_or(&Point::new(
                    to_node.x + to_node.width / 2.0,
                    to_node.y + to_node.height / 2.0,
                ));
                result.edges.push(LayoutEdge {
                    id: format!("{}-{}", edge.from, edge.to),
                    path,
                    arrow_pos,
                });
            }
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
    use crate::{LayoutNode, Point};
    use diagramma_core::Direction;

    const CLEARANCE: f64 = 16.0;
    const STEP: f64 = 24.0;

    /// Route an edge with obstacle avoidance and directional connection points.
    #[must_use]
    pub fn route_edge(
        from: &LayoutNode,
        to: &LayoutNode,
        direction: Direction,
        obstacles: &[LayoutNode],
    ) -> Vec<Point> {
        let (start, end) = connection_points(direction, from, to);
        let (start, end) = nudge_connection_points(direction, start, end, from, to, obstacles);

        if !segments_hit_obstacles(&[(start, end)], from, to, obstacles) {
            return vec![start, end];
        }

        // Try vertical-then-horizontal L bend.
        let mut bend_y = f64::midpoint(start.y, end.y);
        bend_y = adjust_horizontal_clearance(bend_y, start.x, end.x, from, to, obstacles);
        let mut candidate = vec![
            start,
            Point::new(start.x, bend_y),
            Point::new(end.x, bend_y),
            end,
        ];
        if !segments_hit_obstacles(
            &[
                (start, Point::new(start.x, bend_y)),
                (Point::new(start.x, bend_y), Point::new(end.x, bend_y)),
                (Point::new(end.x, bend_y), end),
            ],
            from,
            to,
            obstacles,
        ) {
            return candidate;
        }

        // Try horizontal-then-vertical L bend.
        let mut bend_x = f64::midpoint(start.x, end.x);
        bend_x = adjust_vertical_clearance(bend_x, start.y, end.y, from, to, obstacles);
        candidate = vec![
            start,
            Point::new(bend_x, start.y),
            Point::new(bend_x, end.y),
            end,
        ];
        if !segments_hit_obstacles(
            &[
                (start, Point::new(bend_x, start.y)),
                (Point::new(bend_x, start.y), Point::new(bend_x, end.y)),
                (Point::new(bend_x, end.y), end),
            ],
            from,
            to,
            obstacles,
        ) {
            return candidate;
        }

        // Fallback: dogleg around obstacles by offsetting vertically then horizontally.
        bend_y = find_clear_line(start.y, 1.0, from, to, obstacles, |y| {
            !horizontal_hits_obstacle(y, start.x, end.x, from, to, obstacles)
        });
        vec![
            start,
            Point::new(start.x, bend_y),
            Point::new(end.x, bend_y),
            end,
        ]
    }

    fn connection_points(
        direction: Direction,
        from: &LayoutNode,
        to: &LayoutNode,
    ) -> (Point, Point) {
        match direction {
            Direction::TopDown => (bottom_center(from), top_center(to)),
            Direction::BottomUp => (top_center(from), bottom_center(to)),
            Direction::LeftRight => (right_center(from), left_center(to)),
            Direction::RightLeft => (left_center(from), right_center(to)),
        }
    }

    fn bottom_center(node: &LayoutNode) -> Point {
        Point::new(node.x + node.width / 2.0, node.y + node.height + CLEARANCE)
    }
    fn top_center(node: &LayoutNode) -> Point {
        Point::new(node.x + node.width / 2.0, node.y - CLEARANCE)
    }
    fn left_center(node: &LayoutNode) -> Point {
        Point::new(node.x - CLEARANCE, node.y + node.height / 2.0)
    }
    fn right_center(node: &LayoutNode) -> Point {
        Point::new(node.x + node.width + CLEARANCE, node.y + node.height / 2.0)
    }

    fn adjust_horizontal_clearance(
        mut y: f64,
        x1: f64,
        x2: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> f64 {
        let mut attempts = 0;
        while horizontal_hits_obstacle(y, x1, x2, from, to, obstacles) && attempts < 12 {
            y += STEP;
            attempts += 1;
        }
        y
    }

    fn adjust_vertical_clearance(
        mut x: f64,
        y1: f64,
        y2: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> f64 {
        let mut attempts = 0;
        while vertical_hits_obstacle(x, y1, y2, from, to, obstacles) && attempts < 12 {
            x += STEP;
            attempts += 1;
        }
        x
    }

    fn find_clear_line<F>(
        mut value: f64,
        step_sign: f64,
        _from: &LayoutNode,
        _to: &LayoutNode,
        _obstacles: &[LayoutNode],
        predicate: F,
    ) -> f64
    where
        F: Fn(f64) -> bool,
    {
        let mut attempts = 0;
        while !predicate(value) && attempts < 20 {
            value += STEP * step_sign;
            attempts += 1;
        }
        value
    }

    fn nudge_connection_points(
        direction: Direction,
        mut start: Point,
        mut end: Point,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> (Point, Point) {
        match direction {
            Direction::LeftRight => {
                start.x = shift_clear_right(start.x, from, to, obstacles);
                end.x = shift_clear_left(end.x, from, to, obstacles);
            }
            Direction::RightLeft => {
                start.x = shift_clear_left(start.x, from, to, obstacles);
                end.x = shift_clear_right(end.x, from, to, obstacles);
            }
            Direction::TopDown => {
                start.y = shift_clear_down(start.y, from, to, obstacles);
                end.y = shift_clear_up(end.y, from, to, obstacles);
            }
            Direction::BottomUp => {
                start.y = shift_clear_up(start.y, from, to, obstacles);
                end.y = shift_clear_down(end.y, from, to, obstacles);
            }
        }

        (start, end)
    }

    fn shift_clear_right(
        x: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> f64 {
        let mut value = x;
        let mut attempts = 0;
        while vertical_hits_obstacle(
            value,
            from.y - CLEARANCE,
            from.y + from.height + CLEARANCE,
            from,
            to,
            obstacles,
        ) && attempts < 12
        {
            value += STEP;
            attempts += 1;
        }
        value
    }

    fn shift_clear_left(
        x: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> f64 {
        let mut value = x;
        let mut attempts = 0;
        while vertical_hits_obstacle(
            value,
            to.y - CLEARANCE,
            to.y + to.height + CLEARANCE,
            from,
            to,
            obstacles,
        ) && attempts < 12
        {
            value -= STEP;
            attempts += 1;
        }
        value
    }

    fn shift_clear_down(
        y: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> f64 {
        let mut value = y;
        let mut attempts = 0;
        while horizontal_hits_obstacle(
            value,
            from.x - CLEARANCE,
            from.x + from.width + CLEARANCE,
            from,
            to,
            obstacles,
        ) && attempts < 12
        {
            value += STEP;
            attempts += 1;
        }
        value
    }

    fn shift_clear_up(y: f64, from: &LayoutNode, to: &LayoutNode, obstacles: &[LayoutNode]) -> f64 {
        let mut value = y;
        let mut attempts = 0;
        while horizontal_hits_obstacle(
            value,
            to.x - CLEARANCE,
            to.x + to.width + CLEARANCE,
            from,
            to,
            obstacles,
        ) && attempts < 12
        {
            value -= STEP;
            attempts += 1;
        }
        value
    }

    fn segments_hit_obstacles(
        segments: &[(Point, Point)],
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> bool {
        for obstacle in obstacles {
            if obstacle.id == from.id || obstacle.id == to.id {
                continue;
            }
            for (start, end) in segments {
                if segment_intersects_obstacle(*start, *end, obstacle) {
                    return true;
                }
            }
        }
        false
    }

    fn horizontal_hits_obstacle(
        y: f64,
        x1: f64,
        x2: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> bool {
        segments_hit_obstacles(
            &[(Point::new(x1, y), Point::new(x2, y))],
            from,
            to,
            obstacles,
        )
    }

    fn vertical_hits_obstacle(
        x: f64,
        y1: f64,
        y2: f64,
        from: &LayoutNode,
        to: &LayoutNode,
        obstacles: &[LayoutNode],
    ) -> bool {
        segments_hit_obstacles(
            &[(Point::new(x, y1), Point::new(x, y2))],
            from,
            to,
            obstacles,
        )
    }

    fn segment_intersects_obstacle(start: Point, end: Point, obstacle: &LayoutNode) -> bool {
        if (start.x - end.x).abs() < f64::EPSILON {
            // Vertical segment
            let x = start.x;
            if x < obstacle.x - CLEARANCE || x > obstacle.x + obstacle.width + CLEARANCE {
                return false;
            }
            let (y1, y2) = if start.y < end.y {
                (start.y, end.y)
            } else {
                (end.y, start.y)
            };
            let obs_y1 = obstacle.y - CLEARANCE;
            let obs_y2 = obstacle.y + obstacle.height + CLEARANCE;
            return y2 > obs_y1 && y1 < obs_y2;
        } else if (start.y - end.y).abs() < f64::EPSILON {
            // Horizontal segment
            let y = start.y;
            if y < obstacle.y - CLEARANCE || y > obstacle.y + obstacle.height + CLEARANCE {
                return false;
            }
            let (x1, x2) = if start.x < end.x {
                (start.x, end.x)
            } else {
                (end.x, start.x)
            };
            let obs_x1 = obstacle.x - CLEARANCE;
            let obs_x2 = obstacle.x + obstacle.width + CLEARANCE;
            return x2 > obs_x1 && x1 < obs_x2;
        }
        false
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
    use super::text::{FontMetrics, box_size};
    use super::*;
    use super::{flowchart, routing, structural};
    use diagramma_core::{
        Container, Direction, Edge, Element, FlowchartSpec, Node, NodeShape, StructuralSpec, Theme,
    };
    use insta::assert_yaml_snapshot;
    use proptest::prelude::*;
    use serde::Serialize;

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

    #[test]
    fn test_routing_connection_points_follow_direction() {
        let from = LayoutNode {
            id: "from".into(),
            x: 100.0,
            y: 100.0,
            width: 80.0,
            height: 40.0,
        };
        let to = LayoutNode {
            id: "to".into(),
            x: 260.0,
            y: 100.0,
            width: 80.0,
            height: 40.0,
        };
        let path = routing::route_edge(
            &from,
            &to,
            Direction::LeftRight,
            &[from.clone(), to.clone()],
        );
        assert!(path.first().unwrap().x > from.x + from.width);
        assert!(path.last().unwrap().x < to.x);
        assert_eq!(path.first().unwrap().y, from.y + from.height / 2.0);
        assert_eq!(path.last().unwrap().y, to.y + to.height / 2.0);
    }

    #[test]
    fn test_routing_avoids_obstacles() {
        let from = LayoutNode {
            id: "from".into(),
            x: 100.0,
            y: 100.0,
            width: 80.0,
            height: 40.0,
        };
        let to = LayoutNode {
            id: "to".into(),
            x: 260.0,
            y: 100.0,
            width: 80.0,
            height: 40.0,
        };
        let obstacle = LayoutNode {
            id: "obstacle".into(),
            x: 190.0,
            y: 80.0,
            width: 60.0,
            height: 80.0,
        };
        let path = routing::route_edge(
            &from,
            &to,
            Direction::LeftRight,
            &[from.clone(), to.clone(), obstacle.clone()],
        );
        // Expect at least one bend due to obstacle.
        assert!(path.len() >= 3);
        // Ensure vertical segment steers clear of obstacle bounds.
        for window in path.windows(2) {
            let seg_start = window[0];
            let seg_end = window[1];
            assert!(
                !((seg_start.x - seg_end.x).abs() < f64::EPSILON
                    && seg_start.x > obstacle.x
                    && seg_start.x < obstacle.x + obstacle.width),
                "Vertical segment intersects obstacle"
            );
        }
    }

    #[test]
    fn snapshot_flowchart_basic() {
        let spec = example_flowchart_spec();
        let layout = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
        assert_yaml_snapshot!("flowchart_basic_layout", snapshot_view(&layout));
    }

    #[test]
    fn snapshot_structural_nested() {
        let spec = example_structural_spec();
        let layout = structural::layout(&spec, 24.0, 12.0);
        assert_yaml_snapshot!("structural_nested_layout", snapshot_view(&layout));
    }

    proptest! {
        #[test]
        fn prop_flowchart_nodes_do_not_overlap(count in 1usize..9) {
            let spec = line_flowchart_spec(count);
            let layout = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
            let nodes: Vec<_> = layout.nodes.values().collect();
            for i in 0..nodes.len() {
                for j in (i+1)..nodes.len() {
                    prop_assert!(!rects_overlap(nodes[i], nodes[j]));
                }
            }
        }

        #[test]
        fn prop_viewbox_contains_nodes(count in 1usize..9) {
            let spec = line_flowchart_spec(count);
            let layout = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
            let (vx, vy, vw, vh) = layout.viewbox;
            for node in layout.nodes.values() {
                prop_assert!(node.x >= vx);
                prop_assert!(node.y >= vy);
                prop_assert!(node.x + node.width <= vx + vw + f64::EPSILON);
                prop_assert!(node.y + node.height <= vy + vh + f64::EPSILON);
            }
        }
    }

    fn rects_overlap(a: &LayoutNode, b: &LayoutNode) -> bool {
        let a_right = a.x + a.width;
        let a_bottom = a.y + a.height;
        let b_right = b.x + b.width;
        let b_bottom = b.y + b.height;
        !(a_right <= b.x || b_right <= a.x || a_bottom <= b.y || b_bottom <= a.y)
    }

    fn line_flowchart_spec(count: usize) -> FlowchartSpec {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for idx in 0..count.max(1) {
            nodes.push(Node {
                id: format!("n{idx}").into(),
                label: format!("Node {idx}").into(),
                subtitle: None,
                color: diagramma_core::ColorRamp::Blue,
                shape: NodeShape::Rect,
            });
            if idx > 0 {
                edges.push(Edge {
                    from: format!("n{}", idx - 1).into(),
                    to: format!("n{idx}").into(),
                    label: None,
                    style: diagramma_core::EdgeStyle::Solid,
                    arrow: diagramma_core::ArrowStyle::Closed,
                });
            }
        }
        FlowchartSpec {
            direction: Direction::TopDown,
            nodes,
            edges,
            theme: Theme::Light,
        }
    }

    fn example_flowchart_spec() -> FlowchartSpec {
        FlowchartSpec {
            direction: Direction::TopDown,
            nodes: vec![
                Node {
                    id: "start".into(),
                    label: "Start".into(),
                    subtitle: Some("entry".into()),
                    color: diagramma_core::ColorRamp::Green,
                    shape: NodeShape::Rect,
                },
                Node {
                    id: "process".into(),
                    label: "Process".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Blue,
                    shape: NodeShape::Rect,
                },
                Node {
                    id: "decision".into(),
                    label: "Decide".into(),
                    subtitle: None,
                    color: diagramma_core::ColorRamp::Purple,
                    shape: NodeShape::Diamond,
                },
            ],
            edges: vec![
                Edge {
                    from: "start".into(),
                    to: "process".into(),
                    label: None,
                    style: diagramma_core::EdgeStyle::Solid,
                    arrow: diagramma_core::ArrowStyle::Closed,
                },
                Edge {
                    from: "process".into(),
                    to: "decision".into(),
                    label: None,
                    style: diagramma_core::EdgeStyle::Solid,
                    arrow: diagramma_core::ArrowStyle::Closed,
                },
            ],
            theme: Theme::Light,
        }
    }

    fn example_structural_spec() -> StructuralSpec {
        StructuralSpec {
            containers: vec![Container {
                id: "root".into(),
                label: "System".into(),
                color: diagramma_core::ColorRamp::Blue,
                children: vec![
                    Element::Node(Node {
                        id: "api".into(),
                        label: "API".into(),
                        subtitle: None,
                        color: diagramma_core::ColorRamp::Green,
                        shape: NodeShape::Rect,
                    }),
                    Element::Container(Container {
                        id: "svc".into(),
                        label: "Services".into(),
                        color: diagramma_core::ColorRamp::Purple,
                        children: vec![Element::Node(Node {
                            id: "svc-a".into(),
                            label: "Service A".into(),
                            subtitle: None,
                            color: diagramma_core::ColorRamp::Coral,
                            shape: NodeShape::Rect,
                        })],
                    }),
                ],
            }],
            edges: Vec::new(),
            theme: Theme::Light,
        }
    }

    #[derive(Serialize)]
    struct SnapshotLayout {
        nodes: Vec<SnapshotNode>,
        edges: Vec<SnapshotEdge>,
        containers: Vec<SnapshotContainer>,
        viewbox: (f64, f64, f64, f64),
    }

    #[derive(Serialize)]
    struct SnapshotNode {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    #[derive(Serialize)]
    struct SnapshotEdge {
        id: String,
        path: Vec<(f64, f64)>,
    }

    #[derive(Serialize)]
    struct SnapshotContainer {
        id: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    fn snapshot_view(result: &LayoutResult) -> SnapshotLayout {
        let mut nodes: Vec<_> = result
            .nodes
            .values()
            .map(|node| SnapshotNode {
                id: node.id.to_string(),
                x: node.x,
                y: node.y,
                width: node.width,
                height: node.height,
            })
            .collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));

        let mut edges: Vec<_> = result
            .edges
            .iter()
            .map(|edge| SnapshotEdge {
                id: edge.id.clone(),
                path: edge.path.iter().map(|p| (p.x, p.y)).collect(),
            })
            .collect();
        edges.sort_by(|a, b| a.id.cmp(&b.id));

        let mut containers: Vec<_> = result
            .containers
            .values()
            .map(|container| SnapshotContainer {
                id: container.id.to_string(),
                x: container.x,
                y: container.y,
                width: container.width,
                height: container.height,
            })
            .collect();
        containers.sort_by(|a, b| a.id.cmp(&b.id));

        SnapshotLayout {
            nodes,
            edges,
            containers,
            viewbox: result.viewbox,
        }
    }
}
