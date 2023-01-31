use egui_node_graph::NodeId;
use noise::{
    Abs, Add, BasicMulti, Billow, Blend, Checkerboard, Clamp, Constant, Curve, Cylinders, Displace,
    Exponent, Fbm, HybridMulti, Max, Min, Multiply, Negate, NoiseFn, OpenSimplex, Perlin,
    PerlinSurflet, Power, RidgedMulti, RotatePoint, ScaleBias, ScalePoint, Select, Simplex,
    SuperSimplex, Terrace, TranslatePoint, Turbulence, Value,
};

use crate::noise_graph::{
    node_attribute::{NodeAttribute, NoiseType, Operator, WorleyReturnType},
    node_template::{
        arithmetic::Arithmetic, cache::SyncCache, float::Float, scale::Scale, worley::SyncWorley,
        NodeImpl, NodeTemplate,
    },
    DynNoiseFn, NoiseGraph, OutputsCache,
};

/// Recursively evaluates all dependencies of this node, then evaluates the node itself.
pub fn evaluate_node(
    graph: &NoiseGraph,
    node_id: NodeId,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<NodeAttribute> {
    let node = &graph[node_id];
    let evaluator = &mut NodeEvaluator::new(graph, outputs_cache, node_id);
    match node.user_data.template {
        NodeTemplate::Abs => Abs::evaluate(evaluator),
        NodeTemplate::Add => Add::evaluate(evaluator),
        NodeTemplate::Arithmetic => Arithmetic::evaluate(evaluator),
        NodeTemplate::BasicMulti => BasicMulti::evaluate(evaluator),
        NodeTemplate::Billow => Billow::evaluate(evaluator),
        NodeTemplate::Blend => Blend::evaluate(evaluator),
        NodeTemplate::Cache => SyncCache::evaluate(evaluator),
        NodeTemplate::Checkerboard => Checkerboard::evaluate(evaluator),
        NodeTemplate::Clamp => Clamp::evaluate(evaluator),
        NodeTemplate::Constant => Constant::evaluate(evaluator),
        NodeTemplate::Curve => Curve::evaluate(evaluator),
        NodeTemplate::Cylinders => Cylinders::evaluate(evaluator),
        NodeTemplate::Displace => Displace::evaluate(evaluator),
        NodeTemplate::Exponent => Exponent::evaluate(evaluator),
        NodeTemplate::Fbm => Fbm::evaluate(evaluator),
        NodeTemplate::Float => Float::evaluate(evaluator),
        NodeTemplate::HybridMulti => HybridMulti::evaluate(evaluator),
        NodeTemplate::Max => Max::evaluate(evaluator),
        NodeTemplate::Min => Min::evaluate(evaluator),
        NodeTemplate::Multiply => Multiply::evaluate(evaluator),
        NodeTemplate::Negate => Negate::evaluate(evaluator),
        NodeTemplate::OpenSimplex => OpenSimplex::evaluate(evaluator),
        NodeTemplate::Perlin => Perlin::evaluate(evaluator),
        NodeTemplate::PerlinSurflet => PerlinSurflet::evaluate(evaluator),
        NodeTemplate::Power => Power::evaluate(evaluator),
        NodeTemplate::RidgedMulti => RidgedMulti::evaluate(evaluator),
        NodeTemplate::RotatePoint => RotatePoint::evaluate(evaluator),
        NodeTemplate::Scale => Scale::evaluate(evaluator),
        NodeTemplate::ScaleBias => ScaleBias::evaluate(evaluator),
        NodeTemplate::ScalePoint => ScalePoint::evaluate(evaluator),
        NodeTemplate::Select => Select::evaluate(evaluator),
        NodeTemplate::Simplex => Simplex::evaluate(evaluator),
        NodeTemplate::SuperSimplex => SuperSimplex::evaluate(evaluator),
        NodeTemplate::Terrace => Terrace::evaluate(evaluator),
        NodeTemplate::TranslatePoint => TranslatePoint::evaluate(evaluator),
        NodeTemplate::Turbulence => Turbulence::evaluate(evaluator),
        NodeTemplate::Value => Value::evaluate(evaluator),
        NodeTemplate::Worley => SyncWorley::evaluate(evaluator),
    }
}

pub struct NodeEvaluator<'a> {
    graph: &'a NoiseGraph,
    outputs_cache: &'a mut OutputsCache,
    node_id: NodeId,
}
impl<'a> NodeEvaluator<'a> {
    pub fn new(
        graph: &'a NoiseGraph,
        outputs_cache: &'a mut OutputsCache,
        node_id: NodeId,
    ) -> Self {
        Self {
            graph,
            outputs_cache,
            node_id,
        }
    }
    fn evaluate_input(&mut self, name: &str) -> anyhow::Result<NodeAttribute> {
        // Calling `evaluate_input` recursively evaluates other nodes in the
        // graph until the input value for a paramater has been computed.
        evaluate_input(self.graph, self.node_id, name, self.outputs_cache)
    }
    fn populate_output(
        &mut self,
        name: &str,
        value: NodeAttribute,
    ) -> anyhow::Result<NodeAttribute> {
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
    pub fn get_f64(&mut self, name: &str) -> anyhow::Result<f64> {
        self.evaluate_input(name)?.try_to_f64()
    }
    pub fn get_usize(&mut self, name: &str) -> anyhow::Result<usize> {
        self.evaluate_input(name)?.try_to_usize()
    }
    pub fn get_noise_function(&mut self, name: &str) -> anyhow::Result<DynNoiseFn> {
        self.evaluate_input(name)?.try_to_noise_function()
    }
    pub fn get_noise_type(&mut self) -> anyhow::Result<NoiseType> {
        self.evaluate_input("noise type")?.try_to_noise_type()
    }
    pub fn get_operator(&mut self, name: &str) -> anyhow::Result<Operator> {
        self.evaluate_input(name)?.try_to_operator()
    }
    pub fn get_vec(&mut self, name: &str) -> anyhow::Result<Vec<NodeAttribute>> {
        self.evaluate_input(name)?.try_to_vec()
    }
    pub fn get_return_type(&mut self) -> anyhow::Result<WorleyReturnType> {
        self.evaluate_input("return type")?.try_to_return_type()
    }
    pub fn output_noise(
        &mut self,
        noise: impl NoiseFn<f64, 2> + Send + Sync + 'static,
    ) -> anyhow::Result<NodeAttribute> {
        self.populate_output("out", NodeAttribute::NoiseFunction(DynNoiseFn::new(noise)))
    }
    pub fn output_number(&mut self, value: f64) -> anyhow::Result<NodeAttribute> {
        self.populate_output("out", NodeAttribute::F64(value))
    }
}

fn populate_output(
    graph: &NoiseGraph,
    outputs_cache: &mut OutputsCache,
    node_id: NodeId,
    param_name: &str,
    value: NodeAttribute,
) -> anyhow::Result<NodeAttribute> {
    let output_id = graph[node_id].get_output(param_name)?;
    outputs_cache.insert(output_id, value.clone());
    Ok(value)
}

// Evaluates the input value of
fn evaluate_input(
    graph: &NoiseGraph,
    node_id: NodeId,
    param_name: &str,
    outputs_cache: &mut OutputsCache,
) -> anyhow::Result<NodeAttribute> {
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
                .expect("Cache should be populated")
                .clone())
        }
    }
    // No existing connection, take the inline value instead.
    else {
        Ok(graph[input_id].value.clone())
    }
}
