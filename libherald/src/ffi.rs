/// Used to indicate that the data
/// represents a [`heraldcore::types::MsgId`]
pub type MsgIdRef<'a> = &'a [u8];
/// Used to indicate that the data
/// represents a [`heraldcore::types::MsgId`]
pub type MsgId = Vec<u8>;

/// Used to indicate that the data
/// represents a [`heraldcore::types::ConversationId`]
pub type ConversationIdRef<'a> = &'a [u8];
/// Used to indicate that the data
/// represents a [`heraldcore::types::ConversationId`]
pub type ConversationId = Vec<u8>;

/// Used to indicate that the data
/// represents a [`herald_common::UserId`]
pub type UserId = String;
/// Used to indicate that the data
/// represents a [`herald_common::UserId`]
pub type UserIdRef<'a> = &'a str;