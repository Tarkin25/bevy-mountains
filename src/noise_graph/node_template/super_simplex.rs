use noise::SuperSimplex;

use super::NodeImpl;

impl NodeImpl for SuperSimplex {
    fn build(builder: &mut super::NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let noise = SuperSimplex::default();
        evaluator.output_noise(noise)
    }
}