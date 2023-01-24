use noise::Constant;

use super::NodeImpl;

impl NodeImpl for Constant {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_f64("value", 0.0).output_noise();
    }

    fn evaluate(
        evaluator: &mut super::NodeEvaluator,
    ) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let value = evaluator.get_f64("value")?;
        let noise = Constant::new(value);
        evaluator.output_noise(noise)
    }
}
