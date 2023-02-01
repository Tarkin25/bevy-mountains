use std::{
    collections::HashMap,
    fmt::Debug,
    fs::OpenOptions,
    io::{BufReader, BufWriter, Write},
    sync::Arc,
};

use anyhow::Context;
use bevy::prelude::*;
use bevy_egui::egui;
use egui_node_graph::{
    Graph, GraphEditorState, NodeDataTrait, NodeId, NodeResponse, OutputId, UserResponseTrait,
};
use noise::{
    utils::{ImageRenderer, NoiseMapBuilder, PlaneMapBuilder},
    Checkerboard, NoiseFn,
};
use serde::{Deserialize, Serialize};

use crate::pause::GameState;

use self::{
    connection_type::ConnectionType,
    node_attribute::NodeAttribute,
    node_template::{AllNodeTemplates, NodeTemplate},
};

mod connection_type;
mod node_attribute;
mod node_template;

pub struct NoiseGraphPlugin;

impl Plugin for NoiseGraphPlugin {
    fn build(&self, app: &mut App) {
        let graph = NoiseGraphResource::load().unwrap_or_else(|e| {
            error!("{}", e);
            Default::default()
        });

        app.insert_resource(graph).add_system_set(
            SystemSet::on_exit(GameState::Paused)
                .with_system(evaluate_graph)
                .with_system(save_graph),
        );
    }
}

fn evaluate_graph(mut graph: ResMut<NoiseGraphResource>) {
    graph.update_current_noise();
}

fn save_graph(graph: Res<NoiseGraphResource>) {
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
}

/// The response type is used to encode side-effects produced when drawing a
/// node in the graph. Most side-effects (creating new nodes, deleting existing
/// nodes, handling connections...) are already handled by the library, but this
/// mechanism allows creating additional side effects from user code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MyResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
    SaveImage,
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default, Serialize, Deserialize)]
pub struct NoiseGraphState {
    active_node: Option<NodeId>,
    #[serde(skip)]
    current_noise: Option<DynNoiseFn>,
}

// =========== Then, you need to implement some traits ============

impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for NodeData {
    type Response = MyResponse;
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
    ) -> Vec<NodeResponse<MyResponse, NodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);
        if graph[node_id]
            .outputs(graph)
            .any(|output| output.typ == ConnectionType::Noise)
        {
            if !is_active {
                if ui.button("👁 Set active").clicked() {
                    responses.push(NodeResponse::User(MyResponse::SetActiveNode(node_id)));
                }
            } else {
                let button =
                    egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                        .fill(egui::Color32::GOLD);
                if ui.add(button).clicked() {
                    responses.push(NodeResponse::User(MyResponse::ClearActiveNode));
                }
                if ui.button("Save image").clicked() {
                    responses.push(NodeResponse::User(MyResponse::SaveImage));
                }
            }
        }

        responses
    }
}

pub type NoiseGraph = Graph<NodeData, ConnectionType, NodeAttribute>;

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct NoiseGraphResource {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: GraphEditorState<NodeData, ConnectionType, NodeAttribute, NodeTemplate, NoiseGraphState>,

    user_state: NoiseGraphState,
}

impl NoiseGraphResource {
    const FILE_PATH: &'static str = "assets/noise_graph.json";

    fn load() -> anyhow::Result<Self> {
        let mut graph: Self = serde_json::from_reader(BufReader::new(
            OpenOptions::new()
                .read(true)
                .open(Self::FILE_PATH)
                .context("Unable to open file")?,
        ))
        .context("Unable to parse json")?;
        graph.update_current_noise();
        Ok(graph)
    }

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
                if let Ok(value) =
                    NodeTemplate::evaluate(&self.state.graph, node, &mut HashMap::new())
                {
                    self.user_state.current_noise = value.try_to_noise_function().ok();
                }
            } else {
                self.user_state.active_node = None;
            }
        }
    }
}

impl egui::Widget for &mut NoiseGraphResource {
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
                    MyResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.active_node = None,
                    MyResponse::SaveImage => {
                        self.update_current_noise();

                        if let Err(e) = self.save_image() {
                            error!("{e}");
                            NoiseGraphResource::debug_text(ui.ctx(), e)
                        }
                    }
                }
            }
        }

        ui.allocate_rect(
            ui.min_rect(),
            egui::Sense::click().union(egui::Sense::drag()),
        )
    }
}

type OutputsCache = HashMap<OutputId, NodeAttribute>;

#[derive(Clone)]
pub struct DynNoiseFn(Arc<dyn NoiseFn<f64, 2> + Send + Sync>);

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
