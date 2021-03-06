use super::*;
use crate::{
    err, ffi,
    interface::{MessagesEmitter as Emitter, MessagesList as List, MessagesTrait as Interface},
    none, spawn,
};
use heraldcore::{
    config,
    message::{self, MsgData, ReceiptStatus},
};
use messages_helper::search::SearchState;
use std::collections::HashMap;
use std::convert::TryInto;

mod attachments;
mod author;
mod body;
mod flurry;
mod op;
mod reactions;
mod receipts;
mod search;
mod time;

impl Messages {
    pub(crate) fn new_(
        emit: Emitter,
        model: List,
        builder: MessageBuilder,
    ) -> Self {
        Messages {
            model,
            emit,
            container: Container::default(),
            conversation_id: None,
            local_id: config::id().ok(),
            search: SearchState::new(),
            builder,
            elider: Default::default(),

            typing_sender: Default::default(),
        }
    }

    pub(crate) fn msg_id_(
        &self,
        index: usize,
    ) -> Option<ffi::MsgIdRef> {
        Some(self.container.msg_id(index)?.as_slice())
    }

    pub(crate) fn index_by_id_(
        &self,
        msg_id: ffi::MsgIdRef,
    ) -> i64 {
        let msg_id = err!(msg_id.try_into(), -1);

        none!(self.container.index_by_id(msg_id), -1) as i64
    }

    pub(crate) fn delete_message_(
        &mut self,
        index: u64,
    ) -> bool {
        let ix = index as usize;

        let id = none!(self.container.get(ix), false).msg_id;
        let builder = &mut self.builder;
        let emit = &mut self.emit;
        let container = &mut self.container;
        let model = &mut self.model;
        let search = &mut self.search;

        let cid = none!(self.conversation_id, false);
        container.remove_helper(id, ix, emit, model, search, builder, cid);
        spawn!(message::delete_message(&id), false);

        true
    }

    pub(crate) fn delete_message_by_id_(
        &mut self,
        id: ffi::MsgIdRef,
    ) -> bool {
        let id = err!(id.try_into(), false);
        let ix = none!(self.container.index_by_id(id), false);
        let cid = none!(self.conversation_id, false);

        let builder = &mut self.builder;
        let emit = &mut self.emit;
        let container = &mut self.container;
        let model = &mut self.model;
        let search = &mut self.search;

        container.remove_helper(id, ix, emit, model, search, builder, cid);
        spawn!(message::delete_message(&id), false);

        true
    }

    pub(crate) fn clear_conversation_history_(&mut self) -> bool {
        let id = none!(self.conversation_id, false);

        spawn!(message::delete_conversation_messages(&id), false);

        self.clear_search();
        self.model
            .begin_remove_rows(0, self.container.len().saturating_sub(1));
        self.container = Default::default();
        self.model.end_remove_rows();

        self.emit_last_changed();
        true
    }

    pub(crate) fn aux_data_(
        &self,
        index: usize,
    ) -> String {
        self.container.aux_data_json(index).unwrap_or_default()
    }

    pub(crate) fn send_typing_indicator_(&mut self) {
        use crossbeam_channel::bounded;
        let cid = none!(self.conversation_id);

        let (typing_sender, rx) = match self.typing_sender.as_ref() {
            Some(tx) => {
                let _ = tx.try_send(());
                return;
            }
            None => bounded(0),
        };

        // update the sender
        self.typing_sender.replace(typing_sender);

        spawn!({
            err!(heraldcore::network::send_typing_indicator(cid));

            loop {
                match rx.recv() {
                    Ok(_) => {
                        err!(heraldcore::network::send_typing_indicator(cid));
                        std::thread::sleep(std::time::Duration::from_secs(5));
                    }
                    Err(_) => return,
                }
            }
        });
    }

    pub(crate) fn emit_(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    pub(crate) fn row_count_(&self) -> usize {
        self.container.len()
    }

    pub(crate) fn builder_(&self) -> &MessageBuilder {
        &self.builder
    }

    pub(crate) fn builder_mut_(&mut self) -> &mut MessageBuilder {
        &mut self.builder
    }

    pub(crate) fn last_msg_digest_(&self) -> String {
        let f = || {
            let mid = &self.last_msg_id()?;

            let MsgData {
                author,
                receipts,
                send_status,
                time,
                content,
                ..
            } = messages_helper::container::get(&mid)?;

            let body = content.as_str();
            let time = time.insertion;
            let aux_code = content.aux_code();
            let has_attachments = content
                .attachments()
                .map(|a| !a.is_empty())
                .unwrap_or(false);

            let receipt_status = receipts.iter().map(|(_, r)| r).max().copied();
            let status = heraldcore::message::Status::from((send_status, receipt_status)) as u8;

            let object = json::object! {
                "author" => author.as_str(),
                "body" => body,
                "time" => *time.as_i64(),
                "auxCode" => aux_code,
                "status" => status,
                "hasAttachments" => has_attachments
            };

            Some(object.dump())
        };

        f().unwrap_or_default()
    }
}
