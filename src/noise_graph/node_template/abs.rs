use serde::{Deserialize, Serialize};

use crate::noise_graph::node_attribute::NodeAttribute;

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

#[derive(Debug, Serialize, Deserialize)]
pub struct Abs;

impl NodeImpl for Abs {
    fn build(builder: &mut NodeBuilder) {
        builder.input_noise("source")
        .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let noise = noise::Abs::new(source);
        evaluator.output_noise(noise)
    }
}