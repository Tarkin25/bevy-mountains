use noise::Perlin;

use crate::noise_graph::node_attribute::NodeAttribute;

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

impl NodeImpl for Perlin {
    fn build(builder: &mut NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        evaluator.output_noise(noise::Perlin::default())
    }
}
