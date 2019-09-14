use crate::{
    db::{DBTable, Database},
    errors::HErr,
    image_utils,
};
use herald_common::{ConversationId, UserId, UserIdRef};
use rusqlite::{params, NO_PARAMS};
use std::convert::TryInto;

#[derive(Default)]
/// Wrapper around contacts table.
/// TODO This will be stateful when we have caching logic.
pub struct ContactsHandle {
    db: Database,
}

impl DBTable for ContactsHandle {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/create_table.sql"), NO_PARAMS)?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;

        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/table_exists.sql"))?;

        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(include_str!("sql/contact/drop_table.sql"), NO_PARAMS)?;
        tx.execute(include_str!("sql/contact/create_table.sql"), NO_PARAMS)?;
        tx.commit()?;
        Ok(())
    }
}

/// Gets a contact's name by their `id`.
fn name(db: &Database, id: UserIdRef) -> Result<Option<String>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

    Ok(stmt.query_row(&[id], |row| row.get(0))?)
}

/// Change name of contact by their `id`
fn set_name(db: &Database, id: UserIdRef, name: Option<&str>) -> Result<(), HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

    stmt.execute(params![name, id])?;
    Ok(())
}

/// Gets a contact's profile picture by their `id`.
fn profile_picture(db: &Database, id: UserIdRef) -> Result<Option<String>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/get_profile_picture.sql"))?;

    Ok(stmt.query_row(&[id], |row| row.get(0))?)
}

/// Updates a contact's profile picture.
fn set_profile_picture(
    db: &Database,
    id: UserIdRef,
    profile_picture: Option<String>,
    old_path: Option<&str>,
) -> Result<Option<String>, HErr> {
    let profile_picture = match profile_picture {
        Some(path) => {
            let path_string =
                image_utils::save_profile_picture(id, path, old_path.map(|p| p.into()))?
                    .into_os_string()
                    .into_string()?;
            Some(path_string)
        }
        None => None,
    };

    db.execute(
        include_str!("sql/contact/update_profile_picture.sql"),
        params![profile_picture, id],
    )?;
    Ok(profile_picture)
}

/// Sets a contact's color
fn set_color(db: &Database, id: UserIdRef, color: u32) -> Result<(), HErr> {
    db.execute(
        include_str!("sql/contact/update_color.sql"),
        params![id, color],
    )?;
    Ok(())
}

/// Indicates whether contact exists
fn contact_exists(db: &Database, id: UserIdRef) -> Result<bool, HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/contact_exists.sql"))?;
    Ok(stmt.exists(&[id])?)
}

/// Sets contact status
fn set_status(db: &Database, id: UserIdRef, status: ContactStatus) -> Result<(), HErr> {
    db.execute(
        include_str!("sql/contact/set_status.sql"),
        params![status, id],
    )?;
    Ok(())
}

/// Gets contact status
fn status(db: &Database, id: UserIdRef) -> Result<ContactStatus, HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/get_status.sql"))?;

    Ok(stmt.query_row(&[id], |row| row.get(0))?)
}

/// Returns all contacts
fn all(db: &Database) -> Result<Vec<Contact>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/get_all.sql"))?;

    let rows = stmt.query_map(NO_PARAMS, Contact::from_db)?;

    let mut names: Vec<Contact> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

/// Returns all contacts with the specified `status`
fn get_by_status(db: &Database, status: ContactStatus) -> Result<Vec<Contact>, HErr> {
    let mut stmt = db.prepare(include_str!("sql/contact/get_by_status.sql"))?;

    let rows = stmt.query_map(params![status], Contact::from_db)?;

    let mut names: Vec<Contact> = Vec::new();
    for name_res in rows {
        names.push(name_res?);
    }

    Ok(names)
}

impl ContactsHandle {
    /// Creates new `ContactsHandle`
    pub fn new() -> Result<Self, HErr> {
        Ok(Self {
            db: Database::get()?,
        })
    }

    /// Gets a contact's name by their `id`.
    pub fn name(&self, id: UserIdRef) -> Result<Option<String>, HErr> {
        name(&self.db, id)
    }

    /// Change name of contact by their `id`
    pub fn set_name(&self, id: UserIdRef, name: Option<&str>) -> Result<(), HErr> {
        set_name(&self.db, id, name)
    }

