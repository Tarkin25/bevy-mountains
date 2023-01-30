use noise::NoiseFn;

use crate::noise_graph::DynNoiseFn;

use super::NodeImpl;

pub struct Scale<Source> {
    source: Source,
    scale: f64,
}

impl<Source> Scale<Source> {
    pub fn new(source: Source, scale: f64) -> Self {
        Self { source, scale }
    }
}

impl<Source: NoiseFn<f64, 2>> NoiseFn<f64, 2> for Scale<Source> {
    fn get(&self, [x, y]: [f64; 2]) -> f64 {
        self.source.get([x / self.scale, y / self.scale]) * self.scale / 2.0
    }
}

impl NodeImpl for Scale<DynNoiseFn> {
    fn build(builder: &mut super::NodeBuilder) {
        builder
        .input_noise("source")
        .input_f64("scale", 1.0)
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let scale = evaluator.get_f64("scale")?;
        let noise = Scale::new(source, scale);
        evaluator.output_noise(noise)
    }
}