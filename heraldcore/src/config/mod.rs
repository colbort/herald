use crate::{db::Database, errors::*, types::*};
use herald_common::*;
use rusqlite::{params, NO_PARAMS};

/// Default name for the "Note to Self" conversation
pub static NTS_CONVERSATION_NAME: &str = "Note to Self";

mod db;

/// User configuration
#[derive(Clone)]
pub struct Config {
    /// ID of the local user
    pub id: UserId,
    /// Colorscheme
    pub colorscheme: u32,
    /// Name of the local user
    pub name: String,
    /// Profile picture of the local user
    pub profile_picture: Option<String>,
    /// Color of the local user
    pub color: u32,
    /// The *Note to Self* conversation id.
    pub nts_conversation: ConversationId,
    keypair: sig::KeyPair,
}

/// Builder for `Config`
pub struct ConfigBuilder {
    /// ID of the local user
    id: UserId,
    keypair: sig::KeyPair,
    /// Colorscheme
    colorscheme: Option<u32>,
    /// Name of the local user
    name: Option<String>,
    /// Profile picture of the local user
    profile_picture: Option<String>,
    /// Color of the local user
    color: Option<u32>,
    nts_conversation: Option<ConversationId>,
}

impl ConfigBuilder {
    /// Creates new `ConfigBuilder`
    pub fn new(id: UserId, keypair: sig::KeyPair) -> Self {
        Self {
            id,
            keypair,
            name: None,
            color: None,
            colorscheme: None,
            profile_picture: None,
            nts_conversation: None,
        }
    }

    /// Sets colorscheme, defaults to 0 if not set.
    pub fn colorscheme(mut self, colorscheme: u32) -> Self {
        self.colorscheme = Some(colorscheme);
        self
    }

    /// Sets color, computed from hash of the UserId if not set.
    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set profile picture.
    pub fn profile_picture(mut self, profile_picture: String) -> Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    /// Sets conversation id for "Note to Self" conversation, a new conversation is created
    /// if this is not set.
    pub fn nts_conversation(mut self, conv_id: ConversationId) -> Self {
        self.nts_conversation = Some(conv_id);
        self
    }

    /// Adds configuration.
    pub fn add(self) -> Result<Config, HErr> {
        let db = Database::get()?;
        db::add(db, self)
    }
}

impl Config {
    /// Gets the user's configuration
    pub fn get() -> Result<Config, HErr> {
        let db = Database::get()?;
        db::get(db)
    }

    /// Gets user id
    pub fn id(&self) -> UserId {
        self.id
    }

    /// Gets user id directly from database.
    pub fn static_id() -> Result<UserId, HErr> {
        let db = Database::get()?;
        db::static_id(&db)
    }

    /// Gets the current user's kepair directly from the database.
    pub fn static_keypair() -> Result<sig::KeyPair, HErr> {
        let db = Database::get()?;
        db::static_keypair(&db)
    }

    /// Gets the current user's GlobalId
    pub fn static_gid() -> Result<GlobalId, HErr> {
        let db = Database::get()?;
        db::static_gid(&db)
    }

    /// Updates user's display name
    pub fn set_name(&mut self, name: String) -> Result<(), HErr> {
        crate::contact::set_name(self.id, name.as_str())?;

        self.name = name;
        Ok(())
    }

    /// Updates user's profile picture
    pub fn set_profile_picture(&mut self, profile_picture: Option<String>) -> Result<(), HErr> {
        let path = crate::contact::set_profile_picture(
            self.id,
            profile_picture,
            self.profile_picture.as_ref().map(|s| s.as_str()),
        )?;

        self.profile_picture = path;

        Ok(())
    }

    /// Update user's color
    pub fn set_color(&mut self, color: u32) -> Result<(), HErr> {
        crate::contact::set_color(self.id, color)?;
        self.color = color;

        Ok(())
    }

    /// Update user's colorscheme
    pub fn set_colorscheme(&mut self, colorscheme: u32) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/update_colorscheme.sql"), &[colorscheme])?;

        self.colorscheme = colorscheme;

        Ok(())
    }
}

#[cfg(test)]
pub(crate) fn test_config() -> crate::config::Config {
    let db = Database::get().expect("failed to get database");
    db::test_config(db)
}

#[cfg(test)]
mod tests;
