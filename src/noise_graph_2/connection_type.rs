use std::borrow::Cow;

use bevy_inspector_egui::egui::Color32;
use egui_node_graph::DataTypeTrait;
use serde::{Deserialize, Serialize};

use super::UserState;

#[derive(PartialEq, Eq, Serialize, Deserialize, strum::Display)]
pub enum ConnectionType {
    NoConnection,
    Noise,
    F64,
    Usize,
}

impl DataTypeTrait<UserState> for ConnectionType {
    fn data_type_color(&self, _user_state: &mut UserState) -> Color32 {
        match self {
            ConnectionType::NoConnection => Color32::BLACK,
            ConnectionType::Noise => Color32::BLUE,
            ConnectionType::F64 => Color32::YELLOW,
            ConnectionType::Usize => Color32::BROWN,
        }
    }

    fn name(&self) -> Cow<str> {
        Cow::from(self.to_string())
    }
}
