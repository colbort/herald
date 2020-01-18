use super::{types::Data, *};
use heraldcore::conversation::settings::SettingsUpdate as CoreSettingsUpdate;
use heraldcore::types::{ConversationId, MsgId};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::collections::HashMap;

/// Conversation list updates
#[derive(Debug)]
pub enum ConvUpdate {
    Global(GlobalConvUpdate),
    Item(ConvItemUpdate),
}

/// An update to the conversations model as a whole
#[derive(Debug)]
pub enum GlobalConvUpdate {
    /// A new conversation has been added
    NewConversation(ConversationMeta),
    /// A conversation builder has been finalized
    BuilderFinished(ConversationMeta),
    /// Initial data, sent when the conversations list is constructed
    Init(Vector<Conversation>),
}

/// An update to a particular conversation
#[derive(Debug)]
pub struct ConvItemUpdate {
    pub(crate) cid: ConversationId,
    pub(crate) variant: ConvItemUpdateVariant,
}

#[derive(Debug)]
pub enum ConvItemUpdateVariant {
    /// New activity
    NewActivity,
    /// Expiration period has been changed
    ExpirationChanged(ExpirationPeriod),
    /// Conversation picture has been changed
    PictureChanged(Option<String>),
    /// Conversation title has been changed
    TitleChanged(Option<String>),
    /// The last message in the conversation has been changed
    LastChanged,
    /// Pairwise user data changed
    UserChanged,
}

impl From<(ConversationId, CoreSettingsUpdate)> for crate::Update {
    fn from((cid, update): (ConversationId, CoreSettingsUpdate)) -> Self {
        use ConvItemUpdateVariant::*;
        let update = match update {
            CoreSettingsUpdate::Expiration(period) => ConvItemUpdate {
                cid,
                variant: ExpirationChanged(period),
            },
            CoreSettingsUpdate::Title(title) => ConvItemUpdate {
                cid,
                variant: TitleChanged(title),
            },
            CoreSettingsUpdate::Picture(path) => ConvItemUpdate {
                cid,
                variant: PictureChanged(path),
            },
        };

        update.into()
    }
}

impl From<GlobalConvUpdate> for crate::Update {
    fn from(update: GlobalConvUpdate) -> crate::Update {
        crate::Update::Conv(ConvUpdate::Global(update))
    }
}

impl From<ConvItemUpdate> for crate::Update {
    fn from(update: ConvItemUpdate) -> crate::Update {
        crate::Update::Conv(ConvUpdate::Item(update))
    }
}

impl From<ConvUpdate> for crate::Update {
    fn from(update: ConvUpdate) -> crate::Update {
        crate::Update::Conv(update)
    }
}

pub(crate) fn insert_data(
    cid: ConversationId,
    data: Data,
) {
    conv_data().write().insert(cid, data);
}

pub(crate) fn title(cid: &ConversationId) -> Option<String> {
    conv_data().read().get(cid)?.title.clone()
}

pub(crate) fn picture(cid: &ConversationId) -> Option<String> {
    conv_data().read().get(cid)?.picture.clone()
}

pub(crate) fn color(cid: &ConversationId) -> Option<u32> {
    Some(conv_data().read().get(cid)?.color)
}

pub(crate) fn pairwise(cid: &ConversationId) -> Option<bool> {
    Some(conv_data().read().get(cid)?.pairwise_uid.is_some())
}

pub(crate) fn conv_data() -> &'static RwLock<HashMap<ConversationId, Data>> {
    CONV_DATA.get_or_init(|| RwLock::new(HashMap::default()))
}

pub(crate) fn last_msg_id(cid: &ConversationId) -> Option<MsgId> {
    conv_data().read().get(cid)?.last_msg_id
}

pub(crate) fn update_last_msg_id(
    cid: &ConversationId,
    msg_id: Option<MsgId>,
) -> Option<()> {
    conv_data().write().get_mut(cid)?.last_msg_id = msg_id;
    Some(())
}

static CONV_DATA: OnceCell<RwLock<HashMap<ConversationId, Data>>> = OnceCell::new();
