use std::borrow::Cow;

use egui_node_graph::{NodeTemplateTrait, Graph, NodeId};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;

mod abs;
mod add;
mod arithmetic;
mod blend;
mod core;
mod displace;
mod fbm;
mod float;
mod perlin;
mod ridged_multi;
mod scale_bias;
mod scale_point;
mod select;
mod terrace;
mod turbulence;

use self::{arithmetic::Arithmetic, float::Float, perlin::Perlin, fbm::Fbm, ridged_multi::RidgedMulti, scale_bias::ScaleBias, scale_point::ScalePoint, turbulence::Turbulence, blend::Blend, displace::Displace, add::Add, select::Select, terrace::Terrace, abs::Abs};
pub use self::core::{NodeBuilder, NodeEvaluator, evaluate_node};

use super::{NodeData, connection_type::ConnectionType, NoiseGraphState, node_attribute::NodeAttribute, MyGraph, OutputsCache};

pub trait NodeImpl {
    fn build(builder: &mut NodeBuilder);

    fn evaluate(evaluator: &mut NodeEvaluator) -> anyhow::Result<NodeAttribute>;
}

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy, Debug, strum::EnumIter, strum::Display, Serialize, Deserialize)]
pub enum NodeTemplate {
    Abs,
    Add,
    Arithmetic,
    Blend,
    Displace,
    Fbm,
    Float,
    Perlin,
    RidgedMulti,
    ScaleBias,
    ScalePoint,
    Select,
    Terrace,
    Turbulence,
}

impl NodeTemplate {
    pub fn evaluate(graph: &MyGraph, node_id: NodeId, outputs_cache: &mut OutputsCache) -> anyhow::Result<NodeAttribute> {
        evaluate_node(graph, node_id, outputs_cache)
    }
}

pub struct AllNodeTemplates;

impl egui_node_graph::NodeTemplateIter for AllNodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        NodeTemplate::iter().collect()
    }
}

impl Default for NodeTemplate {
    fn default() -> Self {
        Self::Perlin
    }
}

// A trait for the node kinds, which tells the library how to build new nodes
// from the templates in the node finder
impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = ConnectionType;
    type ValueType = NodeAttribute;
    type UserState = NoiseGraphState;

    fn node_finder_label(&self, _user_state: &mut NoiseGraphState) -> Cow<str> {
        Cow::Owned(format!("{:?}", self))
    }

    fn node_graph_label(&self, user_state: &mut NoiseGraphState) -> String {
        // It's okay to delegate this to node_finder_label if you don't want to
        // show different names in the node finder and the node itself.
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut NoiseGraphState) -> Self::NodeData {        
        NodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {        
        // The nodes are created empty by default. This function needs to take
        // care of creating the desired inputs and outputs based on the template
        
        let mut builder = NodeBuilder::new(graph, node_id);
        // Add a "name" attribute to all nodes
        builder.input_name();
        let builder = &mut builder;

        match self {
            NodeTemplate::Abs => Abs::build(builder),
            NodeTemplate::Add => Add::build(builder),
            NodeTemplate::Arithmetic => Arithmetic::build(builder),
            NodeTemplate::Blend => Blend::build(builder),
            NodeTemplate::Displace => Displace::build(builder),
            NodeTemplate::Fbm => Fbm::build(builder),
            NodeTemplate::Float => Float::build(builder),
            NodeTemplate::Perlin => Perlin::build(builder),
            NodeTemplate::RidgedMulti => RidgedMulti::build(builder),
            NodeTemplate::ScaleBias => ScaleBias::build(builder),
            NodeTemplate::ScalePoint => ScalePoint::build(builder),
            NodeTemplate::Select => Select::build(builder),
            NodeTemplate::Terrace => Terrace::build(builder),
            NodeTemplate::Turbulence => Turbulence::build(builder),
        }
    }
}