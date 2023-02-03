use crate::noise_graph::graph_ext::NodeEvaluator;
use noise::Perlin;

use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};

impl NodeImpl for Perlin {
    fn build(builder: &mut NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        evaluator.output_noise(noise::Perlin::default())
    }
}
