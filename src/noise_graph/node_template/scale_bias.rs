use noise::ScaleBias;

use crate::noise_graph::{node_attribute::NodeAttribute, DynNoiseFn};

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

impl NodeImpl for ScaleBias<f64, DynNoiseFn, 2> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_f64("scale", 1.0)
            .input_f64("bias", 0.0)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let scale = evaluator.get_f64("scale")?;
        let bias = evaluator.get_f64("bias")?;
        let source = evaluator.get_noise_function("source")?;
        let noise = noise::ScaleBias::new(source.clone())
            .set_scale(scale)
            .set_bias(bias);
        evaluator.output_noise(noise)
    }
}
