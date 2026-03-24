//! Core type definitions for diagramma.
//!
//! Defines nodes, edges, containers, diagram specs, and validation helpers
//! used across the rendering pipeline.

use schemars::{JsonSchema, schema::RootSchema, schema_for};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;
use std::collections::HashSet;
use std::fmt;
use std::ops::Deref;
use thiserror::Error;

/// Library version.
#[must_use]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn validate_controls(controls: &[Control]) -> ValidationResult<()> {
    let mut ids = HashSet::new();
    for control in controls {
        let control_id = match control {
            Control::Toggle { id, .. } | Control::Slider { id, .. } => id,
        };
        track_id(control_id, &mut ids)?;
    }
    Ok(())
}

/// Node identifier type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct NodeId(#[schemars(with = "String")] SmolStr);

impl NodeId {
    /// Create a new identifier from any string-like input.
    pub fn new(value: impl Into<SmolStr>) -> Self {
        Self(value.into())
    }
}

impl Deref for NodeId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for NodeId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// Color ramps available to nodes/containers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[non_exhaustive]
pub enum ColorRamp {
    Purple,
    Teal,
    Coral,
    Pink,
    Gray,
    Blue,
    Green,
    Amber,
    Red,
}

/// Theme selection for specs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

/// Primary direction for layout engines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Direction {
    TopDown,
    LeftRight,
    BottomUp,
    RightLeft,
}

/// Node shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum NodeShape {
    Rect,
    Pill,
    Diamond,
    Circle,
}

/// Edge style variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum EdgeStyle {
    Solid,
    Dashed,
}

impl Default for EdgeStyle {
    fn default() -> Self {
        Self::Solid
    }
}

/// Arrow styles for edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ArrowStyle {
    Open,
    Closed,
    None,
}

/// Diagram element types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Element {
    Node(Node),
    Container(Container),
}

/// Node definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Node {
    pub id: NodeId,
    #[schemars(with = "String")]
    pub label: SmolStr,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Option<String>")]
    pub subtitle: Option<SmolStr>,
    pub color: ColorRamp,
    pub shape: NodeShape,
}

/// Edge definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(with = "Option<String>")]
    pub label: Option<SmolStr>,
    #[serde(default)]
    pub style: EdgeStyle,
    #[serde(default = "default_arrow_style")]
    pub arrow: ArrowStyle,
}

const fn default_arrow_style() -> ArrowStyle {
    ArrowStyle::Closed
}

/// Container definition (recursive).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Container {
    pub id: NodeId,
    #[schemars(with = "String")]
    pub label: SmolStr,
    pub color: ColorRamp,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Element>,
}

/// Flowchart diagram specification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FlowchartSpec {
    pub direction: Direction,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub theme: Theme,
}

/// Structural specification (container tree).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StructuralSpec {
    pub containers: Vec<Container>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<Edge>,
    pub theme: Theme,
}

/// Illustrative specification (freeform shapes, annotations).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IllustrativeSpec {
    pub elements: Vec<Element>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[schemars(with = "Vec<String>")]
    pub annotations: Vec<SmolStr>,
    pub theme: Theme,
}

/// Interactive specification (base diagram + controls).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct InteractiveSpec {
    pub base: Box<DiagramSpec>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controls: Vec<Control>,
}

/// Diagram specification umbrella enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DiagramSpec {
    Flowchart(FlowchartSpec),
    Structural(StructuralSpec),
    Illustrative(IllustrativeSpec),
    Interactive(InteractiveSpec),
}

/// Interactive control definitions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "control", rename_all = "snake_case")]
pub enum Control {
    Toggle {
        id: NodeId,
        #[schemars(with = "String")]
        label: SmolStr,
    },
    Slider {
        id: NodeId,
        #[schemars(with = "String")]
        label: SmolStr,
        min: i32,
        max: i32,
        #[serde(default)]
        step: Option<i32>,
    },
}

/// Validation error variants.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("duplicate id: {0}")]
    DuplicateId(NodeId),
    #[error("missing element referenced by edge: {0}")]
    MissingReference(NodeId),
    #[error("container nesting exceeds depth limit: {limit}")]
    ContainerDepth { limit: usize },
    #[error("empty diagram")]
    EmptyDiagram,
    #[error("duplicate control id: {0}")]
    DuplicateControlId(NodeId),
    #[error("invalid edge: {0}")]
    InvalidEdge(String),
}

/// Result alias for validation.
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Produce a JSON schema for `DiagramSpec`.
#[must_use]
pub fn diagram_spec_schema() -> RootSchema {
    schema_for!(DiagramSpec)
}

/// Validate a diagram specification for structural integrity.
///
/// Ensures IDs are unique, edges reference existing elements, container depth is capped,
/// and diagram-specific invariants (non-empty illustrative specs, control IDs) hold.
///
/// # Errors
///
/// Returns [`ValidationError`] when any of the constraints fail (duplicate IDs, missing
/// references, excessive depth, empty diagrams, or invalid controls).
pub fn validate_spec(spec: &DiagramSpec) -> ValidationResult<()> {
    match spec {
        DiagramSpec::Flowchart(flow) => validate_nodes_edges(&flow.nodes, &flow.edges),
        DiagramSpec::Structural(structural) => {
            let mut ids = HashSet::new();
            collect_containers(&structural.containers, 0, &mut ids)?;
            validate_edges_exist(&structural.edges, &ids)
        }
        DiagramSpec::Illustrative(illustrative) => {
            if illustrative.elements.is_empty() {
                return Err(ValidationError::EmptyDiagram);
            }
            let mut ids = HashSet::new();
            collect_elements(&illustrative.elements, 0, &mut ids)
        }
        DiagramSpec::Interactive(interactive) => {
            validate_spec(interactive.base.as_ref())?;
            validate_controls(&interactive.controls)?;
            Ok(())
        }
    }
}

