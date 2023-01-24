use noise::Curve;

use crate::noise_graph::{DynNoiseFn, node_attribute::NodeAttribute};

use super::NodeImpl;

impl NodeImpl for Curve<f64, DynNoiseFn, 2> {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_noise("source")
        .input_vec("control points", NodeAttribute::F64Tuple(0.0, 0.0))
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let control_points = evaluator.get_vec("control points")?;
        let mut noise = Curve::new(source);

        for control_point in control_points {
            let (input, output) = control_point.try_to_f64_tuple()?;
            noise = noise.add_control_point(input, output);
        }

        evaluator.output_noise(noise)
    }
}