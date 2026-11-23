use crate::{Screen, structurs::tier_list::TierList};

#[derive(Default)]
pub struct App {
    screen: Screen,
    tierlist: Option<TierList>,
}
