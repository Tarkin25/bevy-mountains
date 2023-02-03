use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use noise::Constant;

impl NodeImpl for Constant {
    fn build(builder: &mut NodeBuilder) {
        builder.input_f64("value", 0.0).output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let value = evaluator.get_f64("value")?;
        let noise = Constant::new(value);
        evaluator.output_noise(noise)
    }
}
