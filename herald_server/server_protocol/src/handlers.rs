use super::*;
use futures::stream::TryStreamExt;

impl State {
    pub async fn get_sigchain(
        &self,
        of: UserId,
    ) -> Result<Option<sig::SigChain>, Error> {
        Ok(self.new_connection().await?.get_sigchain(of).await?)
    }

    pub async fn recip_exists(
        &self,
        recip: Recip,
    ) -> Result<bool, Error> {
        Ok(self.new_connection().await?.recip_exists(recip).await?)
    }

    //FIXME: interrupt deprecated sessions
    pub async fn new_sig(
        &self,
        sig: Signed<sig::SigUpdate>,
    ) -> Result<PKIResponse, Error> {
        use sig::SigUpdate;

        match sig::validate_update(&sig) {
            SigValid::Yes => {
                let mut conn = self.new_connection().await?;
                if !conn.key_is_valid(*sig.signed_by()).await? {
                    return Ok(PKIResponse::DeadKey);
                }

                let u1 = conn.user_of(*sig.signed_by()).await?;
                let u2 = match sig.data() {
                    SigUpdate::Endorse(end) => Some(*end.data()),
                    SigUpdate::Deprecate(dep) => conn.user_of(*dep).await?,
                };

                if u1.is_none() || u1 != u2 {
                    return Ok(PKIResponse::InvalidOp);
                }

                Ok(conn.add_to_sigchain(sig).await?)
            }
            s => Ok(PKIResponse::BadSig(s)),
        }
    }

    pub async fn new_prekeys(
        &self,
        keys: Vec<(Signed<Prekey>, Option<Prekey>)>,
    ) -> Result<new_prekeys::Res> {
        use new_prekeys::Res;

        for (p, _) in &keys {
            let valid = p.verify_sig();
            if valid != SigValid::Yes {
                return Ok(Res::BadSig(valid, *p.data()));
            }
        }

        Ok(self
            .new_connection()
            .await?
            .new_prekeys(stream::iter(
                keys.into_iter()
                    .map(|(new, old)| PrekeyReplace { new, old }),
            ))
            .await?)
    }

    pub async fn get_prekeys(
        &self,
        of: Vec<sig::PublicKey>,
    ) -> Result<Vec<(sig::PublicKey, Signed<Prekey>)>, Error> {
        Ok(self
            .new_connection()
            .await?
            .get_random_prekeys(stream::iter(of))
            .await?
            .into_iter()
            .map(|t| (t.key, t.prekey))
            .collect())
    }

    // FIXME: verify that user adding to group is part of the group
    pub async fn add_to_group(
        &self,
        requester: UserId,
        cid: ConversationId,
        users: Vec<UserId>,
    ) -> Result<add_to_group::Res, Error> {
        Ok(self
            .new_connection()
            .await?
            .add_to_group(requester, stream::iter(users), cid)
            .await?)
    }

    pub async fn leave_groups(
        &self,
        requester: UserId,
        convs: Vec<ConversationId>,
    ) -> Result<leave_groups::Res, Error> {
        Ok(self
            .new_connection()
            .await?
            .leave_groups(requester, stream::iter(convs))
            .await?)
    }

    // FIXME: check that a user is part of a group before they push to it
    pub async fn send_push(
        &self,
        from: GlobalId,
        to: Recip,
        msg: Bytes,
    ) -> Result<push::Res, Error> {
        let psh = Push {
            tag: to.tag(),
            timestamp: Time::now(),
            gid: from,
            msg,
        };
        match self
            .new_connection()
            .await?
            .add_to_pending_and_get_valid_devs(&to, &psh)
            .await?
        {
            PushedTo::NoRecipients => Ok(push::Res::Success),
            PushedTo::Missing(m) => Ok(push::Res::Missing(m)),
            PushedTo::PushedTo { devs, push_id } => {
                stream::iter(devs)
                    .map(Ok)
                    .try_for_each_concurrent(10, {
                        let psh = &psh;
                        move |d| {
                            async move {
                                self.active
                                    .async_get(d)
                                    .await
                                    .map(|s| s.push(psh.clone(), push_id))
                                    .transpose()
                                    .map(drop)
                            }
                        }
                    })
                    .await?;
                Ok(push::Res::Success)
            }
        }
    }
}