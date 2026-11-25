use egui::Color32;

use crate::structurs::image_id::ImageId;

#[derive(Default)]
pub struct TierRow {
    pub label: String,
    pub images: Vec<ImageId>,
    pub color: Color32,
}
