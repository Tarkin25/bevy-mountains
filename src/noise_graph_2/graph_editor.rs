use egui_node_graph::GraphEditorState;

use super::{ConnectionType, ManagerMessage, NodeAttribute, NodeData, NodeTemplate};

#[derive(Default)]
pub struct NoiseGraphEditor {
    pub editor: GraphEditorState<NodeData, ConnectionType, NodeAttribute, NodeTemplate, UserState>,
    pub user_state: UserState,
}

#[derive(Default)]
pub struct UserState {
    pub manager_messages: Vec<ManagerMessage>,
}
