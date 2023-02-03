use noise::{Perlin, Simplex, Turbulence};

use crate::noise_graph::graph_ext::NodeEvaluator;
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};
use crate::noise_graph::{
    node_attribute::{NodeAttribute, NoiseType},
    DynNoiseFn,
};

impl NodeImpl for Turbulence<DynNoiseFn, Perlin> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise("source")
            .input_noise_type(NoiseType::Perlin)
            .input_f64(
                "frequency",
                noise::Turbulence::<Perlin, Perlin>::DEFAULT_FREQUENCY,
            )
            .input_f64("power", noise::Turbulence::<Perlin, Perlin>::DEFAULT_POWER)
            .input_usize(
                "roughness",
                noise::Turbulence::<Perlin, Perlin>::DEFAULT_ROUGHNESS,
            )
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let frequency = evaluator.get_f64("frequency")?;
        let power = evaluator.get_f64("power")?;
        let roughness = evaluator.get_usize("roughness")?;

        match evaluator.get_noise_type()? {
            NoiseType::Perlin => {
                let noise = noise::Turbulence::<_, Perlin>::new(source)
                    .set_frequency(frequency)
                    .set_power(power)
                    .set_roughness(roughness);
                evaluator.output_noise(noise)
            }
            NoiseType::Simplex => {
                let noise = noise::Turbulence::<_, Simplex>::new(source)
                    .set_frequency(frequency)
                    .set_power(power)
                    .set_roughness(roughness);
                evaluator.output_noise(noise)
            }
        }
    }
}
