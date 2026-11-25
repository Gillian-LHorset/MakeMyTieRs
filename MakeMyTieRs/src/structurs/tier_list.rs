use crate::structurs::{image_id::ImageId, tier_row::TierRow};

#[derive(Default)]
pub struct TierList {
    pub name: String,
    pub rows: Vec<TierRow>,
    pub bank: Vec<ImageId>,
}
