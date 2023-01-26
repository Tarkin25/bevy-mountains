use noise::OpenSimplex;

use super::NodeImpl;

impl NodeImpl for OpenSimplex {
    fn build(builder: &mut super::NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        evaluator.output_noise(OpenSimplex::default())
    }
}