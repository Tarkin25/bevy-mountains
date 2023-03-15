use bevy_inspector_egui::egui::{Response, Ui, Widget};
use slotmap::SlotMap;

use super::NoiseGraphEditor;

pub struct NoiseGraphManager {
    graphs: SlotMap<GraphId, NoiseGraphEditor>,
    active_graph: GraphId,
    graph_stack: Vec<GraphId>,
    next_graph: GraphId,
}

slotmap::new_key_type! {
    pub struct GraphId;
}

pub enum ManagerMessage {
    CreateSubGraph,
}

impl Widget for &mut NoiseGraphManager {
    fn ui(self, ui: &mut Ui) -> Response {
        todo!()
    }
}

impl Default for NoiseGraphManager {
    fn default() -> Self {
        let mut graphs = SlotMap::with_key();
        let active_graph = graphs.insert(NoiseGraphEditor::default());
        let next_graph = graphs.insert(NoiseGraphEditor::default());

        Self {
            graphs,
            active_graph,
            graph_stack: vec![active_graph],
            next_graph,
        }
    }
}
