use noise::RotatePoint;

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::DynNoiseFn;

impl NodeImpl for RotatePoint<DynNoiseFn> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_f64("x angle", 0.0)
            .input_f64("y angle", 0.0)
            .input_f64("z angle", 0.0)
            .input_f64("u angle", 0.0)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let x = evaluator.get_f64("x angle")?;
        let y = evaluator.get_f64("y angle")?;
        let z = evaluator.get_f64("z angle")?;
        let u = evaluator.get_f64("u angle")?;
        let noise = RotatePoint::new(source).set_angles(x, y, z, u);
        evaluator.output_noise(noise)
    }
}
