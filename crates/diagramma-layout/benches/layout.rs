use criterion::{Criterion, criterion_group, criterion_main};
use diagramma_core::{Edge, FlowchartSpec, Node, NodeShape, Theme};
use diagramma_layout::flowchart;

fn bench_flowchart_layout(c: &mut Criterion) {
    let spec_small = sample_spec(10);
    let spec_medium = sample_spec(50);
    let spec_large = sample_spec(200);

    c.bench_function("flowchart_layout_10", |b| {
        b.iter(|| flowchart::layout(&spec_small, 60.0, 40.0, 100.0, 60.0));
    });
    c.bench_function("flowchart_layout_50", |b| {
        b.iter(|| flowchart::layout(&spec_medium, 60.0, 40.0, 100.0, 60.0));
    });
    c.bench_function("flowchart_layout_200", |b| {
        b.iter(|| flowchart::layout(&spec_large, 60.0, 40.0, 100.0, 60.0));
    });
}

fn sample_spec(count: usize) -> FlowchartSpec {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for idx in 0..count {
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
        direction: diagramma_core::Direction::TopDown,
        nodes,
        edges,
        theme: Theme::Light,
    }
}

criterion_group!(layout_benches, bench_flowchart_layout);
criterion_main!(layout_benches);
