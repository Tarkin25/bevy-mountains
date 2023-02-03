use noise::Select;

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::{node_attribute::NodeAttribute, DynNoiseFn};

impl NodeImpl for Select<f64, DynNoiseFn, DynNoiseFn, DynNoiseFn, 2> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source 1")
            .input_noise("source 2")
            .input_noise("control")
            .input_f64("bounds lower", 0.0)
            .input_f64("bounds upper", 1.0)
            .input_f64("falloff", 0.0)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source_1 = evaluator.get_noise_function("source 1")?;
        let source_2 = evaluator.get_noise_function("source 2")?;
        let control = evaluator.get_noise_function("control")?;
        let bounds_lower = evaluator.get_f64("bounds lower")?;
        let bounds_upper = evaluator.get_f64("bounds upper")?;
        let falloff = evaluator.get_f64("falloff")?;
        let noise = noise::Select::new(source_1, source_2, control)
            .set_bounds(bounds_lower, bounds_upper)
            .set_falloff(falloff);
        evaluator.output_noise(noise)
    }
}