fn validate_nodes_edges(nodes: &[Node], edges: &[Edge]) -> ValidationResult<()> {
    let mut seen = HashSet::new();
    for node in nodes {
        track_id(&node.id, &mut seen)?;
    }
    validate_edges_exist(edges, &seen)
}

fn validate_edges_exist(edges: &[Edge], known: &HashSet<NodeId>) -> ValidationResult<()> {
    for edge in edges {
        if edge.from == edge.to {
            return Err(ValidationError::InvalidEdge(format!(
                "Self-referencing edge: {}",
                edge.from
            )));
        }
        if !known.contains(&edge.from) {
            return Err(ValidationError::MissingReference(edge.from.clone()));
        }
        if !known.contains(&edge.to) {
            return Err(ValidationError::MissingReference(edge.to.clone()));
        }
    }
    Ok(())
}

fn collect_containers(
    containers: &[Container],
    depth: usize,
    ids: &mut HashSet<NodeId>,
) -> ValidationResult<()> {
    const MAX_DEPTH: usize = 6;
    if depth >= MAX_DEPTH {
        return Err(ValidationError::ContainerDepth { limit: MAX_DEPTH });
    }
    for container in containers {
        track_id(&container.id, ids)?;
        collect_elements(&container.children, depth + 1, ids)?;
    }
    Ok(())
}

fn collect_elements(
    elements: &[Element],
    depth: usize,
    ids: &mut HashSet<NodeId>,
) -> ValidationResult<()> {
    for element in elements {
        match element {
            Element::Node(node) => track_id(&node.id, ids)?,
            Element::Container(container) => {
                collect_containers(std::slice::from_ref(container), depth, ids)?;
            }
        }
    }
    Ok(())
}

fn track_id(id: &NodeId, seen: &mut HashSet<NodeId>) -> ValidationResult<()> {
    if !seen.insert(id.clone()) {
        return Err(ValidationError::DuplicateId(id.clone()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }

    #[test]
    fn node_roundtrip() {
        let node = Node {
            id: NodeId::new("n1"),
            label: SmolStr::new("Node"),
            subtitle: None,
            color: ColorRamp::Blue,
            shape: NodeShape::Rect,
        };
        let json = serde_json::to_string(&node).unwrap();
        let back: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(node, back);
    }

    #[test]
    fn flowchart_validation_handles_missing_nodes() {
        let nodes = vec![Node {
            id: NodeId::new("a"),
            label: SmolStr::new("A"),
            subtitle: None,
            color: ColorRamp::Purple,
            shape: NodeShape::Rect,
        }];
        let edges = vec![Edge {
            from: NodeId::new("a"),
            to: NodeId::new("b"),
            label: None,
            style: EdgeStyle::Solid,
            arrow: ArrowStyle::Closed,
        }];
        let spec = DiagramSpec::Flowchart(FlowchartSpec {
            direction: Direction::TopDown,
            nodes,
            edges,
            theme: Theme::Light,
        });
        let err = validate_spec(&spec).unwrap_err();
        assert!(matches!(err, ValidationError::MissingReference(id) if id == NodeId::new("b")));
    }

    #[test]
    fn structural_depth_limit_triggered() {
        fn make_container(levels: usize) -> Container {
            Container {
                id: NodeId::new(format!("lvl{levels}")),
                label: SmolStr::new(format!("lvl{levels}")),
                color: ColorRamp::Green,
                children: if levels == 0 {
                    Vec::new()
                } else {
                    vec![Element::Container(make_container(levels - 1))]
                },
            }
        }

        let deep = make_container(8);
        let spec = DiagramSpec::Structural(StructuralSpec {
            containers: vec![deep],
            edges: vec![],
            theme: Theme::Dark,
        });
        assert!(matches!(
            validate_spec(&spec).unwrap_err(),
            ValidationError::ContainerDepth { .. }
        ));
    }

    #[test]
    fn schema_generation_produces_definitions() {
        let schema = diagram_spec_schema();
        assert!(schema.definitions.contains_key("Direction"));
    }

    #[test]
    fn test_validate_self_referencing_edge() {
        let nodes = vec![Node {
            id: NodeId::new("a"),
            label: SmolStr::new("A"),
            subtitle: None,
            color: ColorRamp::Blue,
            shape: NodeShape::Rect,
        }];
        let edges = vec![Edge {
            from: NodeId::new("a"),
            to: NodeId::new("a"),
            label: None,
            style: EdgeStyle::Solid,
            arrow: ArrowStyle::Closed,
        }];
        let spec = DiagramSpec::Flowchart(FlowchartSpec {
            direction: Direction::TopDown,
            nodes,
            edges,
            theme: Theme::Light,
        });
        let err = validate_spec(&spec).unwrap_err();
        assert!(
            matches!(err, ValidationError::InvalidEdge(msg) if msg.contains("Self-referencing edge"))
        );
    }

    proptest! {
        #[test]
        fn duplicate_nodes_fail(ids in proptest::collection::vec("[a-z]{1,4}", 2..5)) {
            let nodes: Vec<Node> = ids.iter().map(|id| Node {
                id: NodeId::new(id),
                label: SmolStr::new("node"),
                subtitle: None,
                color: ColorRamp::Purple,
                shape: NodeShape::Rect,
            }).collect();
            let spec = DiagramSpec::Flowchart(FlowchartSpec {
                direction: Direction::LeftRight,
                nodes,
                edges: vec![],
                theme: Theme::Auto,
            });
            let _ = validate_spec(&spec).err();
        }
    }
}
