use noise::Abs;

use crate::noise_graph::{node_attribute::NodeAttribute, DynNoiseFn};

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

impl NodeImpl for Abs<f64, DynNoiseFn, 2> {
    fn build(builder: &mut NodeBuilder) {
        builder.input_noise("source").output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let noise = Abs::new(source);
        evaluator.output_noise(noise)
    }
}
