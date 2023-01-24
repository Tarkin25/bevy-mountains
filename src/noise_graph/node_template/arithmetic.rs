use crate::noise_graph::node_attribute::{NodeAttribute, Operator};

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

pub struct Arithmetic;

impl NodeImpl for Arithmetic {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_operator("operator", Operator::Add)
            .input_f64("a", 0.0)
            .input_f64("b", 0.0)
            .output_number();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let operator = evaluator.get_operator("operator")?;
        let a = evaluator.get_f64("a")?;
        let b = evaluator.get_f64("b")?;
        evaluator.output_number(operator.apply(a, b))
    }
}
