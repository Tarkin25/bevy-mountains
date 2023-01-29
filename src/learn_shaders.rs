use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::{AsBindGroup, ShaderType}};

pub struct LearnShadersPlugin;

impl Plugin for LearnShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<LearnShadersMaterial>::default())
        .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<LearnShadersMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(LearnShadersMaterial::default()),
        ..Default::default()
    });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Default)]
#[uuid = "6cf55774-e3e4-4cf8-81b3-dd3641cc90de"]
pub struct LearnShadersMaterial {
    pub gradient_points: Vec<GradientPoint>,
}

impl Material for LearnShadersMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/learn_shaders.wgsl".into()
    }
}

#[derive(Clone, Copy, Debug, Default, ShaderType)]
pub struct GradientPoint {
    pub height: f32,
    pub color: Color,
}