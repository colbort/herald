use bytes::Bytes;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

pub use arrayvec::CapacityError;

pub type UserId = arrayvec::ArrayString<[u8; 256]>;
pub type DeviceId = usize;
pub type RawMsg = Bytes;

#[derive(Serialize, Deserialize, Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct User {
    num_devices: usize,
}

#[derive(Serialize, Deserialize, Hash, Debug, Copy, Clone, PartialEq, Eq)]
pub struct GlobalId {
    uid: UserId,
    did: DeviceId,
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToServer {
    // Login(GlobalId),
    Send { to: UserId, text: RawMsg },
}

#[derive(Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
pub enum MessageToClient {
    Message {
        from: UserId,
        text: RawMsg,
        time: DateTime<Utc>,
    },
}
