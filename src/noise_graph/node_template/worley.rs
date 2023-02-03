use noise::{
    core::worley::{distance_functions, worley_2d, ReturnType},
    permutationtable::PermutationTable,
    NoiseFn, Seedable,
};

use super::NodeImpl;

#[derive(Clone)]
pub struct SyncWorley {
    distance_function: DistanceFunction,
    return_type: ReturnType,
    frequency: f64,
    seed: u32,
    perm_table: PermutationTable,
}

pub type DistanceFunction = fn(&[f64], &[f64]) -> f64;

impl NodeImpl for SyncWorley {
    fn build(builder: &mut super::NodeBuilder) {
        builder
            .input_f64("frequency", Self::DEFAULT_FREQUENCY)
            .input_return_type()
            .output_noise();
    }

    fn evaluate(
        evaluator: &mut super::NodeEvaluator,
    ) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let frequency = evaluator.get_f64("frequency")?;
        let return_type = evaluator.get_return_type()?;
        let noise = SyncWorley::default()
            .set_return_type(return_type.into())
            .set_frequency(frequency);
        evaluator.output_noise(noise)
    }
}

impl SyncWorley {
    pub const _DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;

    pub fn new(seed: u32) -> Self {
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            distance_function: distance_functions::euclidean,
            return_type: ReturnType::Value,
            frequency: Self::DEFAULT_FREQUENCY,
        }
    }

    /// Enables or disables applying the distance from the nearest seed point
    /// to the output value.
    pub fn set_return_type(self, return_type: ReturnType) -> Self {
        Self {
            return_type,
            ..self
        }
    }

    /// Sets the frequency of the seed points.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }
}

impl Default for SyncWorley {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Seedable for SyncWorley {
    /// Sets the seed value used by the Worley cells.
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

impl NoiseFn<f64, 2> for SyncWorley {
    fn get(&self, [x, y]: [f64; 2]) -> f64 {
        worley_2d(
            &self.perm_table,
            self.distance_function,
            self.return_type,
            [x * self.frequency, y * self.frequency],
        )
    }
}
