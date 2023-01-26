use noise::Negate;

use crate::noise_graph::DynNoiseFn;

use super::NodeImpl;

impl NodeImpl for Negate<f64, DynNoiseFn, 2> {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_noise("source")
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let noise = Negate::new(source);
        evaluator.output_noise(noise)
    }
}