use egui_node_graph::{Graph, InputParamKind, NodeId};

use crate::noise_graph::{
    connection_type::ConnectionType,
    node_attribute::{NodeAttribute, NoiseType, Operator},
    NodeData,
};

pub struct NodeBuilder<'a> {
    graph: &'a mut Graph<NodeData, ConnectionType, NodeAttribute>,
    node_id: NodeId,
}

impl<'a> NodeBuilder<'a> {
    pub fn new(
        graph: &'a mut Graph<NodeData, ConnectionType, NodeAttribute>,
        node_id: NodeId,
    ) -> Self {
        Self { graph, node_id }
    }

    pub fn input_name(&mut self) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            "New Node".into(),
            ConnectionType::NoConnection,
            NodeAttribute::Name("New Node".into()),
            InputParamKind::ConstantOnly,
            true,
        );
        self
    }

    pub fn input_f64(&mut self, name: &str, initial: f64) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::F64,
            NodeAttribute::F64(initial),
            InputParamKind::ConnectionOrConstant,
            true,
        );
        self
    }

    pub fn input_usize(&mut self, name: &str, initial: usize) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::Usize,
            NodeAttribute::Usize(initial),
            InputParamKind::ConnectionOrConstant,
            true,
        );
        self
    }

    pub fn input_noise(&mut self, name: &str) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::Noise,
            NodeAttribute::NoInput,
            InputParamKind::ConnectionOnly,
            true,
        );
        self
    }

    pub fn input_noise_type(&mut self, initial: NoiseType) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            "noise type".into(),
            ConnectionType::NoiseType,
            NodeAttribute::NoiseType(initial),
            InputParamKind::ConstantOnly,
            true,
        );
        self
    }

    pub fn input_operator(&mut self, name: &str, initial: Operator) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::F64,
            NodeAttribute::Operator(initial),
            InputParamKind::ConstantOnly,
            true,
        );
        self
    }

    pub fn input_vec(&mut self, name: &str, template: NodeAttribute) -> &mut Self {
        self.graph.add_input_param(
            self.node_id,
            name.into(),
            ConnectionType::NoConnection,
            NodeAttribute::Vec {
                values: vec![],
                template: Box::new(template),
            },
            InputParamKind::ConstantOnly,
            true,
        );
        self
    }

    pub fn output_noise(&mut self) -> &mut Self {
        self.graph
            .add_output_param(self.node_id, "out".into(), ConnectionType::Noise);
        self
    }

    pub fn output_number(&mut self) -> &mut Self {
        self.graph
            .add_output_param(self.node_id, "out".into(), ConnectionType::F64);
        self
    }
}
