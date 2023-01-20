use std::sync::Mutex;

use noise::NoiseFn;

#[derive(Debug)]
pub struct SyncCache<Source> {
    pub source: Source,
    value: Mutex<Option<f64>>,
    point: Mutex<Vec<f64>>,
}

impl<Source> SyncCache<Source> {
    pub fn _new(source: Source) -> Self {
        Self {
            source,
            value: Mutex::new(None),
            point: Mutex::new(Vec::new()),
        }
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
