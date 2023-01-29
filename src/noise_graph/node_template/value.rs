use noise::Value;

use super::NodeImpl;

impl NodeImpl for Value {
    fn build(builder: &mut super::NodeBuilder) {
        builder.output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let noise = Value::default();
        evaluator.output_noise(noise)
    }
}