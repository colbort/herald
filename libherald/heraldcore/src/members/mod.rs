use crate::{db::Database, errors::HErr, types::*};
use herald_common::UserId;
use rusqlite::params;

/// Add a user with `member_id` to the conversation with `conversation_id`.
pub fn add_member(conversation_id: &ConversationId, member_id: UserId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/members/add_member.sql"),
        params![conversation_id, member_id],
    )?;
    Ok(())
}

/// Remove a user with `member_id` to the conversation with `conversation_id`.
pub fn remove_member(conversation_id: &ConversationId, member_id: UserId) -> Result<(), HErr> {
    let db = Database::get()?;
    db.execute(
        include_str!("../sql/members/remove_member.sql"),
        params![conversation_id, member_id],
    )?;
    Ok(())
}

/// Gets the members of a conversation.
pub fn members(conversation_id: &ConversationId) -> Result<Vec<UserId>, HErr> {
    let db = Database::get()?;
    let mut stmt = db.prepare(include_str!("../sql/members/get_conversation_members.sql"))?;
    let res = stmt.query_map(params![conversation_id], |row| row.get(0))?;

    let mut members = Vec::new();
    for member in res {
        members.push(member?);
    }

    Ok(members)
}

#[cfg(test)]
mod tests;
