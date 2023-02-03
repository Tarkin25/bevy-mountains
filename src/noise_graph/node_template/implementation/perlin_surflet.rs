use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use noise::PerlinSurflet;

impl NodeImpl for PerlinSurflet {
    fn build(builder: &mut NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let noise = PerlinSurflet::default();
        evaluator.output_noise(noise)
    }
}
