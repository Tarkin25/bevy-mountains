use std::{collections::HashMap, fmt::Debug, sync::Arc};

use bevy::prelude::*;
use bevy_egui::{egui::{self, TextStyle}, EguiContext};
use egui_node_graph::{NodeId, UserResponseTrait, NodeDataTrait, Graph, GraphEditorState, NodeResponse, OutputId};
use noise::{NoiseFn, Checkerboard};

use crate::pause::GameState;

use self::{connection_type::ConnectionType, node_template::{NodeTemplate, AllNodeTemplates}, node_value::NodeValue, evaluate::evaluate_node};

mod connection_type;
mod node_template;
mod node_value;
mod evaluate;

pub struct NoiseGraphPlugin;

impl Plugin for NoiseGraphPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<NoiseGraph>()
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(draw_graph));
    }
}

fn draw_graph(mut context: ResMut<EguiContext>, mut graph: ResMut<NoiseGraph>) {
    graph.draw(context.ctx_mut());
}

// ========= First, define your user data types =============

/// The NodeData holds a custom data struct inside each node. It's useful to
/// store additional information that doesn't live in parameters. For this
/// example, the node data stores the template (i.e. the "type") of the node.
#[derive(Default)]
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
}

/// The graph 'global' state. This state struct is passed around to the node and
/// parameter drawing callbacks. The contents of this struct are entirely up to
/// the user. For this example, we use it to keep track of the 'active' node.
#[derive(Default)]
pub struct NoiseGraphState {
    active_node: Option<NodeId>,
    current_noise: Option<DynNoiseFn>,
}

// =========== Then, you need to implement some traits ============

impl UserResponseTrait for MyResponse {}
impl NodeDataTrait for NodeData {
    type Response = MyResponse;
    type UserState = NoiseGraphState;
    type DataType = ConnectionType;
    type ValueType = NodeValue;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<NodeData, ConnectionType, NodeValue>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, NodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        // This logic is entirely up to the user. In this case, we check if the
        // current node we're drawing is the active one, by comparing against
        // the value stored in the global user state, and draw different button
        // UIs based on that.

        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        // Pressing the button will emit a custom user response to either set,
        // or clear the active node. These responses do nothing by themselves,
        // the library only makes the responses available to you after the graph
        // has been drawn. See below at the update method for an example.
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
        }

        responses
    }
}

type MyGraph = Graph<NodeData, ConnectionType, NodeValue>;
type MyEditorState =
    GraphEditorState<NodeData, ConnectionType, NodeValue, NodeTemplate, NoiseGraphState>;

#[derive(Default, Resource)]
pub struct NoiseGraph {
    // The `GraphEditorState` is the top-level object. You "register" all your
    // custom types by specifying it as its generic parameters.
    state: MyEditorState,

    user_state: NoiseGraphState,
}

impl NoiseGraph {
    pub fn get_noise_fn(&self) -> DynNoiseFn {
        self.user_state.current_noise.as_ref().map(|noise| noise.clone()).unwrap_or_else(|| DynNoiseFn::new(Checkerboard::default()))
    }
    
    fn draw(&mut self, ctx: &egui::Context) {
        let graph_response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                self.state
                    .draw_graph_editor(ui, AllNodeTemplates, &mut self.user_state)
            })
            .inner;
        for node_response in graph_response.node_responses {
            // Here, we ignore all other graph events. But you may find
            // some use for them. For example, by playing a sound when a new
            // connection is created
            if let NodeResponse::User(user_event) = node_response {
                match user_event {
                    MyResponse::SetActiveNode(node) => self.user_state.active_node = Some(node),
                    MyResponse::ClearActiveNode => self.user_state.active_node = None,
                }
            }
        }

        if let Some(node) = self.user_state.active_node {
            if self.state.graph.nodes.contains_key(node) {
                let text = match evaluate_node(&self.state.graph, node, &mut HashMap::new()) {
                    Ok(value) => {
                        let text = format!("The result is: {:?}", &value);
                        self.user_state.current_noise = value.try_to_noise_function().ok();
                        text
                    },
                    Err(err) => format!("Execution error: {}", err),
                };
                ctx.debug_painter().text(
                    egui::pos2(10.0, 35.0),
                    egui::Align2::LEFT_TOP,
                    text,
                    TextStyle::Button.resolve(&ctx.style()),
                    egui::Color32::WHITE,
                );
            } else {
                self.user_state.active_node = None;
            }
        }
    }
}

type OutputsCache = HashMap<OutputId, NodeValue>;

#[derive(Clone)]
pub struct DynNoiseFn(Arc<dyn NoiseFn<f64, 2> + Send + Sync>);

impl DynNoiseFn {
    fn new<T: NoiseFn<f64, 2> + Send + Sync + 'static>(noise: T) -> Self {
        Self(Arc::new(noise))
    }
}

impl NoiseFn<f64, 2> for DynNoiseFn
{
    fn get(&self, point: [f64; 2]) -> f64 {
        self.0.get(point)
    }
}