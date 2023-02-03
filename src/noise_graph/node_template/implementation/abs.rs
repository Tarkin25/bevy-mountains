use noise::Abs;

use crate::noise_graph::{node_attribute::NodeAttribute, node_template::NodeImpl, DynNoiseFn};

impl NodeImpl for Abs<f64, DynNoiseFn, 2> {
    fn build(builder: &mut crate::noise_graph::node_template::NodeBuilder) {
        builder.input_noise("source").output_noise();
    }

    fn evaluate(
        evaluator: &mut crate::noise_graph::graph_ext::NodeEvaluator,
    ) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let noise = Abs::new(source);
        evaluator.output_noise(noise)
    }
}
