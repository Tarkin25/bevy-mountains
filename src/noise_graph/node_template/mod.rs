use std::borrow::Cow;

use egui_node_graph::{Graph, NodeId, NodeTemplateTrait};
use noise::{
    Abs, Add, BasicMulti, Billow, Blend, Checkerboard, Clamp, Constant, Curve, Cylinders, Displace,
    Fbm, Perlin, RidgedMulti, ScaleBias, ScalePoint, Select, Terrace, Turbulence, Exponent, HybridMulti, Max, Min, Multiply, Negate, OpenSimplex, PerlinSurflet, Power, RotatePoint, Simplex, SuperSimplex, TranslatePoint, Value,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

mod abs;
mod add;
mod arithmetic;
mod basic_multi;
mod billow;
mod blend;
mod cache;
mod checkerboard;
mod clamp;
mod constant;
mod core;
mod curve;
mod cylinders;
mod displace;
mod exponent;
mod fbm;
mod float;
mod hybrid_multi;
mod max;
mod min;
mod multiply;
mod negate;
mod open_simplex;
mod perlin;
mod perlin_surflet;
mod power;
mod ridged_multi;
mod rotate_point;
mod scale;
mod scale_bias;
mod scale_point;
mod select;
mod simplex;
mod super_simplex;
mod terrace;
mod translate_point;
mod turbulence;
mod value;
mod worley;

pub use self::core::{evaluate_node, NodeBuilder, NodeEvaluator};
use self::{arithmetic::Arithmetic, cache::SyncCache, float::Float, worley::SyncWorley, scale::Scale};

use super::{
    connection_type::ConnectionType, node_attribute::NodeAttribute, NoiseGraph, NodeData,
    NoiseGraphState, OutputsCache,
};

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
    SuperSimplex,
    Terrace,
    TranslatePoint,
    Turbulence,
    Value,
    Worley,
}

impl NodeTemplate {
    pub fn evaluate(
        graph: &NoiseGraph,
        node_id: NodeId,
        outputs_cache: &mut OutputsCache,
    ) -> anyhow::Result<NodeAttribute> {
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
            NodeTemplate::BasicMulti => BasicMulti::build(builder),
            NodeTemplate::Billow => Billow::build(builder),
            NodeTemplate::Blend => Blend::build(builder),
            NodeTemplate::Cache => SyncCache::build(builder),
            NodeTemplate::Checkerboard => Checkerboard::build(builder),
            NodeTemplate::Clamp => Clamp::build(builder),
            NodeTemplate::Constant => Constant::build(builder),
            NodeTemplate::Curve => Curve::build(builder),
            NodeTemplate::Cylinders => Cylinders::build(builder),
            NodeTemplate::Displace => Displace::build(builder),
            NodeTemplate::Exponent => Exponent::build(builder),
            NodeTemplate::Fbm => Fbm::build(builder),
            NodeTemplate::Float => Float::build(builder),
            NodeTemplate::HybridMulti => HybridMulti::build(builder),
            NodeTemplate::Max => Max::build(builder),
            NodeTemplate::Min => Min::build(builder),
            NodeTemplate::Multiply => Multiply::build(builder),
            NodeTemplate::Negate => Negate::build(builder),
            NodeTemplate::OpenSimplex => OpenSimplex::build(builder),
            NodeTemplate::Perlin => Perlin::build(builder),
            NodeTemplate::PerlinSurflet => PerlinSurflet::build(builder),
            NodeTemplate::Power => Power::build(builder),
            NodeTemplate::RidgedMulti => RidgedMulti::build(builder),
            NodeTemplate::RotatePoint => RotatePoint::build(builder),
            NodeTemplate::Scale => Scale::build(builder),
            NodeTemplate::ScaleBias => ScaleBias::build(builder),
            NodeTemplate::ScalePoint => ScalePoint::build(builder),
            NodeTemplate::Select => Select::build(builder),
            NodeTemplate::Simplex => Simplex::build(builder),
            NodeTemplate::SuperSimplex => SuperSimplex::build(builder),
            NodeTemplate::Terrace => Terrace::build(builder),
            NodeTemplate::TranslatePoint => TranslatePoint::build(builder),
            NodeTemplate::Turbulence => Turbulence::build(builder),
            NodeTemplate::Value => Value::build(builder),
            NodeTemplate::Worley => SyncWorley::build(builder),
        }
    }
}
