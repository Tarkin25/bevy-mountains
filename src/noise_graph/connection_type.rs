use std::borrow::Cow;

use bevy_egui::egui::Color32;
use egui_node_graph::DataTypeTrait;
use serde::{Serialize, Deserialize};

use super::NoiseGraphState;

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq, Serialize, Deserialize, strum::Display)]
pub enum ConnectionType {
    Noise,
    NoiseType,
    F64,
    Usize,
}

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<NoiseGraphState> for ConnectionType {
    fn data_type_color(&self, _user_state: &mut NoiseGraphState) -> Color32 {
        match self {
            ConnectionType::Noise => Color32::BLUE,
            ConnectionType::F64 => Color32::YELLOW,
            ConnectionType::NoiseType => Color32::LIGHT_RED,
            ConnectionType::Usize => Color32::BROWN,
        }
    }

    fn name(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }
}