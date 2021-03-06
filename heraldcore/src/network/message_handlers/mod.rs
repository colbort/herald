use super::*;
use crate::types::cmessages;
use crypto_store::prelude as cstore;
use network_types::{
    cmessages::ConversationMessage,
    umessages::{self, UserMessage},
    Substance,
};
use ratchet_chat::protocol as proto;

pub(crate) fn handle_push(push: Push) -> Result<Event, HErr> {
    let ts = push.timestamp;
    let from = push.gid;
    let (substance, mut event) = w!(decode_push(push));
    if let Some(substance) = substance {
        let e2 = match substance {
            Substance::Cm { cid, msg } => w!(handle_cmessage(ts, from, cid, msg)),
            Substance::Um(um) => w!(handle_umessage(ts, from, um)),
        };
        event.merge(e2);
    }
    Ok(event)
}

fn decode_push(
    Push {
        tag,
        timestamp,
        msg,
        gid: from,
    }: Push
) -> Result<(Option<Substance>, Event), HErr> {
    let mut ev = Event::default();
    let mut substance = None;

    let uid = w!(config::id());
    let kp = w!(config::keypair());

    let proto::MsgResult {
        ack,
        forward,
        output,
        response,
    } = {
        get_crypto_conn!(store);

        let msg: proto::Msg = w!(kson::from_bytes(msg));

        let res = w!(proto::handle_incoming(&mut store, &kp, from, msg));

        w!(store.commit());

        res
    };

    if let Some(ack) = ack {
        ev.add_msg_to_device(from.did, proto::Msg::Ack(ack));
    }

    if let Some(forward) = forward {
        w!(ev.add_msg_to_self(forward))
    }

    if let Some(response) = response {
        ev.add_msg_to_user(from.uid, response);
    }

    if let Some(msg) = output {
        match kson::from_bytes(msg) {
            Ok(s) => {
                substance.replace(s);
            }
            Err(e) => {
                // not sure what to do here yet
                todo!();
            }
        }
    }

    Ok((substance, ev))
}

mod content_handlers;
use content_handlers::handle_content;

fn handle_cmessage(
    ts: Time,
    GlobalId { uid, did: _ }: GlobalId,
    cid: ConversationId,
    msg: ConversationMessage,
) -> Result<Event, HErr> {
    use ConversationMessage::*;
    let mut ev = Event::default();

    match msg {
        AddedToConvo { info } => {
            use crate::types::cmessages::AddedToConvo;

            let AddedToConvo {
                members,
                cid,
                title,
                picture,
                expiration_period,
            } = *info;

            let mut conv_builder = crate::conversation::ConversationBuilder::new();
            conv_builder
                .conversation_id(cid)
                .override_members(members)
                .expiration_period(expiration_period);

            conv_builder.title = title;

            conv_builder.picture = match picture {
                Some(bytes) => Some(w!(image_utils::update_picture_buf(&bytes))),
                None => None,
            };

            let mut db = w!(crate::db::Database::get());
            let conv = w!(conv_builder.add_db(&mut db));

            ev.notifications
                .push(Notification::NewConversation(conv.meta));
        }

        Message(content) => w!(handle_content(cid, uid, ts, &mut ev, content)),
    }

    Ok(ev)
}

fn handle_umessage(
    _: Time,
    from: GlobalId,
    msg: UserMessage,
) -> Result<Event, HErr> {
    let mut ev = Event::default();

    let GlobalId { uid, .. } = from;

    match msg {
        UserMessage::Req(cr) => {
            let umessages::UserReq { cid } = cr;
            let (user, conversation) = w!(crate::user::UserBuilder::new(uid)
                .pairwise_conversation(cid)
                .add());

            let coretypes::conversation::Conversation { meta, .. } = conversation;

            ev.notifications
                .push(Notification::NewUser(Box::new((user, meta))));

            let chain =
                w!(w!(helper::get_sigchain(&from.uid)).ok_or(HeraldError("missing user".into())));
            let valid = chain.validate();
            if valid != SigValid::Yes {
                return Err(HeraldError("bad sigchain found on server".into()));
            }
            let sig::SigChain { initial, sig_chain } = chain;

            get_crypto_conn!(lock, store);
            w!(store.start_sigchain(initial));
            for link in sig_chain {
                w!(store.extend_sigchain(uid, link));
            }
            w!(store.commit());
            drop(lock);

            w!(ev.push_cm(
                cid,
                ConversationMessage::Message(NetContent::UserReqAck(cmessages::UserReqAck(true))),
            ));
        }
    }

    Ok(ev)
}
