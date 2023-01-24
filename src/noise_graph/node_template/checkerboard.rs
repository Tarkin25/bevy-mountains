use noise::Checkerboard;

use super::NodeImpl;

impl NodeImpl for Checkerboard {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_usize("size", 1).output_noise();
    }

    fn evaluate(
        evaluator: &mut super::NodeEvaluator,
    ) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let size = evaluator.get_usize("size")?;
        let noise = Checkerboard::new(size);
        evaluator.output_noise(noise)
    }
}