    /// Gets a contact's profile picture by their `id`.
    pub fn profile_picture(&self, id: UserIdRef) -> Result<Option<String>, HErr> {
        profile_picture(&self.db, id)
    }

    /// Updates a contact's profile picture.
    pub fn set_profile_picture(
        &self,
        id: UserIdRef,
        profile_picture: Option<String>,
        old_path: Option<&str>,
    ) -> Result<Option<String>, HErr> {
        set_profile_picture(&self.db, id, profile_picture, old_path)
    }

    /// Sets a contact's color
    pub fn set_color(&self, id: UserIdRef, color: u32) -> Result<(), HErr> {
        set_color(&self.db, id, color)
    }

    /// Indicates whether contact exists
    pub fn contact_exists(&self, id: UserIdRef) -> Result<bool, HErr> {
        contact_exists(&self.db, id)
    }

    /// Sets contact status
    pub fn set_status(&self, id: UserIdRef, status: ContactStatus) -> Result<(), HErr> {
        set_status(&self.db, id, status)
    }

    /// Gets contact status
    pub fn status(&self, id: UserIdRef) -> Result<ContactStatus, HErr> {
        status(&self.db, id)
    }

    /// Returns all contacts
    pub fn all(&self) -> Result<Vec<Contact>, HErr> {
        all(&self.db)
    }

