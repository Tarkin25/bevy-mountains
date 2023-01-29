use noise::PerlinSurflet;

use super::NodeImpl;

impl NodeImpl for PerlinSurflet {
    fn build(builder: &mut super::NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let noise = PerlinSurflet::default();
        evaluator.output_noise(noise)
    }
}