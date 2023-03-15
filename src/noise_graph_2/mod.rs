mod connection_type;
mod graph_editor;
mod manager;
mod node_attribute;
mod node_data;
mod node_template;
mod user_response;

use std::sync::Arc;

pub use connection_type::*;
use egui_node_graph::Graph;
pub use graph_editor::*;
pub use manager::*;
pub use node_attribute::*;
pub use node_data::*;
pub use node_template::*;
use noise::NoiseFn;
pub use user_response::*;

pub type NoiseGraph = Graph<NodeData, ConnectionType, NodeAttribute>;
pub type EvaluateNodeFunction = fn();
pub type BuildNodeFunction = fn(NodeBuilder);

#[derive(Clone)]
pub struct DynNoiseFn(Arc<dyn NoiseFn<f64, 2> + Send + Sync + 'static>);

impl DynNoiseFn {
    fn new<T: NoiseFn<f64, 2> + Send + Sync + 'static>(noise: T) -> Self {
        Self(Arc::new(noise))
    }
}

impl NoiseFn<f64, 2> for DynNoiseFn {
    fn get(&self, point: [f64; 2]) -> f64 {
        self.0.get(point)
    }
}
