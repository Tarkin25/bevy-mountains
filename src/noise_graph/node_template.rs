use std::borrow::Cow;

use egui_node_graph::{NodeTemplateTrait, Graph, NodeId, InputParamKind};
use noise::{RidgedMulti, Perlin};
use strum::IntoEnumIterator;

use super::{NodeData, connection_type::ConnectionType, NoiseGraphState, node_value::{NodeValue, NoiseType, Operator}};

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy, Debug, strum::EnumIter, strum::Display)]
pub enum NodeTemplate {
    Perlin,
    ScaleBias,
    RidgedMulti,
    ScalePoint,
    Number,
    Arithmetic,
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
    type ValueType = NodeValue;
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

        match self {
            NodeTemplate::Arithmetic => {
                builder
                .input_operator("operator", Operator::Add)
                .input_f64("a", 0.0)
                .input_f64("b", 0.0)
                .output_number();
            }
            NodeTemplate::Number => {
                builder.input_f64("value", 1.0)
                .output_number();
            },
            NodeTemplate::Perlin => {
                builder.output_noise();
            },
            NodeTemplate::ScaleBias => {
                builder.input_noise("source")
                .input_f64("scale", 1.0)
                .input_f64("bias", 0.0)
                .output_noise();
            },
            NodeTemplate::RidgedMulti => {
                builder.input_noise_type(NoiseType::Perlin)
                .input_usize("octaves", RidgedMulti::<Perlin>::DEFAULT_OCTAVE_COUNT)
                .input_f64("frequency", RidgedMulti::<Perlin>::DEFAULT_FREQUENCY)
                .input_f64("lacunarity", RidgedMulti::<Perlin>::DEFAULT_LACUNARITY)
                .input_f64("persistence", RidgedMulti::<Perlin>::DEFAULT_PERSISTENCE)
                .input_f64("attenuation", RidgedMulti::<Perlin>::DEFAULT_ATTENUATION)
                .output_noise();
            },
            NodeTemplate::ScalePoint => {
                builder.input_noise("source")
                .input_f64("x", 1.0)
                .input_f64("y", 1.0)
                .input_f64("z", 1.0)
                .input_f64("u", 1.0)
                .output_noise();
            }
        }
    }
}

struct NodeBuilder<'a> {
    graph: &'a mut Graph<NodeData, ConnectionType, NodeValue>,
    node_id: NodeId,
}

impl<'a> NodeBuilder<'a> {
    fn new(graph: &'a mut Graph<NodeData, ConnectionType, NodeValue>, node_id: NodeId) -> Self {
        Self { graph, node_id }
    }

    fn input_f64(&mut self, name: &str, initial: f64) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::Number,
            NodeValue::F64(initial),
            InputParamKind::ConnectionOrConstant,
            true,
        );
        self
    }

    fn input_usize(&mut self, name: &str, initial: usize) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::Number,
            NodeValue::Usize(initial),
            InputParamKind::ConnectionOrConstant,
            true,
        );
        self
    }

    fn input_noise(&mut self, name: &str) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::Noise,
            NodeValue::NoInput,
            InputParamKind::ConnectionOnly,
            true
        );
        self
    }

    fn input_noise_type(&mut self, initial: NoiseType) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            "noise type".into(),
            ConnectionType::NoiseType,
            NodeValue::NoiseType(initial),
            InputParamKind::ConstantOnly,
            true,
        );
        self
    }

    fn input_operator(&mut self, name: &str, initial: Operator) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::Number,
            NodeValue::Operator(initial),
            InputParamKind::ConstantOnly,
            true,
        );
        self
    }

    fn output_noise(&mut self) -> &mut Self {
        self.graph.add_output_param(
            self.node_id,
            "out".into(),
            ConnectionType::Noise,
        );
        self
    }

    fn output_number(&mut self) -> &mut Self {
        self.graph.add_output_param(
            self.node_id,
            "out".into(),
            ConnectionType::Number,
        );
        self
    }
}