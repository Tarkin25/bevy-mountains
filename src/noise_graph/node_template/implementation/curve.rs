use noise::Curve;

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::{node_attribute::NodeAttribute, DynNoiseFn};

impl NodeImpl for Curve<f64, DynNoiseFn, 2> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_vec("control points", NodeAttribute::F64Tuple(0.0, 0.0))
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let control_points = evaluator.get_vec("control points")?;
        if control_points.len() < 4 {
            anyhow::bail!("Curve needs at least 4 control points");
        }
        let mut noise = Curve::new(source);

        for control_point in control_points {
            let (input, output) = control_point.try_to_f64_tuple()?;
            noise = noise.add_control_point(input, output);
        }

        evaluator.output_noise(noise)
    }
}
