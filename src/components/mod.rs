pub mod held_block_info;
pub mod logger;

use crate::{
    component::Component,
    components::{held_block_info::HeldBlockInfo, logger::Logger},
};

pub fn init_components() -> Vec<Box<dyn Component>> {
    vec![Box::new(Logger), Box::new(HeldBlockInfo)]
}
