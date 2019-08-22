use crate::{
    db::{DBTable, Database},
    errors::HErr,
    image_utils,
};
use rusqlite::{
    types::{Null, ToSql},
    NO_PARAMS,
};

#[derive(Default)]
/// Wrapper around contacts table.
/// TODO This will be stateful when we have caching logic.
pub struct Contacts {}

impl DBTable for Contacts {
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
}

impl Contacts {
    /// Inserts contact into contacts table.
    pub fn add(id: &str, name: Option<&str>, profile_picture: Option<&str>) -> Result<(), HErr> {
        let name = match name {
            Some(name) => name.to_sql()?,
            None => id.to_sql()?,
        };

        let profile_picture = match profile_picture {
            Some(profile_picture) => profile_picture.to_sql()?,
            None => Null.to_sql()?,
        };

        let db = Database::get()?;
        db.execute(
            include_str!("sql/contact/add.sql"),
            &[id.to_sql()?, name, profile_picture],
        )?;
        Ok(())
    }

    /// Gets a contact's name by their `id`.
    pub fn name(id: &str) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_name.sql"))?;

        Ok(stmt.query_row(&[id], |row| row.get(0))?)
    }

    /// Change name of contact by their `id`
    pub fn set_name(id: &str, name: Option<&str>) -> Result<(), HErr> {
        let name = match name {
            Some(name) => name.to_sql()?,
            None => Null.to_sql()?,
        };

        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/update_name.sql"))?;

        stmt.execute(&[name, id.to_sql()?])?;
        Ok(())
    }

    /// Gets a contact's profile picture by their `id`.
    pub fn profile_picture(id: String) -> Result<Option<String>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_profile_picture.sql"))?;

        Ok(stmt.query_row(&[id.as_str()], |row| row.get(0))?)
    }

    /// Updates a contact's profile picture.
    pub fn set_profile_picture(
        id: &str,
        profile_picture: Option<String>,
    ) -> Result<Option<String>, HErr> {
        let profile_picture = match profile_picture {
            Some(path) => {
                let path_string = image_utils::save_profile_picture(id, path)?
                    .into_os_string()
                    .into_string()?;
                Some(path_string)
            }
            None => {
                image_utils::delete_profile_picture(id)?;
                None
            }
        };

        let db = Database::get()?;

        db.execute(
            include_str!("sql/contact/update_profile_picture.sql"),
            &[profile_picture.to_sql()?, id.to_sql()?],
        )?;
        Ok(profile_picture)
    }

    /// Indicates whether contact exists
    pub fn contact_exists(id: &str) -> Result<bool, HErr> {
        let db = Database::get()?;

        let mut stmt = db.prepare(include_str!("sql/contact/contact_exists.sql"))?;
        Ok(stmt.exists(&[id])?)
    }

    /// Deletes a contact by their `id`.
    pub fn delete(id: &str) -> Result<(), HErr> {
        let mut db = Database::get()?;

        let tx = db.transaction()?;
        tx.execute(include_str!("sql/contact/delete.sql"), &[id])?;
        tx.execute(include_str!("sql/message/delete_conversation.sql"), &[id])?;
        tx.commit()?;
        Ok(())
    }

    /// Archives a contact.
    pub fn archive(id: &str) -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(include_str!("sql/contact/archive_contact.sql"), &[id])?;
        Ok(())
    }

    /// Indicates whether a contact is archived.
    pub fn is_archived(id: &str) -> Result<bool, HErr> {
        let db = Database::get()?;

        let val: i64 = db.query_row(include_str!("sql/contact/is_archived.sql"), &[id], |row| {
            Ok(row.get(0)?)
        })?;

        Ok(val == 1)
    }

    /// Returns all contact, including archived contacts
    pub fn all() -> Result<Vec<Contact>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_all.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            Ok(Contact {
                id: row.get(0)?,
                name: row.get(1)?,
                profile_picture: row.get(2)?,
            })
        })?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }

    /// Returns all active Contacts:: excluding archived Contacts::
    pub fn active() -> Result<Vec<Contact>, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("sql/contact/get_active.sql"))?;

        let rows = stmt.query_map(NO_PARAMS, |row| {
            Ok(Contact {
                id: row.get(0)?,
                name: row.get(1)?,
                profile_picture: row.get(2)?,
            })
        })?;

        let mut names: Vec<Contact> = Vec::new();
        for name_res in rows {
            names.push(name_res?);
        }

        Ok(names)
    }
}

