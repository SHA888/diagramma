use crate::routing;
use crate::types::{LayoutEdge, LayoutNode, LayoutResult, Point};
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
/// # Examples
///
/// ```
/// use diagramma_core::{FlowchartSpec, Node, Edge, Direction, Theme, ColorRamp, NodeShape};
/// use diagramma_layout::flowchart;
///
/// let spec = FlowchartSpec {
///     direction: Direction::TopDown,
///     nodes: vec![
///         Node {
///             id: "start".into(),
///             label: "Start".into(),
///             subtitle: None,
///             color: ColorRamp::Blue,
///             shape: NodeShape::Rect,
///         },
///         Node {
///             id: "end".into(),
///             label: "End".into(),
///             subtitle: None,
///             color: ColorRamp::Green,
///             shape: NodeShape::Rect,
///         },
///     ],
///     edges: vec![Edge {
///         from: "start".into(),
///         to: "end".into(),
///         label: None,
///         style: diagramma_core::EdgeStyle::Solid,
///         arrow: diagramma_core::ArrowStyle::Closed,
///     }],
///     theme: Theme::Light,
/// };
///
/// let layout = flowchart::layout(&spec, 60.0, 40.0, 100.0, 60.0);
/// assert_eq!(layout.nodes.len(), 2);
/// assert_eq!(layout.edges.len(), 1);
/// ```
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
                nodes_in_row as f64 * node_width + (nodes_in_row - 1) as f64 * intra_layer_spacing
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
            rows_in_layer as f64 * node_height + (rows_in_layer - 1) as f64 * intra_layer_spacing
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
