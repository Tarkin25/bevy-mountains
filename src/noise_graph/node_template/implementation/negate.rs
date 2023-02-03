use noise::Negate;

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::DynNoiseFn;

impl NodeImpl for Negate<f64, DynNoiseFn, 2> {
    fn build(builder: &mut NodeBuilder) {
        builder.input_noise("source").output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let noise = Negate::new(source);
        evaluator.output_noise(noise)
    }
}
