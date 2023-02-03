use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use noise::Cylinders;

impl NodeImpl for Cylinders {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_f64("frequency", Self::DEFAULT_FREQUENCY)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let frequency = evaluator.get_f64("frequency")?;
        let noise = Cylinders::new().set_frequency(frequency);
        evaluator.output_noise(noise)
    }
}
