use noise::TranslatePoint;

use crate::noise_graph::DynNoiseFn;

use super::NodeImpl;

impl NodeImpl for TranslatePoint<DynNoiseFn> {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_noise("source")
        .input_f64("x", 0.0)
        .input_f64("y", 0.0)
        .input_f64("z", 0.0)
        .input_f64("u", 0.0)
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let x = evaluator.get_f64("x")?;
        let y = evaluator.get_f64("y")?;
        let z = evaluator.get_f64("z")?;
        let u = evaluator.get_f64("u")?;
        let noise = TranslatePoint::new(source).set_all_translations(x, y, z, u);
        evaluator.output_noise(noise)
    }
}