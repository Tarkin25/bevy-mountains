use serde::{Deserialize, Serialize};

use crate::noise_graph::node_attribute::NodeAttribute;

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

#[derive(Debug, Serialize, Deserialize)]
pub struct Displace;

impl NodeImpl for Displace {
    fn build(builder: &mut NodeBuilder) {
        builder
        .input_noise("source")
        .input_noise("x")
        .input_noise("y")
        .input_noise("z")
        .input_noise("u")
        .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let x = evaluator.get_noise_function("x")?;
        let y = evaluator.get_noise_function("y")?;
        let z = evaluator.get_noise_function("z")?;
        let u = evaluator.get_noise_function("u")?;
        let noise = noise::Displace::new(source, x, y, z, u);
        evaluator.output_noise(noise)
    }
}