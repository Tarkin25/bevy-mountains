use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};

pub struct Float;

impl NodeImpl for Float {
    fn build(builder: &mut NodeBuilder) {
        builder.input_f64("value", 1.0).output_number();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let value = evaluator.get_f64("value")?;
        evaluator.output_number(value)
    }
}
