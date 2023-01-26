use crate::noise_graph::DynNoiseFn;

use super::NodeImpl;
use noise::Min;

impl NodeImpl for Min<f64, DynNoiseFn, DynNoiseFn, 2> {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_noise("source 1")
        .input_noise("source 2")
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let source_1 = evaluator.get_noise_function("source 1")?;
        let source_2 = evaluator.get_noise_function("source 2")?;
        let noise = Min::new(source_1, source_2);
        evaluator.output_noise(noise)
    }
}