use std::fmt::Debug;
use bevy_egui::egui::{self, DragValue};
use egui_node_graph::{WidgetValueTrait, NodeId};

use super::{DynNoiseFn, NoiseGraphState, NodeData, MyResponse};

/// In the graph, input parameters can optionally have a constant value. This
/// value can be directly edited in a widget inside the node itself.
///
/// There will usually be a correspondence between DataTypes and ValueTypes. But
/// this library makes no attempt to check this consistency. For instance, it is
/// up to the user code in this example to make sure no parameter is created
/// with a DataType of Scalar and a ValueType of Vec2.
//#[derive(custom_debug::Debug)]
#[derive(Clone)]
pub enum Node {
    F64(f64),
    Perlin,
    ScaleBias,
    NoiseFunction(DynNoiseFn),
    NoInput,
}

impl WidgetValueTrait for Node {
    type UserState = NoiseGraphState;
    type NodeData = NodeData;
    type Response = MyResponse;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut NoiseGraphState,
        _node_state: &NodeData,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            Node::F64(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            }
            Node::Perlin => {
                ui.label(param_name);
            },
            Node::ScaleBias => {
                ui.label(param_name);
            },
            Node::NoiseFunction(_) => {},
            Node::NoInput => {},
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            Node::F64(value) => f.debug_tuple("F64").field(value).finish(),
            Node::Perlin => f.write_str("Perlin"),
            Node::ScaleBias => f.write_str("ScaleBias"),
            Node::NoiseFunction(_) => f.write_str("NoiseFunction"),
            Node::NoInput => f.write_str("NoInput"),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::F64(0.0)
    }
}

impl Node {
    pub fn try_to_f64(self) -> anyhow::Result<f64> {
        if let Node::F64(value) = self {
            Ok(value)
        } else {
            anyhow::bail!("Invalid cast from {:?} to f64", self);
        }
    }

    pub fn try_to_noise_function(self) -> anyhow::Result<DynNoiseFn> {
        if let Node::NoiseFunction(noise_function) = self {
            Ok(noise_function)
        } else {
            anyhow::bail!("Invalid cast from {:?} to NoiseFunction", self)
        }
    }
}