use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_attribute::NodeAttribute;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use noise::Checkerboard;

impl NodeImpl for Checkerboard {
    fn build(builder: &mut NodeBuilder) {
        builder.input_usize("size", 1).output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let size = evaluator.get_usize("size")?;
        let noise = Checkerboard::new(size);
        evaluator.output_noise(noise)
    }
}
