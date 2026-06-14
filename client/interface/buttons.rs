use egui::{Color32, Rect};
use nalgebra::Vector2;

pub struct ButtonColors {
    pub hover: Color32,
    pub background: Color32,
    pub text: Color32,
    pub click: Color32
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            background: Color32::from_hex("#afaffc").unwrap(),
            hover: Color32::from_hex("#9897e2").unwrap(),
            click: Color32::from_hex("#7164e4").unwrap(),
            text: Color32::from_hex("#0d0e13").unwrap()
        }
    }
}

pub struct ButtonStyle {
    pub colors: ButtonColors,
    pub size: Vector2<f32>,
    pub position: Vector2<f32>,
    pub font_size: f32
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            size: Vector2::new(150., 65.),
            colors: ButtonColors::default(),
            position: Vector2::new(0., 0.),
            font_size: 16.0
        }
    }
}

pub fn styled_button(ui: &mut egui::Ui, label: &str, style: ButtonStyle) -> egui::Response {

    let rect = Rect::from_min_max(
        egui::pos2(style.position.x, style.position.y), 
        egui::pos2(style.position.x + style.size.x, style.position.y + style.size.y)
    );

    let response = ui.allocate_rect(rect, egui::Sense::CLICK | egui::Sense::HOVER);
    
    let color = if response.is_pointer_button_down_on() {
        style.colors.click
    } else if response.hovered() {
        style.colors.hover
    } else {
        style.colors.background
    };

    ui.painter().rect_filled(rect, 6.0, color);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(style.font_size),
        style.colors.text,
    );

    response
}