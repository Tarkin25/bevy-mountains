use bevy::prelude::*;
use bevy_inspector_egui::egui::{Response, Sense, Ui, Widget};
use serde::{Deserialize, Serialize};
use crate::noise_graph::node_template::{AllNodeTemplates, NodeTemplate};
use crate::noise_graph::{NoiseGraph, NodeResponse, UserResponse};

#[derive(Resource, Serialize, Deserialize)]
pub struct NoiseGraphManager {
    noise_graphs: Vec<NoiseGraph>,
    active_graph: Option<GraphId>,
    graph_stack: Vec<GraphId>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphId(usize);

pub enum ManagerMessage {
    CreateSubGraph,
}

struct AvailableNodeTemplates(Vec<NodeTemplate>);

impl Default for NoiseGraphManager {
    fn default() -> Self {
        Self {
            noise_graphs: vec![NoiseGraph::default()],
            active_graph: Some(GraphId(0)),
            graph_stack: vec![GraphId(0)],
        }
    }
}

impl Widget for &mut NoiseGraphManager {
    fn ui(self, ui: &mut Ui) -> Response {
        let next_graph_id = GraphId(self.noise_graphs.len());
        let show_close_button = self.graph_stack.len() > 1;
        let active_graph = self.active_graph_mut();
        active_graph.user_state.next_graph_id = next_graph_id;

        let mut show_sub_graph = None;
        let mut close_active_graph = false;

        if show_close_button && ui.button("Close").clicked() {
            close_active_graph = true;
        }

        let graph_response = active_graph.state.draw_graph_editor(ui, AllNodeTemplates, &mut active_graph.user_state);

        for node_response in graph_response.node_responses {
            if let NodeResponse::User(user_response) = node_response {
                match user_response {
                    UserResponse::SetActiveNode(node) => active_graph.user_state.active_node = Some(node),
                    UserResponse::ClearActiveNode => active_graph.user_state.active_node = None,
                    UserResponse::SaveImage => info!("TODO - save image??"),
                    UserResponse::ShowSubGraph(id) => show_sub_graph = Some(id),
                    UserResponse::CloseSubGraph => close_active_graph = true,
                }
            }
        }

        if let Some(message) = active_graph.user_state.message_to_manager.take() {
            match message {
                ManagerMessage::CreateSubGraph => self.create_sub_graph()
            }
        }

        if let Some(id) = show_sub_graph {
            self.push_sub_graph(id);
        }
        if close_active_graph {
            self.close_active_graph();
        }

        ui.allocate_rect(
            ui.min_rect(),
            Sense::click().union(Sense::drag()),
        )
    }
}

impl NoiseGraphManager {
    fn create_sub_graph(&mut self) {
        self.noise_graphs.push(NoiseGraph::default());
    }

    fn push_sub_graph(&mut self, id: GraphId) {
        self.graph_stack.push(id);
        self.active_graph = Some(id);
    }

    fn close_active_graph(&mut self) {
        self.graph_stack.pop();
        self.active_graph = self.graph_stack.last().copied();
    }

    fn active_graph_mut(&mut self) -> &mut NoiseGraph {
        &mut self.noise_graphs[self.active_graph.expect("No active graph in graph manager").0]
    }
}