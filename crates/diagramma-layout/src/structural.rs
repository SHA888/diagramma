use crate::types::{LayoutContainer, LayoutElement, LayoutNode, LayoutResult};
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
                let layout_node = layout_structural_node(node, origin_x + inner_padding, current_y);
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
