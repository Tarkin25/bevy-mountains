use noise::Cylinders;

use super::NodeImpl;

impl NodeImpl for Cylinders {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_f64("frequency", Self::DEFAULT_FREQUENCY)
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let frequency = evaluator.get_f64("frequency")?;
        let noise = Cylinders::new().set_frequency(frequency);
        evaluator.output_noise(noise)
    }
}