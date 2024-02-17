pub const MESSAGE_PROTOCOL_LINE: &'static str = "Chatroom-rs Message Packet";
pub const MESSAGE_USERNAME_PREFIX: &'static str = "Username: ";
pub const MESSAGE_ROOM_PREFIX: &'static str = "Room: ";
pub const MESSAGE_LENGTH_PREFIX: &'static str = "Content-Length: ";
pub const MESSAGE_CONTENT_PREFIX: &'static str = "Content: ";

#[derive(Clone)]
pub struct MessagePacket {
    pub username: String,
    pub room: String,
    pub content: String,
}