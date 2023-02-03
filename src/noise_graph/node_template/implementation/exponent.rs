use noise::Exponent;

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::DynNoiseFn;

impl NodeImpl for Exponent<f64, DynNoiseFn, 2> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_f64("exponent", 1.0)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let exponent = evaluator.get_f64("exponent")?;
        let noise = Exponent::new(source).set_exponent(exponent);
        evaluator.output_noise(noise)
    }
}
