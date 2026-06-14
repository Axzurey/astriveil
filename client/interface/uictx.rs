use egui::{FontData, FontDefinitions, FontFamily};
use nalgebra::Vector2;

#[derive(Default)]
pub struct DimD2 {
    pub scale_x: f32,
    pub scale_y: f32,
    pub offset_x: f32,
    pub offset_y: f32
}

impl DimD2 {
    pub fn new(scale_x: f32, scale_y: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {
            scale_x, scale_y, offset_x, offset_y
        }
    }

    pub fn from_scale(scale_x: f32, scale_y: f32) -> Self {
        Self {
            scale_x, scale_y, ..Default::default()
        }
    }

    pub fn from_offset(offset_x: f32, offset_y: f32) -> Self {
        Self {
            offset_x, offset_y, ..Default::default()
        }
    }

    pub fn calculate_absolute(&self, reference: (u32, u32)) -> Vector2<f32> {
        Vector2::new(self.scale_x * reference.0 as f32 + self.offset_x, self.scale_y * reference.1 as f32 + self.offset_y)
    }
}

pub fn load_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert("exo2".to_owned(),
    std::sync::Arc::new(FontData::from_static(include_bytes!("../../res/client/fonts/exo2.ttf")))
    );

    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "exo2".to_owned());

    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("exo2".to_owned());

    ctx.set_fonts(fonts);
}