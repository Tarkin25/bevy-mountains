use noise::{NoiseFn, Seedable};

pub struct PhantomSeedable<Source>(pub Source);

impl<Source: Default> Default for PhantomSeedable<Source> {
    fn default() -> Self {
        Self(Source::default())
    }
}

impl<Source> Seedable for PhantomSeedable<Source> {
    fn seed(&self) -> u32 {
        0
    }

    fn set_seed(self, _seed: u32) -> Self {
        self
    }
}

impl<Source, const DIM: usize> NoiseFn<f64, DIM> for PhantomSeedable<Source>
where
    Source: NoiseFn<f64, DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        self.0.get(point)
    }
}
