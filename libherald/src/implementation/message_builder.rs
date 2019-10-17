use crate::{ffi, interface::*, ret_err, ret_none};
use heraldcore::message::*;
use std::convert::TryInto;
use std::path::PathBuf;

/// Message builder, used for interactively composing messages
pub struct MessageBuilder {
    emit: Emitter,
    model: List,
    inner: OutboundMessageBuilder,
}

type Emitter = MessageBuilderEmitter;
type List = MessageBuilderList;

impl MessageBuilderTrait for MessageBuilder {
    fn new(emit: MessageBuilderEmitter, model: MessageBuilderList) -> Self {
        Self {
            emit,
            model,
            inner: OutboundMessageBuilder::default(),
        }
    }

    fn emit(&mut self) -> &mut MessageBuilderEmitter {
        &mut self.emit
    }

    fn conversation_id(&self) -> Option<ffi::ConversationIdRef> {
        Some(self.inner.conversation.as_ref()?.as_slice())
    }

    fn set_conversation_id(&mut self, cid: Option<ffi::ConversationIdRef>) {
        let cid = ret_err!(ret_none!(cid).try_into());
        self.inner.conversation_id(cid);
    }

    fn replying_to(&self) -> Option<ffi::MsgIdRef> {
        Some(self.inner.op.as_ref()?.as_slice())
    }

    fn is_reply(&self) -> bool {
        self.inner.op.is_some()
    }

    fn is_media_message(&self) -> bool {
        !self.inner.attachments.is_empty()
    }

    fn set_replying_to(&mut self, op_msg_id: Option<ffi::MsgIdRef>) {
        match (op_msg_id, self.inner.op) {
            (Some(op_msg_id), None) => {
                let op_msg_id = ret_err!(op_msg_id.try_into());
                self.inner.replying_to(Some(op_msg_id));
                self.emit.is_reply_changed();
            }
            (None, Some(_)) => {
                self.inner.replying_to(None);
                self.emit.is_reply_changed();
            }
            _ => {}
        }
    }

    fn clear_reply(&mut self) {
        self.set_replying_to(None);
    }

    fn add_attachment(&mut self, path: String) -> bool {
        let path = crate::utils::strip_qrc(path);
        let path = PathBuf::from(path);
        let len = self.inner.attachments.len();

        self.model.begin_insert_rows(len, len);
        self.inner.add_attachment(path);
        self.model.end_insert_rows();

        if len == 0 {
            self.emit.is_media_message_changed();
        }

        true
    }

    fn set_body(&mut self, body: Option<String>) {
        match body {
            Some(body) => {
                self.inner.body(ret_err!(body.try_into()));
            }
            None => {
                self.inner.body = None;
            }
        }
    }

    fn body(&self) -> Option<&str> {
        Some(self.inner.body.as_ref()?.as_str())
    }

    /// Finalizes the builder, stores and sends the message, and resets the builder.
    fn finalize(&mut self) {
        let builder = std::mem::replace(&mut self.inner, Default::default());
        self.inner.conversation = builder.conversation;

        let cid = ret_none!(builder.conversation);

        ret_err!(builder.store_and_send(move |m| {
            use crate::shared::messages::*;
            use crossbeam_channel::*;
            use heraldcore::message::StoreAndSend::*;

            match m {
                Msg(msg) => {
                    let tx = match MSG_TXS.get(&cid) {
                        Some(tx) => tx.clone(),
                        None => {
                            let (tx, rx) = unbounded();

                            MSG_TXS.insert(cid, tx);
                            MSG_RXS.insert(cid, rx);
                            ret_none!(MSG_TXS.get(&cid)).clone()
                        }
                    };

                    ret_err!(tx.send(MsgUpdate::FullMsg(msg)));
                    if let Some(mut emitter) = MSG_EMITTERS.get_mut(&cid) {
                        emitter.new_data_ready();
                    }
                }
                Error { error, line_number } => {
                    // TODO better line number usage
                    eprintln!("Error at {}", line_number);
                    ret_err!(Err(error))
                }
                Done => {
                    // TODO: send status?
                }
            }
        }));
    }

    fn remove_attachment(&mut self, path: String) -> bool {
        let path = PathBuf::from(path);
        let pos = ret_none!(
            self.inner.attachments.iter().rposition(|p| p == &path),
            false
        );

        self.model.begin_remove_rows(pos, pos);
        self.inner.attachments.remove(pos);
        self.model.end_remove_rows();

        if self.inner.attachments.is_empty() {
            self.emit.is_media_message_changed();
        }

        true
    }

    fn remove_attachment_by_index(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;

        if row_index < self.inner.attachments.len() {
            return false;
        }

        self.model.begin_remove_rows(row_index, row_index);
        self.inner.attachments.remove(row_index);
        self.model.end_remove_rows();

        if self.inner.attachments.is_empty() {
            self.emit.is_media_message_changed();
        }

        true
    }

    fn remove_last(&mut self) {
        self.model.begin_remove_rows(
            self.inner.attachments.len().saturating_sub(1),
            self.inner.attachments.len().saturating_sub(1),
        );
        self.inner.attachments.pop();
        self.model.end_remove_rows();

        if self.inner.attachments.is_empty() {
            self.emit.is_media_message_changed();
        }
    }

    fn row_count(&self) -> usize {
        self.inner.attachments.len()
    }

    fn attachment_path(&self, index: usize) -> &str {
        ret_none! {
            ret_none!(self.inner.attachments.get(index), "").to_str(),
            ""
        }
    }
}