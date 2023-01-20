use std::{collections::HashMap, fmt::Debug, sync::Arc};

use bevy::prelude::*;
use bevy_egui::{egui::{self, TextStyle}, EguiContext};
use egui_node_graph::{NodeId, UserResponseTrait, NodeDataTrait, Graph, GraphEditorState, NodeResponse, OutputId};
use noise::{NoiseFn, Perlin, ScaleBias, Checkerboard};

use crate::pause::GameState;

use self::{connection_type::ConnectionType, node_template::{NodeTemplate, AllNodeTemplates}, node::Node};

mod connection_type;
mod node_template;
mod node;

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
    type ValueType = Node;

    // This method will be called when drawing each node. This allows adding
    // extra ui elements inside the nodes. In this case, we create an "active"
    // button which introduces the concept of having an active node in the
    // graph. This is done entirely from user code with no modifications to the
    // node graph library.
    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<NodeData, ConnectionType, Node>,
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

type MyGraph = Graph<NodeData, ConnectionType, Node>;
type MyEditorState =
    GraphEditorState<NodeData, ConnectionType, Node, NodeTemplate, NoiseGraphState>;

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

type OutputsCache = HashMap<OutputId, Node>;

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
    graph: &MyGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<Node> {
    // To solve a similar problem as creating node types above, we define an
    // Evaluator as a convenience. It may be overkill for this small example,
    // but something like this makes the code much more readable when the
    // number of nodes starts growing.

    struct Evaluator<'a> {
        graph: &'a MyGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    }
    impl<'a> Evaluator<'a> {
        fn new(graph: &'a MyGraph, outputs_cache: &'a mut OutputsCache, node_id: NodeId) -> Self {
            Self {
                graph,
                outputs_cache,
                node_id,
            }
        }
        fn evaluate_input(&mut self, name: &str) -> anyhow::Result<Node> {
            // Calling `evaluate_input` recursively evaluates other nodes in the
            // graph until the input value for a paramater has been computed.
            evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
        }
        fn populate_output(
            &mut self,
            name: &str,
            value: Node,
        ) -> anyhow::Result<Node> {
            // After computing an output, we don't just return it, but we also
            // populate the outputs cache with it. This ensures the evaluation
            // only ever computes an output once.
            //
            // The return value of the function is the "final" output of the
            // node, the thing we want to get from the evaluation. The example
            // would be slightly more contrived when we had multiple output
            // values, as we would need to choose which of the outputs is the
            // one we want to return. Other outputs could be used as
            // intermediate values.
            //
            // Note that this is just one possible semantic interpretation of
            // the graphs, you can come up with your own evaluation semantics!
            populate_output(self.graph, self.outputs_cache, self.node_id, name, value)
        }
        fn input_f64(&mut self, name: &str) -> anyhow::Result<f64> {
            self.evaluate_input(name)?.try_to_f64()
        }
        fn input_noise_function(&mut self, name: &str) -> anyhow::Result<DynNoiseFn> {
            self.evaluate_input(name)?.try_to_noise_function()
        }
        fn output(&mut self, noise: impl NoiseFn<f64, 2> + Send + Sync + 'static) -> anyhow::Result<Node> {
            self.populate_output("out", Node::NoiseFunction(DynNoiseFn::new(noise)))
        }
    }

    let node = &graph[node_id];
    let mut evaluator = Evaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        NodeTemplate::Perlin => {
            evaluator.output(Perlin::default())
        }
        NodeTemplate::ScaleBias => {
            let scale = evaluator.input_f64("scale")?;
            let bias = evaluator.input_f64("bias")?;
            let source = evaluator.input_noise_function("source")?;
            let noise = ScaleBias::new(source.clone()).set_scale(scale).set_bias(bias);
            evaluator.output(noise)
        }
    }
}

fn populate_output(
    graph: &MyGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: Node,
) -> anyhow::Result<Node> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &MyGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<Node> {
    let input_id = graph[node_id].get_input(param_name)?;

    // The output of another node is connected.
    if let Some(other_output_id) = graph.connection(input_id) {
        // The value was already computed due to the evaluation of some other
        // node. We simply return value from the cache.
        if let Some(other_value) = outputs_cache.get(&other_output_id) {
            Ok(other_value.clone())
        }
        // This is the first time encountering this node, so we need to
        // recursively evaluate it.
        else {
            // Calling this will populate the cache
            evaluate_node(graph, graph[other_output_id].node, outputs_cache)?;

            // Now that we know the value is cached, return it
            Ok(outputs_cache
                .get(&other_output_id)
                .expect("Cache should be populated").clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}

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