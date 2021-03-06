use super::*;

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushTag {
    User,
    Key,
}

#[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
pub struct Push {
    pub tag: PushTag,
    pub timestamp: Time,
    pub msg: Bytes,
    pub gid: GlobalId,
}

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushMeta {
    pub tag: PushTag,
    pub gid: GlobalId,
}

#[derive(Ser, De, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PushAck {
    Success,
    LogFailure,
    Quit,
}

pub mod catchup {
    use super::*;

    pub const CHUNK_SIZE: usize = 256;

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum Catchup {
        NewMessages,
        Done,
    }

    #[derive(Ser, De, Debug, Clone, PartialEq, Eq)]
    pub enum CatchupAck {
        Success,
        Failure,
    }
}
