use std::sync::Mutex;

use noise::NoiseFn;

use crate::noise_graph::DynNoiseFn;

use super::NodeImpl;

#[derive(Debug)]
pub struct SyncCache<Source> {
    pub source: Source,
    value: Mutex<Option<f64>>,
    point: Mutex<Vec<f64>>,
}

impl<Source> SyncCache<Source> {
    pub fn new(source: Source) -> Self {
        Self {
            source,
            value: Mutex::new(None),
            point: Mutex::new(Vec::new()),
        }
    }
}

impl NodeImpl for SyncCache<DynNoiseFn> {
    fn build(builder: &mut super::NodeBuilder) {
        builder.input_noise("source")
        .output_noise();
    }

    fn evaluate(evaluator: &mut super::NodeEvaluator) -> anyhow::Result<crate::noise_graph::node_attribute::NodeAttribute> {
        let source = evaluator.get_noise_function("source")?;
        let noise = SyncCache::new(source);
        evaluator.output_noise(noise)
    }
}

impl<Source, const DIM: usize> NoiseFn<f64, DIM> for SyncCache<Source>
where
    Source: NoiseFn<f64, DIM>,
{
    fn get(&self, point: [f64; DIM]) -> f64 {
        let mut value = self.value.lock().unwrap();
        
        match *value {
            Some(value) if quick_eq(&self.point.lock().unwrap(), &point) => value,
            Some(_) | None => {
                let new_value = self.source.get(point);
                *value = Some(new_value);

                let mut cached_point = self.point.lock().unwrap();
                cached_point.clear();
                cached_point.extend_from_slice(&point);

                new_value
            }
        }
    }
}

fn quick_eq(a: &[f64], b: &[f64]) -> bool {
    assert_eq!(a.len(), b.len());
    a.iter().eq(b)
}
