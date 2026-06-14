use egui::{Color32, Id, Response, TextEdit};
use nalgebra::Vector2;

use crate::interface::{buttons::{ButtonStyle, styled_button}, uictx::DimD2};

pub fn main_screen(ctx: &mut egui::Ui, dims: (u32, u32), server_addr: &mut String) -> bool {
    let mut done = false;
    ctx.horizontal(|ui| {
        server_input(ui, dims, server_addr);
        if play_button(ui, dims).clicked() { 
            done = true;
        }
    });
    done
}

fn server_input(ui: &mut egui::Ui, dims: (u32, u32), server_addr: &mut String) {
    ui.add_sized(
        [350.0, 100.0],
        TextEdit::singleline(server_addr)
            .background_color(Color32::from_hex("#afaffc").unwrap())
            .text_color(Color32::from_hex("#0d0e13").unwrap()),
    );
}

fn play_button(ui: &mut egui::Ui, dims: (u32, u32)) -> Response {
    
    styled_button(ui, "Join Server", ButtonStyle {
        font_size: 28.0,
        size: Vector2::new(350.0, 100.0),
        //position: DimD2::new(0.5, 0.5, 0., 0.).calculate_absolute(dims),
        ..Default::default()
    })
}