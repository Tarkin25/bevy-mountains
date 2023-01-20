use std::borrow::Cow;

use egui_node_graph::{NodeTemplateTrait, Graph, NodeId, InputParamKind, NodeTemplateIter};

use super::{NodeData, connection_type::ConnectionType, NoiseGraphState, node::Node};

/// NodeTemplate is a mechanism to define node templates. It's what the graph
/// will display in the "new node" popup. The user code needs to tell the
/// library how to convert a NodeTemplate into a Node.
#[derive(Clone, Copy)]
pub enum NodeTemplate {
    Perlin,
    ScaleBias,
}

pub struct AllNodeTemplates;

impl NodeTemplateIter for AllNodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        // This function must return a list of node kinds, which the node finder
        // will use to display it to the user. Crates like strum can reduce the
        // boilerplate in enumerating all variants of an enum.
        vec![
            NodeTemplate::Perlin,
            NodeTemplate::ScaleBias,
        ]
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
    type ValueType = Node;
    type UserState = NoiseGraphState;

    fn node_finder_label(&self, _user_state: &mut NoiseGraphState) -> Cow<str> {
        Cow::Borrowed(match self {
            NodeTemplate::Perlin => "Perlin",
            NodeTemplate::ScaleBias => "Scale Bias",
        })
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
        let input_f64 = |graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>, name: &str| {
            graph.add_input_param(
                node_id,
                name.into(),
                ConnectionType::Constant,
                Node::F64(0.0),
                InputParamKind::ConstantOnly,
                true,
            );
        };

        let input_noise = |graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>, name: &str| {
            graph.add_input_param(
                node_id,
                name.into(),
                ConnectionType::Noise,
                Node::NoInput,
                InputParamKind::ConnectionOnly,
                true
            )
        };

        let output_noise = |graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>| {
            graph.add_output_param(
                node_id,
                "out".into(),
                ConnectionType::Noise,
            )
        };

        match self {
            NodeTemplate::Perlin => {
                output_noise(graph);
            },
            NodeTemplate::ScaleBias => {
                input_noise(graph, "source");
                input_f64(graph, "scale");
                input_f64(graph, "bias");
                output_noise(graph);
            }
        }
    }
}