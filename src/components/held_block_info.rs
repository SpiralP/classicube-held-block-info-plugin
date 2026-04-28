use std::{cell::RefCell, os::raw::c_int};

use classicube_helpers::tick::TickEventHandler;
use classicube_sys::{
    Block_UNSAFE_GetName, BlockID, Chat_AddOf, Inventory_SelectedBlock,
    MsgType_MSG_TYPE_ANNOUNCEMENT, OwnedString,
};
use tracing::debug;

use crate::component::Component;

thread_local!(
    static LAST: RefCell<Option<BlockID>> = const { RefCell::new(None) };
    static TICK_HANDLER: RefCell<Option<TickEventHandler>> = const { RefCell::new(None) };
);

#[derive(Default)]
pub struct HeldBlockInfo;

impl Component for HeldBlockInfo {
    fn name(&self) -> &'static str {
        "HeldBlockInfo"
    }

    fn init(&mut self) {
        let mut handler = TickEventHandler::new();
        handler.on(|_| on_tick());
        TICK_HANDLER.with_borrow_mut(|h| *h = Some(handler));
    }

    fn free(&mut self) {
        TICK_HANDLER.with_borrow_mut(|h| *h = None);
        LAST.with_borrow_mut(|l| *l = None);
    }

    fn reset(&mut self) {
        LAST.with_borrow_mut(|l| *l = None);
    }
}

fn on_tick() {
    let current = Inventory_SelectedBlock();
    LAST.with_borrow_mut(|last| {
        if let Some(prev) = *last
            && prev != current
        {
            announce(current);
        }
        *last = Some(current);
    });
}

fn announce(id: BlockID) {
    let name = unsafe { Block_UNSAFE_GetName(id) }.to_string();
    debug!(id, %name, "held block changed");

    let mut msg = format!("{name} (#{id})");
    if msg.len() > 255 {
        msg.truncate(255);
    }
    let owned = OwnedString::new(msg);
    unsafe {
        Chat_AddOf(owned.as_cc_string(), MsgType_MSG_TYPE_ANNOUNCEMENT as c_int);
    }
}
