use bevy_inspector_egui::egui::Ui;
use egui_node_graph::{NodeDataTrait, NodeId, NodeResponse, UserResponseTrait};

use super::{
    ConnectionType, EvaluateNodeFunction, NodeAttribute, NoiseGraph, UserResponse, UserState,
};

pub struct NodeData {
    pub evaluate_node: EvaluateNodeFunction,
}

impl NodeDataTrait for NodeData {
    type Response = UserResponse;
    type UserState = UserState;
    type DataType = ConnectionType;
    type ValueType = NodeAttribute;

    fn bottom_ui(
        &self,
        ui: &mut Ui,
        node_id: NodeId,
        graph: &NoiseGraph,
        user_state: &mut UserState,
    ) -> Vec<NodeResponse<UserResponse, Self>>
    where
        Self::Response: UserResponseTrait,
    {
        todo!()
    }
}