#[derive(Debug, PartialEq)]
/// A Herald contact.
pub struct Contact {
    /// Contact id
    pub id: String,
    /// Contact name
    pub name: Option<String>,
    /// Path of profile picture
    pub profile_picture: Option<String>,
}

impl Contact {
    /// Create new contact.
    pub fn new(id: String, name: Option<String>, profile_picture: Option<String>) -> Self {
        Contact {
            name,
            id,
            profile_picture,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial;

    #[test]
    #[serial]
    fn create_drop_exists() {
        // drop twice, it shouldn't panic on multiple drops
        Contacts::drop_table().unwrap();
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        assert!(Contacts::exists().unwrap());
        Contacts::create_table().unwrap();
        assert!(Contacts::exists().unwrap());
        Contacts::drop_table().unwrap();
        assert!(!Contacts::exists().unwrap());
    }

    #[test]
    #[serial]
    fn add_contact() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, Some("name"), None).expect("Failed to add contact");
        Contacts::add(id2, None, None).expect("Failed to add contact");
    }

    #[test]
    #[serial]
    fn delete_contact() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, None, None).expect("Failed to add contact");
        Contacts::add(id2, None, None).expect("Failed to add contact");
        crate::message::Messages::create_table().unwrap();

        Contacts::delete(id1).expect("Failed to delete contact");

        assert!(Contacts::name(id1).is_err());
        assert!(Contacts::name(id2).is_ok());
    }

    #[test]
    #[serial]
    fn get_contact_name() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        let id = "Hello World";

        Contacts::add(id, Some("name"), None).expect("Failed to add contact");
        assert_eq!(
            Contacts::name(id).expect("Failed to get name").unwrap(),
            "name"
        );
    }

    #[test]
    #[serial]
    fn get_contact_profile_picture() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();
        let id = "Hello World";
        let profile_picture = "picture";
        Contacts::add(id, None, Some(profile_picture)).expect("Failed to add contact");
        assert_eq!(
            Contacts::profile_picture(id.into())
                .expect("Failed to get profile picture")
                .unwrap()
                .as_str(),
            profile_picture
        );
    }

    #[test]
    #[serial]
    fn update_name() {
        Contacts::drop_table().unwrap();
        Contacts::create_table().unwrap();

        let id = "userid";

        Contacts::add(id, Some("Hello"), None).unwrap();
        Contacts::set_name(id, Some("World")).expect("Failed to update name");

        assert_eq!(
            Contacts::name(id).expect("Failed to get contact").unwrap(),
            "World"
        );
    }

    #[test]
    #[serial]
    fn all_contacts() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, None, None).expect("Failed to add id1");
        Contacts::add(id2, None, None).expect("Failed to add id2");

        let contacts = Contacts::all().unwrap();
        assert_eq!(contacts.len(), 2);
        assert_eq!(contacts[0].id, id1);
        assert_eq!(contacts[1].id, id2);
    }

    #[test]
    #[serial]
    fn archive_contact() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id = "Hello World";
        Contacts::add(id, None, None).unwrap();
        Contacts::archive(id).unwrap();

        assert!(Contacts::is_archived(id).expect("Failed to determine if contact was archived"));
    }

    #[test]
    #[serial]
    fn get_active_contacts() {
        Contacts::drop_table().unwrap();

        Contacts::create_table().unwrap();

        let id1 = "Hello";
        let id2 = "World";

        Contacts::add(id1, None, None).unwrap();
        Contacts::add(id2, None, None).unwrap();

        Contacts::archive(id2).unwrap();

        let contacts = Contacts::active().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].id, id1);
    }
}