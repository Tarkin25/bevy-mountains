use serde::{Deserialize, Serialize};

use crate::noise_graph::node_attribute::NodeAttribute;

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

#[derive(Debug, Serialize, Deserialize)]
pub struct Terrace;

impl NodeImpl for Terrace {
    fn build(builder: &mut NodeBuilder) {
        builder.input_noise("source")
        .input_vec("control points", NodeAttribute::F64(0.0))
        .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let control_points = evaluator.get_vec("control points")?;
        let mut noise = noise::Terrace::new(source);
        for control_point in control_points {
            noise = noise.add_control_point(control_point.try_to_f64()?);
        }
        evaluator.output_noise(noise)
    }
}