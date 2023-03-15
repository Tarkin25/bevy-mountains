#![allow(unused)]
use std::borrow::Cow;

use bevy_inspector_egui::egui::{Color32, Ui};
use egui_node_graph::{
    DataTypeTrait, GraphEditorState, NodeDataTrait, NodeTemplateIter, NodeTemplateTrait,
    UserResponseTrait, WidgetValueTrait,
};

struct GraphManager {}

struct MyGraph {
    editor_state: GraphEditorState<NodeData, ConnectionType, Attribute, NodeTemplate, UserState>,
    user_state: UserState,
}

struct UserState;

struct NodeData {
    evaluate_node: EvaluateNodeFunction,
}

impl NodeDataTrait for NodeData {
    type Response = UserResponse;
    type UserState = UserState;
    type DataType = ConnectionType;
    type ValueType = Attribute;

    fn bottom_ui(
        &self,
        ui: &mut bevy_inspector_egui::egui::Ui,
        node_id: egui_node_graph::NodeId,
        graph: &egui_node_graph::Graph<Self, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<egui_node_graph::NodeResponse<Self::Response, Self>>
    where
        Self::Response: egui_node_graph::UserResponseTrait,
    {
        Vec::new()
    }
}

#[derive(Default)]
struct Attribute;

impl WidgetValueTrait for Attribute {
    type Response = UserResponse;
    type UserState = UserState;
    type NodeData = NodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: egui_node_graph::NodeId,
        ui: &mut bevy_inspector_egui::egui::Ui,
        user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        Vec::new()
    }
}

#[derive(Clone)]
struct NodeTemplate {
    name: &'static str,
    build_node: BuildNodeFunction,
    evaluate_node: EvaluateNodeFunction,
}

impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = ConnectionType;
    type ValueType = Attribute;
    type UserState = UserState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> std::borrow::Cow<str> {
        Cow::from(self.name)
    }

    fn node_graph_label(&self, _user_state: &mut Self::UserState) -> String {
        self.name.into()
    }

    fn user_data(&self, user_state: &mut Self::UserState) -> Self::NodeData {
        NodeData {
            evaluate_node: self.evaluate_node,
        }
    }

    fn build_node(
        &self,
        graph: &mut egui_node_graph::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
        node_id: egui_node_graph::NodeId,
    ) {
        (self.build_node)();
    }
}

#[derive(PartialEq, Eq)]
struct ConnectionType;

impl DataTypeTrait<UserState> for ConnectionType {
    fn data_type_color(&self, user_state: &mut UserState) -> bevy_inspector_egui::egui::Color32 {
        Color32::BLUE
    }

    fn name(&self) -> std::borrow::Cow<str> {
        Cow::from("ConnectionType")
    }
}

type BuildNodeFunction = fn();

type EvaluateNodeFunction = fn();

struct NodeTemplates(Vec<NodeTemplate>);

impl NodeTemplateIter for NodeTemplates {
    type Item = NodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
struct UserResponse;

impl UserResponseTrait for UserResponse {}
