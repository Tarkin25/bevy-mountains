use std::fmt::Debug;
use bevy_egui::egui::{self, DragValue, ComboBox};
use egui_node_graph::{WidgetValueTrait, NodeId};
use strum::IntoEnumIterator;

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
pub enum NodeValue {
    F64(f64),
    Usize(usize),
    Perlin,
    ScaleBias,
    NoiseFunction(DynNoiseFn),
    NoInput,
    NoiseType(NoiseType),
    Operator(Operator),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::Display, strum::EnumIter)]
pub enum NoiseType {
    Perlin,
    Simplex,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::Display, strum::EnumIter)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn apply(self, a: f64, b: f64) -> f64 {
        use Operator::*;
        
        match self {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
        }
    }
}

impl WidgetValueTrait for NodeValue {
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
            NodeValue::F64(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value).max_decimals(5));
                });
            }
            NodeValue::Usize(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            },
            NodeValue::Perlin => {
                ui.label(param_name);
            },
            NodeValue::ScaleBias => {
                ui.label(param_name);
            },
            NodeValue::NoiseFunction(_) => {},
            NodeValue::NoInput => {},
            NodeValue::NoiseType(noise_type) => {
                ui.horizontal(|ui| {
                    ComboBox::from_label(param_name).selected_text(noise_type.to_string()).show_ui(ui, |ui| {
                        for available in NoiseType::iter() {
                            ui.selectable_value(noise_type, available, available.to_string());
                        }
                    });
                });
            },
            NodeValue::Operator(operator) => {
                ComboBox::from_label(param_name).selected_text(operator.to_string()).show_ui(ui, |ui| {
                    for available in Operator::iter() {
                        ui.selectable_value(operator, available, available.to_string());
                    }
                });
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

impl Debug for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            NodeValue::F64(value) => f.debug_tuple("F64").field(value).finish(),
            NodeValue::Perlin => f.write_str("Perlin"),
            NodeValue::ScaleBias => f.write_str("ScaleBias"),
            NodeValue::NoiseFunction(_) => f.write_str("NoiseFunction"),
            NodeValue::NoInput => f.write_str("NoInput"),
            NodeValue::NoiseType(noise_type) => f.debug_tuple("NoiseType").field(noise_type).finish(),
            NodeValue::Usize(value) => f.debug_tuple("Usize").field(value).finish(),
            NodeValue::Operator(operator) => f.debug_tuple("Operator").field(operator).finish()
        }
    }
}

impl Default for NodeValue {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::F64(0.0)
    }
}

impl NodeValue {
    pub fn try_to_f64(self) -> anyhow::Result<f64> {
        if let NodeValue::F64(value) = self {
            Ok(value)
        } else {
            self.invalid_cast("F64")
        }
    }

    pub fn try_to_usize(self) -> anyhow::Result<usize> {
        if let NodeValue::Usize(value) = self {
            Ok(value)
        } else {
            self.invalid_cast("Usize")
        }
    }

    pub fn try_to_noise_function(self) -> anyhow::Result<DynNoiseFn> {
        if let NodeValue::NoiseFunction(noise_function) = self {
            Ok(noise_function)
        } else {
            self.invalid_cast("NoiseFunction")
        }
    }

    pub fn try_to_noise_type(self) -> anyhow::Result<NoiseType> {
        if let NodeValue::NoiseType(ty) = self {
            Ok(ty)
        } else {
            self.invalid_cast("NoiseType")
        }
    }

    pub fn try_to_operator(self) -> anyhow::Result<Operator> {
        if let NodeValue::Operator(operator) = self {
            Ok(operator)
        } else {
            self.invalid_cast("Operator")
        }
    }

    fn invalid_cast<T>(self, ty: &str) -> anyhow::Result<T> {
        anyhow::bail!("Invalid cast from {:?} to {}", self, ty)
    }
}