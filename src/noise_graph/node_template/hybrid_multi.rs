use noise::{HybridMulti, Perlin, MultiFractal, Simplex};

use crate::noise_graph::node_attribute::NoiseType;

use super::NodeImpl;

impl NodeImpl for HybridMulti<Perlin> {
    fn build(builder: &mut super::NodeBuilder) {
       builder.input_noise_type(NoiseType::Perlin)
       .input_usize("octaves", Self::DEFAULT_OCTAVES)
       .input_f64("frequency", Self::DEFAULT_FREQUENCY)
       .input_f64("lacunarity", Self::DEFAULT_LACUNARITY)
       .input_f64("persistence", Self::DEFAULT_PERSISTENCE)
       .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let octaves = evaluator.get_usize("octaves")?;
        let frequency = evaluator.get_f64("frequency")?;
        let lacunarity = evaluator.get_f64("lacunarity")?;
        let persistence = evaluator.get_f64("persistence")?;

        match evaluator.get_noise_type()? {
            NoiseType::Perlin => {
                let noise = HybridMulti::<Perlin>::default()
                .set_octaves(octaves)
                .set_frequency(frequency)
                .set_lacunarity(lacunarity)
                .set_persistence(persistence);
                evaluator.output_noise(noise)
            },
            NoiseType::Simplex => {
                let noise = HybridMulti::<Simplex>::default()
                .set_octaves(octaves)
                .set_frequency(frequency)
                .set_lacunarity(lacunarity)
                .set_persistence(persistence);
                evaluator.output_noise(noise)
            }
        }
    }
}