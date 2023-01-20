use std::borrow::Cow;

use bevy_egui::egui::Color32;
use egui_node_graph::DataTypeTrait;
use serde::{Serialize, Deserialize};

use super::NoiseGraphState;

/// `DataType`s are what defines the possible range of connections when
/// attaching two ports together. The graph UI will make sure to not allow
/// attaching incompatible datatypes.
#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionType {
    Noise,
    NoiseType,
    Number,
}

// A trait for the data types, to tell the library how to display them
impl DataTypeTrait<NoiseGraphState> for ConnectionType {
    fn data_type_color(&self, _user_state: &mut NoiseGraphState) -> Color32 {
        match self {
            ConnectionType::Noise => Color32::BLUE,
            ConnectionType::Number => Color32::LIGHT_YELLOW,
            ConnectionType::NoiseType => Color32::LIGHT_RED,
        }
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            ConnectionType::Noise => Cow::Borrowed("noise"),
            ConnectionType::Number => Cow::Borrowed("number"),
            ConnectionType::NoiseType => Cow::Borrowed("noise type"),
        }
    }
}