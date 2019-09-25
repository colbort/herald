use super::*;
use crate::{prelude::*, store::*};
use futures::compat::*;
use futures::stream::{Stream, StreamExt};
use sodiumoxide::crypto::sign;
use std::collections::VecDeque;
use tokio::sync::mpsc::{
    unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender,
};
use warp::filters::ws;
use warp::{Future as WFut, Stream as WStream};

impl State {
    pub async fn login<W, E>(&self, mut ws: W) -> Result<(), Error>
    where
        W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
        Error: From<E>,
    {
        use login::*;

        let mut store = self.new_connection()?;

        let bytes = UQ::new();

        let m = ws.next().await.ok_or(LoginFailed)??;
        let g = serde_cbor::from_slice::<SignAs>(m.as_bytes())?.0;

        let res = if !store.key_is_valid(g.did)? {
            SignAsResponse::KeyDeprecated
        } else if !store.user_exists(&g.uid)? {
            SignAsResponse::MissingUID
        } else {
            SignAsResponse::Sign(bytes)
        };
        ws.send(ws::Message::binary(serde_cbor::to_vec(&res)?))
            .await?;

        let m = ws.next().await.ok_or(LoginFailed)??;
        let s = serde_cbor::from_slice::<LoginToken>(m.as_bytes())?.0;

        let res = if sign::verify_detached(&s, bytes.as_ref(), &g.did) {
            LoginTokenResponse::Success
        } else {
            LoginTokenResponse::BadSig
        };
        ws.send(ws::Message::binary(serde_cbor::to_vec(&res)?))
            .await?;

        if res == LoginTokenResponse::Success {
            let (sender, mut receiver) = channel();
            self.active.insert(g.did, sender);
            // TODO: handle this error somehow?
            // for now we're just dropping it
            if self.catchup(g.did, &mut store, &mut ws).await.is_ok() {
                // TODO: maybe handle this one too?
                // again just dropping it since the flow must go on
                drop(self.send_pushes(ws, &mut receiver).await);
            }
            self.active.remove(&g.did);
            self.archive_pushes(&mut store, receiver, g.did).await?;
        }

        Ok(())
    }

    async fn catchup<S, W, E>(
        &self,
        did: sign::PublicKey,
        s: &mut S,
        ws: &mut W,
    ) -> Result<(), Error>
    where
        S: Store,
        W: Stream<Item = Result<ws::Message, warp::Error>> + Sink<ws::Message, Error = E> + Unpin,
        Error: From<E>,
    {
        use catchup::*;
        let pending = s.get_pending(did)?;

        // TCP over TCP...
        for chunk in pending.chunks(CHUNK_SIZE) {
            // TODO: remove unnecessary memcpy here by using a draining chunk iterator?
            let msg = Catchup(Vec::from(chunk));
            loop {
                ws.send(ws::Message::binary(serde_cbor::to_vec(&msg)?))
                    .await?;

                let m = ws.next().await.ok_or(CatchupFailed)??;

                if CatchupAck(chunk.len() as u64) == serde_cbor::from_slice(m.as_bytes())? {
                    break;
                }
            }
        }

        s.expire_pending(did)?;

        Ok(())
    }

    async fn send_pushes<Tx, E, Rx>(&self, mut tx: Tx, rx: &mut Rx) -> Result<(), Error>
    where
        Tx: Sink<ws::Message, Error = E> + Unpin,
        Error: From<E>,
        Rx: Stream<Item = Push> + Unpin,
    {
        while let Some(p) = rx.next().await {
            tx.send(ws::Message::binary(serde_cbor::to_vec(&p)?))
                .await?;
        }
        Ok(())
    }

    async fn archive_pushes<S, Rx>(
        &self,
        store: &mut S,
        mut rx: Rx,
        to: sig::PublicKey,
    ) -> Result<(), Error>
    where
        S: Store,
        Rx: Stream<Item = Push> + Unpin,
    {
        while let Some(p) = rx.next().await {
            store.add_pending(vec![to], p)?;
        }
        Ok(())
    }
}
