use crate::structurs::image_item::ImageItem;

#[derive(Default)]
pub struct TierRow {
    pub label: String,
    pub images: Vec<ImageItem>
}