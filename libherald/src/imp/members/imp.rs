use super::*;
use heraldcore::errors::HErr;

impl Members {
    pub(crate) fn set_conversation_id(
        &mut self,
        id: ConversationId,
    ) -> Result<(), HErr> {
        shared::EMITTERS.insert(id, self.emit().clone());
        let list = user::conversation_members(&id)?;

        self.model
            .begin_insert_rows(0, list.len().saturating_sub(1));
        self.list = list
            .into_iter()
            .map(|u| {
                let id = u.id;
                User { id, matched: true }
            })
            .collect();
        self.model.end_insert_rows();

        Ok(())
    }
}
