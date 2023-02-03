use noise::TranslatePoint;

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::DynNoiseFn;

impl NodeImpl for TranslatePoint<DynNoiseFn> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_f64("x", 0.0)
            .input_f64("y", 0.0)
            .input_f64("z", 0.0)
            .input_f64("u", 0.0)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let x = evaluator.get_f64("x")?;
        let y = evaluator.get_f64("y")?;
        let z = evaluator.get_f64("z")?;
        let u = evaluator.get_f64("u")?;
        let noise = TranslatePoint::new(source).set_all_translations(x, y, z, u);
        evaluator.output_noise(noise)
    }
}
