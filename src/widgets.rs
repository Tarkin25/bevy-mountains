use bevy::prelude::Color;
use bevy_inspector_egui::egui::{Widget, Color32};

pub struct ColorWidget<'a>(pub &'a mut Color);

impl<'a> Widget for ColorWidget<'a> {
    fn ui(self, ui: &mut bevy_inspector_egui::egui::Ui) -> bevy_inspector_egui::egui::Response {
        let rgba: [f32; 4] = (*self.0).into();
        let [r, g, b, a] = rgba.map(|v| (v * u8::MAX as f32) as u8);
        let mut color = Color32::from_rgba_premultiplied(r, g, b, a);
        let response = ui.color_edit_button_srgba(&mut color);
        *self.0 = color.to_array().map(|v| v as f32 / (u8::MAX as f32)).into();
        response
    }
}

pub struct ListWidget<'a, T>(pub &'a mut Vec<T>);

impl<'a, T> Widget for ListWidget<'a, T>
where
    T: Default,
    for<'r> &'r mut T: Widget
{
    fn ui(self, ui: &mut bevy_inspector_egui::egui::Ui) -> bevy_inspector_egui::egui::Response {
        ui.vertical(|ui| {
            let list = self.0;
            let mut indices_to_remove = Vec::with_capacity(list.len());

            for i in 0..list.len() {
                ui.horizontal(|ui| {
                    ui.add(&mut list[i]);
                    if ui.button("X").clicked() {
                        indices_to_remove.push(i);
                    }
                });
            }

            for index in indices_to_remove {
                list.remove(index);
            }

            if ui.button("+").clicked() {
                list.push(T::default());
            }
        }).response
    }
}