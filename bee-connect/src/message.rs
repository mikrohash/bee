pub const MESSAGE_LENGTH : usize = 10;

/// A type for the payload messages that can be exchanged between bee nodes.
pub type Message = [u8; MESSAGE_LENGTH];

pub const DISCONNECT_REQUEST: Message = *b"/DISCONNEC";
pub const DISCONNECT_RESPONSE: Message = *b"+DISCONNEC";
pub const CONFUSED_RESPONSE: Message = *b"-CONFUSED ";