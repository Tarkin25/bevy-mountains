use std::{
    fmt::Debug,
    fs::OpenOptions,
    io::{BufWriter, Write},
    sync::Arc,
};

use anyhow::Context;
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_inspector_egui::bevy_egui::egui;
use egui_node_graph::{
    Graph, GraphEditorState, NodeDataTrait, NodeId, NodeResponse, UserResponseTrait,
};
use noise::{
    utils::{ImageRenderer, NoiseMapBuilder, PlaneMapBuilder},
    Checkerboard, NoiseFn,
};
use serde::{Deserialize, Serialize};
use crate::noise_graph::graph_manager::{GraphId, ManagerMessage, NoiseGraphManager};

use crate::pause::GameState;

use self::{
    connection_type::ConnectionType,
    graph_ext::GraphExt,
    node_attribute::NodeAttribute,
    node_template::{AllNodeTemplates, NodeTemplate},
};

mod connection_type;
mod graph_ext;
mod node_attribute;
mod node_template;
pub mod graph_manager;

pub struct NoiseGraphPlugin; // TODO - use asset handles all over + save extension for AssetServer

impl Plugin for NoiseGraphPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NoiseGraphManager>()
            .add_system_set(
            SystemSet::on_enter(GameState::Running)
                .with_system(evaluate_graph)
                .with_system(save_graph),
        );
    }
}

fn evaluate_graph(mut graph: ResMut<NoiseGraph>) {
    graph.update_current_noise();
}

fn save_graph(graph: Res<NoiseGraph>) {
    if let Err(e) = graph.save() {
        error!("Error while saving noise graph: {}", e);
    }
}

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[derive(Serialize, Deserialize)]
pub struct NodeData {
    template: NodeTemplate,
    graph_id: Option<GraphId>,
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UserResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
    SaveImage,
    ShowSubGraph(GraphId),
    CloseSubGraph,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default, Serialize, Deserialize)]
pub struct NoiseGraphState {
    active_node: Option<NodeId>,
    #[serde(skip)]
    current_noise: Option<DynNoiseFn>,
    #[serde(skip)]
    message_to_manager: Option<ManagerMessage>,
    #[serde(skip)]
    next_graph_id: GraphId,
}

#[derive(Default, Resource, Serialize, Deserialize, TypeUuid)]
#[uuid = "b452a8a1-82fe-42a1-be25-c931c310e008"]
pub struct NoiseGraph {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: GraphEditorState<NodeData, ConnectionType, NodeAttribute, NodeTemplate, NoiseGraphState>,

    user_state: NoiseGraphState,
}

#[derive(Clone)]
pub struct DynNoiseFn(Arc<dyn NoiseFn<f64, 2> + Send + Sync>);

// =========== Then, you need to implement some traits ============

impl UserResponseTrait for UserResponse {}

impl NodeDataTrait for NodeData {
    type Response = UserResponse;
    type UserState = NoiseGraphState;
    type DataType = ConnectionType;
    type ValueType = NodeAttribute;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        graph: &Graph<NodeData, ConnectionType, NodeAttribute>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<UserResponse, NodeData>>
    where
        UserResponse: UserResponseTrait,
    {
        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        let outputs_noise = graph[node_id]
            .outputs(graph)
            .any(|output| output.typ == ConnectionType::Noise);

        if outputs_noise {
            if !is_active {
                if ui.button("👁 Set active").clicked() {
                    responses.push(NodeResponse::User(UserResponse::SetActiveNode(node_id)));
                }
            } else {
                let button =
                    egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                        .fill(egui::Color32::GOLD);
                if ui.add(button).clicked() {
                    responses.push(NodeResponse::User(UserResponse::ClearActiveNode));
                }
                if ui.button("Save image").clicked() {
                    responses.push(NodeResponse::User(UserResponse::SaveImage));
                }
            }
        }

        if matches!(graph[node_id].user_data.template, NodeTemplate::SubGraph) {
            if ui.button("Show").clicked() {
                responses.push(NodeResponse::User(UserResponse::ShowSubGraph(graph[node_id].user_data.graph_id.expect("SubGraph node without graph id"))));
            }
        }

        responses
    }
}

impl NoiseGraph {
    const FILE_PATH: &'static str = "assets/noise_graph.json";

    fn save(&self) -> anyhow::Result<()> {
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(Self::FILE_PATH)
                .context("Unable to open file")?,
        );
        serde_json::to_writer_pretty(&mut writer, &self).context("Unable to save to json")?;
        writer.flush().context("Unable to save file")
    }

    fn save_image(&self) -> anyhow::Result<()> {
        let node = self
            .user_state
            .active_node
            .map(|id| &self.state.graph[id])
            .ok_or(anyhow::anyhow!("No active node"))?;
        let name = match self.state.graph.get_input(node.get_input("name")?).value() {
            NodeAttribute::Name(name) => name,
            _ => anyhow::bail!("Node doesn't have a name"),
        };

        let half_bounds = 2048.0;
        let size = 1024;
        let map = PlaneMapBuilder::new(self.get_noise_fn())
            .set_size(size, size)
            .set_x_bounds(-half_bounds, half_bounds)
            .set_y_bounds(-half_bounds, half_bounds)
            .build();

        ImageRenderer::new()
            .set_gradient(noise::utils::ColorGradient::new().build_terrain_gradient())
            .render(&map)
            .write_to_file(&format!("{name}.png"));

        Ok(())
    }

    pub fn get_noise_fn(&self) -> DynNoiseFn {
        self.user_state
            .current_noise
            .as_ref()
            .map(|noise| noise.clone())
            .unwrap_or_else(|| DynNoiseFn::new(Checkerboard::default()))
    }

    fn debug_text(ctx: &egui::Context, text: impl ToString) {
        ctx.debug_painter().text(
            egui::pos2(10.0, 35.0),
            egui::Align2::LEFT_TOP,
            text,
            egui::TextStyle::Button.resolve(&ctx.style()),
            egui::Color32::WHITE,
        );
    }

    fn update_current_noise(&mut self) {
        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                if let Ok(value) = self.state.graph.evaluate(node) {
                    self.user_state.current_noise = value.try_to_noise_function().ok();
                }
            } else {
                self.user_state.active_node = None;
            }
        }
    }
}

impl egui::Widget for &mut NoiseGraph {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let graph_response =
            self.state
                .draw_graph_editor(ui, AllNodeTemplates, &mut self.user_state);

        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    UserResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    UserResponse::ClearActiveNode => self.user_state.active_node = None,
                    UserResponse::SaveImage => {
                        self.update_current_noise();

                        if let Err(e) = self.save_image() {
                            error!("{e}");
                            NoiseGraph::debug_text(ui.ctx(), e)
                        }
                    },
                    UserResponse::ShowSubGraph(id) => {

                    },
                    UserResponse::CloseSubGraph => {

                    },
                }
            }
        }

        ui.allocate_rect(
            ui.min_rect(),
            egui::Sense::click().union(egui::Sense::drag()),
        )
    }
}

impl DynNoiseFn {
    fn new<T: NoiseFn<f64, 2> + Send + Sync + 'static>(noise: T) -> Self {
        Self(Arc::new(noise))
    }
}

impl NoiseFn<f64, 2> for DynNoiseFn {
    fn get(&self, point: [f64; 2]) -> f64 {
        self.0.get(point)
    }
}
