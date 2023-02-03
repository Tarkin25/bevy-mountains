use bevy::{prelude::*, reflect::TypeUuid};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;
use serde_json::Value;

use crate::{chunk::ChunksConfig, noise_graph::NoiseGraph, pause::GameState};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(JsonAssetPlugin::<ValueWrapper>::new(&["json"]))
            .add_loading_state(
                LoadingState::new(GameState::AssetsLoading)
                    .continue_to_state(GameState::Running)
                    .with_collection::<LoadedAssets>(),
            )
            .add_state(GameState::AssetsLoading)
            .add_system_set(
                SystemSet::on_exit(GameState::AssetsLoading)
                    .with_system(insert_assets_as_resources),
            );
    }
}

#[derive(AssetCollection, Resource)]
struct LoadedAssets {
    #[asset(path = "noise_graph.json")]
    noise_graph: Handle<ValueWrapper>,
    #[asset(path = "chunks_config.json")]
    chunks_config: Handle<ValueWrapper>,
}

#[derive(TypeUuid)]
#[uuid = "268b4029-0f1f-4b67-80b5-f070c8a044f3"]
struct ValueWrapper(Value);

impl<'de> Deserialize<'de> for ValueWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self(value))
    }
}

fn insert_assets_as_resources(
    assets: Res<LoadedAssets>,
    mut commands: Commands,
    mut json_assets: ResMut<Assets<ValueWrapper>>,
) {
    let noise_graph = json_assets.remove(assets.noise_graph.clone()).unwrap();
    let noise_graph: NoiseGraph =
        serde_json::from_value(noise_graph.0).expect("Failed to parse noise graph");
    let chunks_config = json_assets.remove(assets.chunks_config.clone()).unwrap();
    let chunks_config: ChunksConfig =
        serde_json::from_value(chunks_config.0).expect("Failed to parse chunks config");

    commands.insert_resource(noise_graph);
    commands.insert_resource(chunks_config);
    commands.remove_resource::<LoadedAssets>();
}