    /// Returns all contacts with the specified `status`
    pub fn get_by_status(&self, status: ContactStatus) -> Result<Vec<Contact>, HErr> {
        get_by_status(&self.db, status)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
/// Status of the contact
pub enum ContactStatus {
    /// The contact is active
    Active = 0,
    /// The contact is archived
    Archived = 1,
    /// The contact is deleted
    Deleted = 2,
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
/// Type of the contact
pub enum ContactType {
    /// The contact is local
    Local = 0,
    /// The contact is remote
    Remote = 1,
}

impl rusqlite::types::FromSql for ContactType {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl rusqlite::ToSql for ContactType {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
        use rusqlite::types::*;
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl std::convert::TryFrom<u8> for ContactType {
    type Error = HErr;

    fn try_from(n: u8) -> Result<Self, HErr> {
        use ContactType::*;
        match n {
            0 => Ok(Local),
            1 => Ok(Remote),
            unknown => Err(HErr::HeraldError(format!(
                "Unknown contact status {}",
                unknown
            ))),
        }
    }
}

impl std::convert::TryFrom<i64> for ContactType {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n.try_into(),
            Err(_) => Err(HErr::HeraldError(format!("Unknown contact status {}", n))),
        }
    }
}

impl rusqlite::types::FromSql for ContactStatus {
    fn column_result(value: rusqlite::types::ValueRef) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

impl rusqlite::ToSql for ContactStatus {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput, rusqlite::Error> {
        use rusqlite::types::*;
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

impl std::convert::TryFrom<u8> for ContactStatus {
    type Error = HErr;

    fn try_from(n: u8) -> Result<Self, HErr> {
        use ContactStatus::*;
        match n {
            0 => Ok(Active),
            1 => Ok(Archived),
            2 => Ok(Deleted),
            unknown => Err(HErr::HeraldError(format!(
                "Unknown contact status {}",
                unknown
            ))),
        }
    }
}

impl std::convert::TryFrom<i64> for ContactStatus {
    type Error = HErr;

    fn try_from(n: i64) -> Result<Self, HErr> {
        match u8::try_from(n) {
            Ok(n) => n.try_into(),
            Err(_) => Err(HErr::HeraldError(format!("Unknown contact status {}", n))),
        }
    }
}

/// Builder for `Contact`
pub struct ContactBuilder {
    /// Contact id
    id: UserId,
    /// Contact name
    name: Option<String>,
    /// Path of profile picture
    profile_picture: Option<String>,
    /// User set color for user
    color: Option<u32>,
    /// Indicates whether user is archived
    status: Option<ContactStatus>,
    /// Pairwise conversation corresponding to contact
    pairwise_conversation: Option<ConversationId>,
    /// Indicates that the contact is the local contact
    contact_type: Option<ContactType>,
}

impl ContactBuilder {
    /// Creates new `ContactBuilder`
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            profile_picture: None,
            name: None,
            color: None,
            status: None,
            pairwise_conversation: None,
            contact_type: None,
        }
    }

    /// Sets the name of the contact being built.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the profile picture of the contact being built.
    pub fn profile_picture(mut self, profile_picture: String) -> Self {
        self.profile_picture = Some(profile_picture);
        self
    }

    /// Sets the color of the contact being built.
    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the status of the contact being built.
    pub fn status(mut self, status: ContactStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the pairwise conversation id of the contact being built.
    pub fn pairwise_conversation(mut self, pairwise_conversation: ConversationId) -> Self {
        self.pairwise_conversation = Some(pairwise_conversation);
        self
    }

    pub(crate) fn local(mut self) -> Self {
        self.contact_type = Some(ContactType::Local);
        self
    }

    /// Adds contact to database
    pub fn add(self) -> Result<Contact, HErr> {
        let mut db = Database::get()?;

        let tx = db.transaction()?;
        let contact = Self::add_with_tx(self, &tx);
        tx.commit()?;
        contact
    }

    pub(crate) fn add_with_tx(self, conn: &rusqlite::Transaction) -> Result<Contact, HErr> {
        let color = self
            .color
            .unwrap_or_else(|| crate::utils::id_to_color(self.id.as_str()));

        let name = self.name.as_ref().map(|s| s.as_str());

        let contact_type = self.contact_type.unwrap_or(ContactType::Remote);

        let title = if let ContactType::Local = contact_type {
            Some(crate::config::NTS_CONVERSATION_NAME)
        } else {
            name
        };

        let pairwise_conversation = match self.pairwise_conversation {
            Some(conv_id) => crate::conversation::add_conversation(&conn, Some(&conv_id), title)?,
            None => crate::conversation::add_conversation(&conn, None, title)?,
        };

        let contact = Contact {
            id: self.id,
            name: self.name,
            profile_picture: self.profile_picture,
            color,
            status: self.status.unwrap_or(ContactStatus::Active),
            pairwise_conversation,
            contact_type,
        };

        conn.execute(
            include_str!("sql/contact/add.sql"),
            params![
                contact.id,
                contact.name,
                contact.profile_picture,
                contact.color,
                contact.status,
                contact.pairwise_conversation.as_slice(),
                contact.contact_type as u8
            ],
        )?;
        conn.execute(
            include_str!("sql/members/add_member.sql"),
            params![contact.pairwise_conversation.as_slice(), contact.id],
        )?;

        Ok(contact)
    }
}

#[derive(Debug, PartialEq, Clone)]
/// A Herald contact.
pub struct Contact {
    /// Contact id
    pub id: UserId,
    /// Contact name
    pub name: Option<String>,
    /// Path of profile picture
    pub profile_picture: Option<String>,
    /// User set color for user
    pub color: u32,
    /// Indicates whether user is archived
    pub status: ContactStatus,
    /// Pairwise conversation corresponding to contact
    pub pairwise_conversation: ConversationId,
    /// Contact type, local or remote
    pub contact_type: ContactType,
}

impl Contact {
    /// Returns name
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    ///// Sets contact name
    //pub fn set_name(&mut self, name: Option<&str>) -> Result<(), HErr> {
    //    ContactsHandle::set_name(&self.id, name)?;
    //    self.name = name.map(|s| s.to_owned());
    //    Ok(())
    //}

    /// Returns path to profile picture
    pub fn profile_picture(&self) -> Option<&str> {
        self.profile_picture.as_ref().map(|s| s.as_ref())
    }

    ///// Sets profile picture
    //pub fn set_profile_picture(&mut self, profile_picture: Option<String>) -> Result<(), HErr> {
    //    let path = ContactsHandle::set_profile_picture(
    //        self.id.as_str(),
    //        profile_picture,
    //        self.profile_picture.as_ref().map(|p| p.as_str()),
    //    )?;
    //    self.profile_picture = path;
    //    Ok(())
    //}

    /// Returns contact's color
    pub fn color(&self) -> u32 {
        self.color
    }

    ///// Sets color
    //pub fn set_color(&mut self, color: u32) -> Result<(), HErr> {
    //    ContactsHandle::set_color(self.id.as_str(), color)?;
    //    self.color = color;
    //    Ok(())
    //}

    /// Returns contact's status
    pub fn status(&self) -> ContactStatus {
        self.status
    }

    ///// Sets status
    //pub fn set_status(&mut self, status: ContactStatus) -> Result<(), HErr> {
    //    ContactsHandle::set_status(self.id.as_str(), status)?;
    //    self.status = status;
    //    Ok(())
    //}

    fn from_db(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Contact {
            id: row.get(0)?,
            name: row.get(1)?,
            profile_picture: row.get(2)?,
            color: row.get(3)?,
            status: row.get(4)?,
            pairwise_conversation: row.get::<_, Vec<u8>>(5)?.into_iter().collect(),
            contact_type: row.get(6)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;
    use serial_test_derive::serial;
    use womp::*;

    #[test]
    #[serial]
    fn create_drop_exists() {
        Database::reset_all().expect(womp!());
        // drop twice, it shouldn't panic on multiple drops
        ContactsHandle::drop_table().expect(womp!());
        ContactsHandle::drop_table().expect(womp!());

        ContactsHandle::create_table().expect(womp!());
        assert!(ContactsHandle::exists().expect(womp!()));
        ContactsHandle::create_table().expect(womp!());
        assert!(ContactsHandle::exists().expect(womp!()));
        ContactsHandle::drop_table().expect(womp!());
        assert!(!ContactsHandle::exists().expect(womp!()));
    }

    #[test]
    #[serial]
    fn add_contact() {
        Database::reset_all().expect(womp!());

        let id1 = "Hello";
        let id2 = "World";

        ContactBuilder::new(id1.into())
            .name("name".into())
            .add()
            .expect("Failed to add contact");
        ContactBuilder::new(id2.into())
            .color(1)
            .add()
            .expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        Database::reset_all().expect(womp!());

        let id = "Hello World";

        let handle = ContactsHandle::new().expect(womp!());

        ContactBuilder::new(id.into())
            .name("name".into())
            .add()
            .expect("Failed to add contact");

        assert_eq!(
            handle.name(id).expect("Failed to get name").expect(womp!()),
            "name"
        );
    }

    #[test]
    #[serial]
    fn get_contact_profile_picture() {
        Database::reset_all().expect(womp!());

        let id = "Hello World";
        let profile_picture = "picture";

        let handle = ContactsHandle::new().expect(womp!());

        ContactBuilder::new(id.into())
            .profile_picture(profile_picture.into())
            .add()
            .expect("Failed to add contact");

        assert_eq!(
            handle
                .profile_picture(id.into())
                .expect("Failed to get profile picture")
                .expect(womp!())
                .as_str(),
            profile_picture
        );
    }

    #[test]
    #[serial]
    fn update_name() {
        Database::reset_all().expect(womp!());

        let id = "userid";

        let handle = ContactsHandle::new().expect(womp!());

        ContactBuilder::new(id.into())
            .name("Hello".into())
            .add()
            .expect(womp!());
        handle
            .set_name(id, Some("World"))
            .expect("Failed to update name");

        assert_eq!(
            handle
                .name(id)
                .expect("Failed to get contact")
                .expect(womp!()),
            "World"
        );
    }

    #[test]
    #[serial]
    fn all_contacts() {
        Database::reset_all().expect(womp!());

        let id1 = "Hello";
        let id2 = "World";

        let handle = ContactsHandle::new().expect(womp!());

        ContactBuilder::new(id1.into())
            .add()
            .expect("Failed to add id1");
        ContactBuilder::new(id2.into())
            .add()
            .expect("Failed to add id2");

        let contacts = handle.all().expect(womp!());
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].id, id1);
        assert_eq!(contacts[1].id, id2);
    }

    #[test]
    #[serial]
    fn set_status() {
        Database::reset_all().expect(womp!());

        let handle = ContactsHandle::new().expect(womp!());
        let id = "Hello World";
        ContactBuilder::new(id.into()).add().expect(womp!());
        handle
            .set_status(id, ContactStatus::Archived)
            .expect(womp!());

        assert_eq!(
            handle
                .status(id)
                .expect("Failed to determine contact status"),
            ContactStatus::Archived
        );
    }

    #[test]
    #[serial]
    fn by_status_contacts() {
        Database::reset_all().expect(womp!());

        let id1 = "Hello";
        let id2 = "World";

        let handle = ContactsHandle::new().expect(womp!());

        ContactBuilder::new(id1.into())
            .add()
            .expect("Failed to add id1");
        ContactBuilder::new(id2.into())
            .status(ContactStatus::Archived)
            .add()
            .expect("Failed to add id2");

        let contacts = handle.get_by_status(ContactStatus::Active).expect(womp!());
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].id, id1);
    }
}
