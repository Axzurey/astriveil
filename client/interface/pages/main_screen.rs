use egui::{Id, Response};

use crate::interface::buttons::{ButtonStyle, styled_button};

pub fn main_screen(ctx: &mut egui::Ui, dims: (u32, u32)) {
    egui::Area::new(Id::new("Main Menu")).default_size([dims.0 as f32, dims.1 as f32]).show(ctx, |ui| {
        if play_button(ui).clicked() {  }

    });
}

fn play_button(ui: &mut egui::Ui) -> Response {
    styled_button(ui, "Servers", ButtonStyle::default())
}