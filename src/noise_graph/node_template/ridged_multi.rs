use noise::{Perlin, MultiFractal, Simplex};
use serde::{Deserialize, Serialize};

use crate::noise_graph::node_attribute::{NodeAttribute, NoiseType};

use super::{NodeBuilder, NodeEvaluator, NodeImpl};

#[derive(Debug, Serialize, Deserialize)]
pub struct RidgedMulti;

impl NodeImpl for RidgedMulti {
    fn build(builder: &mut NodeBuilder) {
        builder.input_noise_type(NoiseType::Perlin)
                .input_usize("octaves", noise::RidgedMulti::<Perlin>::DEFAULT_OCTAVE_COUNT)
                .input_f64("frequency", noise::RidgedMulti::<Perlin>::DEFAULT_FREQUENCY)
                .input_f64("lacunarity", noise::RidgedMulti::<Perlin>::DEFAULT_LACUNARITY)
                .input_f64("persistence", noise::RidgedMulti::<Perlin>::DEFAULT_PERSISTENCE)
                .input_f64("attenuation", noise::RidgedMulti::<Perlin>::DEFAULT_ATTENUATION)
                .output_noise();
    }

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute> {
        let octaves = evaluator.get_usize("octaves")?;
            let frequency = evaluator.get_f64("frequency")?;
            let lacunarity = evaluator.get_f64("lacunarity")?;
            let persistence = evaluator.get_f64("persistence")?;
            let attenuation = evaluator.get_f64("attenuation")?;

            match evaluator.get_noise_type()? {
                NoiseType::Perlin => {
                    let noise = noise::RidgedMulti::<Perlin>::default()
                        .set_octaves(octaves)
                        .set_frequency(frequency)
                        .set_lacunarity(lacunarity)
                        .set_persistence(persistence)
                        .set_attenuation(attenuation);
                    evaluator.output_noise(noise)
                },
                NoiseType::Simplex => {
                    let noise = noise::RidgedMulti::<Simplex>::default()
                        .set_octaves(octaves)
                        .set_frequency(frequency)
                        .set_lacunarity(lacunarity)
                        .set_persistence(persistence)
                        .set_attenuation(attenuation);
                    evaluator.output_noise(noise)
                }
            }
    }
}