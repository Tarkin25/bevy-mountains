use serde::{Deserialize, Serialize};

use crate::noise_graph::node_attribute::NodeAttribute;

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalePoint;

impl NodeImpl for ScalePoint {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_f64("x", 1.0)
            .input_f64("y", 1.0)
            .input_f64("z", 1.0)
            .input_f64("u", 1.0)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let x = evaluator.get_f64("x")?;
        let y = evaluator.get_f64("y")?;
        let z = evaluator.get_f64("z")?;
        let u = evaluator.get_f64("u")?;
        let noise = noise::ScalePoint::new(source).set_all_scales(x, y, z, u);
        evaluator.output_noise(noise)
    }
}
