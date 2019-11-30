use super::*;
use kdf_ratchet::Cipher;
pub(crate) use network_types::amessages::*;

#[derive(Ser, De)]
struct AuxAD {
    cid: ConversationId,
    to: UserId,
    from: GlobalId,
    gen: u32,
}

/// Seals the messages.
pub fn seal(
    // note: this is only mut because BlockStore thinks it should be
    to: UserId,
    content: &AuxMessage,
) -> Result<Cipher, HErr> {
    let cid = crate::user::by_user_id(to)?.pairwise_conversation;
    let cbytes = kson::to_vec(content).into();
    let from = config::gid()?;

    chainkeys::db::with_tx(move |tx| {
        let gen = tx.get_generation(cid, from.did)?;
        let ad = kson::to_vec(&AuxAD { cid, to, from, gen }).into();

        let (_, cipher) = tx.seal_msg(cid, from.did, ad, cbytes)?;
        Ok(cipher)
    })
}

/// Opens the message.
pub fn open(cipher: Cipher) -> Result<(ConversationId, GlobalId, ConversationMessage), HErr> {
    let AuxAD { cid, to, from, gen } = kson::from_bytes(cipher.ad.clone())?;

    if to != config::id()? {
        return Err(HeraldError(format!(
            "auxiliary message was sent by {} to {}, who is not you!",
            from.uid, to
        )));
    }

    let decrypted =
        chainkeys::open_msg(cid, from.did, gen, cipher)?.ok_or(ChainKeysError::DecryptionFailed)?;

    let parsed = kson::from_bytes(decrypted.pt.into())?;

    Ok((cid, from, parsed))
}