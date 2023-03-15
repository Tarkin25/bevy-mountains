use std::borrow::Cow;

use egui_node_graph::{NodeId, NodeTemplateIter, NodeTemplateTrait};

use super::{
    BuildNodeFunction, ConnectionType, EvaluateNodeFunction, GraphId, NodeAttribute, NodeData,
    NoiseGraph, UserState,
};

#[derive(Clone)]
pub struct NodeTemplate {
    pub kind: NodeTemplateKind,
    pub build_node: BuildNodeFunction,
    pub evaluate_node: EvaluateNodeFunction,
    pub next_graph: GraphId,
}

#[derive(Clone, Copy, strum::Display)]
pub enum NodeTemplateKind {
    Abs,
    Add,
    Arithmetic,
    BasicMulti,
    Billow,
    Blend,
    Cache,
    Checkerboard,
    Clamp,
    Constant,
    Curve,
    Cylinders,
    Displace,
    Exponent,
    Fbm,
    Float,
    HybridMulti,
    Max,
    Min,
    Multiply,
    Negate,
    OpenSimplex,
    Perlin,
    PerlinSurflet,
    Power,
    RidgedMulti,
    RotatePoint,
    Scale,
    ScaleBias,
    ScalePoint,
    Select,
    Simplex,
    SubGraph,
    SuperSimplex,
    Terrace,
    TranslatePoint,
    Turbulence,
    Value,
    Worley,
}

pub struct NodeBuilder<'a> {
    graph: &'a mut NoiseGraph,
    user_state: &'a mut UserState,
    node_id: NodeId,
}

pub struct NodeTemplates(Vec<NodeTemplate>);

impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = ConnectionType;
    type ValueType = NodeAttribute;
    type UserState = UserState;

    fn node_finder_label(&self, user_state: &mut UserState) -> Cow<str> {
        Cow::from(self.kind.to_string())
    }

    fn node_graph_label(&self, user_state: &mut UserState) -> String {
        self.kind.to_string()
    }

    fn user_data(&self, user_state: &mut UserState) -> NodeData {
        NodeData {
            evaluate_node: self.evaluate_node,
        }
    }

    fn build_node(&self, graph: &mut NoiseGraph, user_state: &mut UserState, node_id: NodeId) {
        (self.build_node)(NodeBuilder::new(graph, user_state, node_id));
    }
}

impl<'a> NodeBuilder<'a> {
    fn new(graph: &'a mut NoiseGraph, user_state: &'a mut UserState, node_id: NodeId) -> Self {
        Self {
            graph,
            user_state,
            node_id,
        }
    }
}

impl NodeTemplateIter for NodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0.clone()
    }
}
