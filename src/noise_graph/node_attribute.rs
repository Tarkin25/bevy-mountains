use std::fmt::Debug;
use bevy_egui::egui::{self, DragValue, ComboBox, TextEdit, Color32};
use egui_node_graph::{WidgetValueTrait, NodeId};
use serde::{Serialize, Deserialize};
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
#[derive(Clone, Serialize, Deserialize, strum::Display)]
pub enum NodeAttribute {
    F64(f64),
    Usize(usize),
    Perlin,
    ScaleBias,
    #[serde(skip)]
    NoiseFunction(DynNoiseFn),
    NoInput,
    NoiseType(NoiseType),
    Operator(Operator),
    Name(String),
    Vec {
        values: Vec<NodeAttribute>,
        template: Box<NodeAttribute>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::Display, strum::EnumIter, Serialize, Deserialize)]
pub enum NoiseType {
    Perlin,
    Simplex,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::Display, strum::EnumIter, Serialize, Deserialize)]
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

impl WidgetValueTrait for NodeAttribute {
    type UserState = NoiseGraphState;
    type NodeData = NodeData;
    type Response = MyResponse;
    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        user_state: &mut NoiseGraphState,
        node_state: &NodeData,
    ) -> Vec<MyResponse> {
        // This trait is used to tell the library which UI to display for the
        // inline parameter widgets.
        match self {
            NodeAttribute::F64(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value).max_decimals(5));
                });
            }
            NodeAttribute::Usize(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(DragValue::new(value));
                });
            },
            NodeAttribute::NoiseType(noise_type) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ComboBox::from_id_source(param_name).selected_text(noise_type.to_string()).show_ui(ui, |ui| {
                        for available in NoiseType::iter() {
                            ui.selectable_value(noise_type, available, available.to_string());
                        }
                    });
                });
            },
            NodeAttribute::Operator(operator) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ComboBox::from_id_source(param_name).selected_text(operator.to_string()).show_ui(ui, |ui| {
                        for available in Operator::iter() {
                            ui.selectable_value(operator, available, available.to_string());
                        }
                    });
                });
            },
            NodeAttribute::Name(name) => {
                ui.add(TextEdit::singleline(name).text_color(Color32::LIGHT_GREEN));
            },
            NodeAttribute::Vec { values, template } => {
                ui.label(param_name);
                ui.indent("values", |ui| {
                    ui.vertical(|ui| {
                        for i in 0..values.len() {
                            ui.horizontal(|ui| {
                                values[i].value_widget(&i.to_string(), node_id, ui, user_state, node_state);
                            });
                        }
                    });
                    if ui.button("Add element").clicked() {
                        values.push(*template.clone());
                    }
                });
            },
            _ => {
                ui.label(param_name);
            }
        }
        // This allows you to return your responses from the inline widgets.
        Vec::new()
    }
}

impl Default for NodeAttribute {
    fn default() -> Self {
        // NOTE: This is just a dummy `Default` implementation. The library
        // requires it to circumvent some internal borrow checker issues.
        Self::F64(0.0)
    }
}

impl NodeAttribute {
    pub fn try_to_f64(self) -> anyhow::Result<f64> {
        if let NodeAttribute::F64(value) = self {
            Ok(value)
        } else {
            self.invalid_cast("F64")
        }
    }

    pub fn try_to_usize(self) -> anyhow::Result<usize> {
        if let NodeAttribute::Usize(value) = self {
            Ok(value)
        } else {
            self.invalid_cast("Usize")
        }
    }

    pub fn try_to_noise_function(self) -> anyhow::Result<DynNoiseFn> {
        if let NodeAttribute::NoiseFunction(noise_function) = self {
            Ok(noise_function)
        } else {
            self.invalid_cast("NoiseFunction")
        }
    }

    pub fn try_to_noise_type(self) -> anyhow::Result<NoiseType> {
        if let NodeAttribute::NoiseType(ty) = self {
            Ok(ty)
        } else {
            self.invalid_cast("NoiseType")
        }
    }

    pub fn try_to_operator(self) -> anyhow::Result<Operator> {
        if let NodeAttribute::Operator(operator) = self {
            Ok(operator)
        } else {
            self.invalid_cast("Operator")
        }
    }

    pub fn try_to_vec(self) -> anyhow::Result<Vec<NodeAttribute>> {
        if let NodeAttribute::Vec { values, .. } = self {
            Ok(values)
        } else {
            self.invalid_cast("Vec")
        }
    }

    fn invalid_cast<T>(self, ty: &str) -> anyhow::Result<T> {
        anyhow::bail!("Invalid cast from {} to {}", self, ty)
    }
}