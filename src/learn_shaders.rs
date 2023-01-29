use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::{AsBindGroup, PreparedBindGroup}};

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

#[derive(TypeUuid, Debug, Clone, Default)]
#[uuid = "6cf55774-e3e4-4cf8-81b3-dd3641cc90de"]
pub struct LearnShadersMaterial {
    pub standard_material: StandardMaterial,
}

impl AsBindGroup for LearnShadersMaterial {
    type Data = <StandardMaterial as AsBindGroup>::Data;
    
    fn as_bind_group(
            &self,
            layout: &bevy::render::render_resource::BindGroupLayout,
            render_device: &bevy::render::renderer::RenderDevice,
            images: &bevy::render::render_asset::RenderAssets<Image>,
            fallback_image: &bevy::render::texture::FallbackImage,
        ) -> Result<bevy::render::render_resource::PreparedBindGroup<Self>, bevy::render::render_resource::AsBindGroupError> {
        let PreparedBindGroup { bindings, bind_group, data } = self.standard_material.as_bind_group(layout, render_device, images, fallback_image)?;
        Ok(PreparedBindGroup {
            bindings, bind_group, data
        })
    }

    fn bind_group_layout(render_device: &bevy::render::renderer::RenderDevice) -> bevy::render::render_resource::BindGroupLayout {
        StandardMaterial::bind_group_layout(render_device)
    }
}

impl Material for LearnShadersMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/learn_shaders.wgsl".into()
    }
}