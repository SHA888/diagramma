#![allow(clippy::float_cmp)]

use crate::text::{FontMetrics, box_size};
use crate::*;
use crate::{flowchart, routing, structural};
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
