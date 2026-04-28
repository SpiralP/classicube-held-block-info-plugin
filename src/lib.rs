mod component;
mod components;

use classicube_sys::{Chat_Add, OwnedString};

pub fn chat_print(msg: &str) {
    let msg = &msg[..msg.len().min(255)];
    let owned = OwnedString::new(msg);
    unsafe {
        Chat_Add(owned.as_cc_string());
    }
}
