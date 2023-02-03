use crate::noise_graph::graph_ext::NodeEvaluator;
use noise::{Fbm, MultiFractal, Perlin, Simplex};

use crate::noise_graph::node_attribute::{NodeAttribute, NoiseType};
use crate::noise_graph::node_template::{NodeBuilder, NodeImpl};

impl NodeImpl for Fbm<Perlin> {
    fn build(builder: &mut NodeBuilder) {
        builder
            .input_noise_type(NoiseType::Perlin)
            .input_usize("octaves", noise::Fbm::<Perlin>::DEFAULT_OCTAVE_COUNT)
            .input_f64("frequency", noise::Fbm::<Perlin>::DEFAULT_FREQUENCY)
            .input_f64("lacunarity", noise::Fbm::<Perlin>::DEFAULT_LACUNARITY)
            .input_f64("persistence", noise::Fbm::<Perlin>::DEFAULT_PERSISTENCE)
            .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let octaves = evaluator.get_usize("octaves")?;
        let frequency = evaluator.get_f64("frequency")?;
        let lacunarity = evaluator.get_f64("lacunarity")?;
        let persistence = evaluator.get_f64("persistence")?;

        match evaluator.get_noise_type()? {
            NoiseType::Perlin => {
                let noise = noise::Fbm::<Perlin>::default()
                    .set_octaves(octaves)
                    .set_frequency(frequency)
                    .set_lacunarity(lacunarity)
                    .set_persistence(persistence);
                evaluator.output_noise(noise)
            }
            NoiseType::Simplex => {
                let noise = noise::Fbm::<Simplex>::default()
                    .set_octaves(octaves)
                    .set_frequency(frequency)
                    .set_lacunarity(lacunarity)
                    .set_persistence(persistence);
                evaluator.output_noise(noise)
            }
        }
    }
}
