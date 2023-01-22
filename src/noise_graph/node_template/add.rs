use serde::{Deserialize, Serialize};

use crate::noise_graph::node_attribute::NodeAttribute;

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

#[derive(Debug, Serialize, Deserialize)]
pub struct Add;

impl NodeImpl for Add {
    fn build(builder: &mut NodeBuilder) {
        builder.input_noise("source 1")
        .input_noise("source 2")
        .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source_1 = evaluator.get_noise_function("source 1")?;
        let source_2 = evaluator.get_noise_function("source 2")?;
        let noise = noise::Add::new(source_1, source_2);
        evaluator.output_noise(noise)
    }
}