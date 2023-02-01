use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            encase::StorageBuffer, AsBindGroup, AsBindGroupError, BindGroupDescriptor,
            BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
            BindingType, BufferBindingType, BufferInitDescriptor, BufferUsages,
            OwnedBindingResource, PreparedBindGroup, ShaderRef, ShaderStages, ShaderType,
        },
        renderer::RenderDevice,
        texture::FallbackImage,
    },
};
use bevy_inspector_egui::egui::{DragValue, Response, Ui, Widget};

use crate::{
    pause::GameState,
    widgets::{ColorWidget, ListWidget},
};

pub struct LearnShadersPlugin;

impl Plugin for LearnShadersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColorGradient>()
            .add_plugin(MaterialPlugin::<LearnShadersMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(update_materials))
            .add_startup_system(insert_material_config);
    }
}

fn insert_material_config(
    color_gradient: Res<ColorGradient>,
    mut materials: ResMut<Assets<LearnShadersMaterial>>,
    mut commands: Commands,
) {
    commands.insert_resource(MaterialConfig {
        chunk_material: materials.add(LearnShadersMaterial {
            gradient_points: color_gradient.gradient_points.clone(),
        }),
    });
}

fn update_materials(
    mut materials: ResMut<Assets<LearnShadersMaterial>>,
    color_gradient: Res<ColorGradient>,
    query: Query<&Handle<LearnShadersMaterial>>,
) {
    query.for_each(|handle| {
        if let Some(material) = materials.get_mut(handle) {
            if material.gradient_points != color_gradient.gradient_points {
                material.gradient_points = color_gradient.gradient_points.clone();
            }
        }
    })
}

#[derive(TypeUuid, Debug, Clone)]
#[uuid = "6cf55774-e3e4-4cf8-81b3-dd3641cc90de"]
pub struct LearnShadersMaterial {
    pub gradient_points: Vec<GradientPoint>,
}

impl AsBindGroup for LearnShadersMaterial {
    type Data = ();
    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        _images: &RenderAssets<Image>,
        _fallback_image: &FallbackImage,
    ) -> Result<PreparedBindGroup<Self>, AsBindGroupError> {
        let bindings = vec![{
            let mut buffer = StorageBuffer::new(Vec::new());
            buffer.write(&self.gradient_points).unwrap();
            OwnedBindingResource::Buffer(render_device.create_buffer_with_data(
                &BufferInitDescriptor {
                    label: None,
                    usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
                    contents: buffer.as_ref(),
                },
            ))
        }];
        let bind_group = {
            let descriptor = BindGroupDescriptor {
                entries: &[BindGroupEntry {
                    binding: 0u32,
                    resource: bindings[0usize].get_binding(),
                }],
                label: None,
                layout: &layout,
            };
            render_device.create_bind_group(&descriptor)
        };
        Ok(PreparedBindGroup {
            bindings,
            bind_group,
            data: (),
        })
    }
    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0u32,
                visibility: ShaderStages::all(),
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: Some(<Vec<GradientPoint> as ShaderType>::min_size()),
                },
                count: None,
            }],
            label: None,
        })
    }
}

impl Material for LearnShadersMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/learn_shaders.wgsl".into()
    }
}

#[derive(Clone, Copy, Debug, Default, ShaderType, PartialEq)]
pub struct GradientPoint {
    pub color: Color,
    pub height: f32,
}

impl Widget for &mut GradientPoint {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.add(DragValue::new(&mut self.height).max_decimals(5));
            ui.add(ColorWidget(&mut self.color));
        })
        .response
    }
}

#[derive(Debug, Resource)]
pub struct ColorGradient {
    pub gradient_points: Vec<GradientPoint>,
}

impl Default for ColorGradient {
    fn default() -> Self {
        Self {
            gradient_points: vec![
                GradientPoint {
                    height: -1.0,
                    color: Color::PURPLE,
                },
                GradientPoint {
                    height: 0.0,
                    color: Color::WHITE,
                },
                GradientPoint {
                    height: 1.0,
                    color: Color::CYAN,
                },
            ],
        }
    }
}

impl Widget for &mut ColorGradient {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.heading("Color Gradient");
        ui.horizontal(|ui| {
            ui.label("gradient_points");
            ui.vertical(|ui| {
                ui.add(ListWidget(&mut self.gradient_points));
                if ui.button("Sort").clicked() {
                    self.gradient_points
                        .sort_by(|a, b| a.height.total_cmp(&b.height));
                }
            });
        })
        .response
    }
}

#[derive(Default, Debug, Resource)]
pub struct MaterialConfig {
    pub chunk_material: Handle<LearnShadersMaterial>,
}
