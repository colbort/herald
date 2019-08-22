use crate::interface::*;
use heraldcore::{
    contact::{self, Contacts as Core},
    db::DBTable,
};
use im_rc::vector::Vector as ImVector;

#[derive(Default, Clone)]
struct ContactsItem {
    contact_id: String,
    name: Option<String>,
    profile_picture: Option<String>,
}

impl From<contact::Contact> for ContactsItem {
    #[inline]
    fn from(val: contact::Contact) -> Self {
        let contact::Contact {
            id: contact_id,
            name,
            profile_picture,
        } = val;
        ContactsItem {
            contact_id,
            name,
            profile_picture,
        }
    }
}

pub struct Contacts {
    emit: ContactsEmitter,
    model: ContactsList,
    list: ImVector<ContactsItem>,
}

impl ContactsTrait for Contacts {
    fn new(emit: ContactsEmitter, model: ContactsList) -> Contacts {
        // create table if it does not already exist
        if let Err(e) = Core::create_table() {
            eprintln!("{}", e);
        }

        let list = match Core::all() {
            Ok(v) => v.into_iter().map(|c| c.into()).collect(),
            Err(_) => ImVector::new(),
        };
        Contacts { emit, model, list }
    }

    fn remove_all(&mut self) {
        self.model.begin_reset_model();

        if let Err(e) = Core::drop_table() {
            eprintln!("{}", e);
        }
        if let Err(e) = Core::create_table() {
            eprintln!("{}", e);
        }

        self.list = ImVector::new();

        self.model.end_reset_model();
    }

    /// Adds a contact by their `id`
    ///
    /// Returns `false` on failure.
    fn add(&mut self, id: String) -> bool {
        assert!(id.len() < 256);

        if let Err(e) = Core::add(id.as_str(), None, None) {
            eprintln!("Error: {}", e);
            return false;
        }

        self.model.begin_insert_rows(0, 0);
        self.list.push_front(ContactsItem {
            contact_id: id,
            name: None,
            profile_picture: None,
        });
        self.model.end_insert_rows();
        true
    }

    /// Returns profile picture given the contact's id.
    fn profile_picture(&self, row_index: usize) -> Option<&str> {
        self.list[row_index]
            .profile_picture
            .as_ref()
            .map(|s| s.as_str())
    }

    /// Sets profile picture.
    ///
    /// Returns bool indicating success.
    fn set_profile_picture(&mut self, row_index: usize, picture: Option<String>) -> bool {
        let id = self.list[row_index].contact_id.as_str();

        match Core::set_profile_picture(id, picture) {
            Ok(s) => {
                self.list[row_index].profile_picture = s;
                true
            }
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        }
    }

    /// Removes a contact by their `id`, returns a boolean to indicate success.
    fn remove(&mut self, row_index: u64) -> bool {
        let row_index = row_index as usize;

        let id = self.list[row_index].contact_id.as_str();
        match Core::delete(id) {
            Ok(_) => {
                self.model.begin_remove_rows(row_index, row_index);
                self.list.remove(row_index);
                self.model.end_remove_rows();
                true
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                false
            }
        }
    }

    /// Updates a contact's name, returns a boolean to indicate success.
    fn set_name(&mut self, row_index: usize, name: Option<String>) -> bool {
        self.list[row_index].name = name;
        let id = self.list[row_index].contact_id.as_str();

        let name = self.list[row_index].name.as_ref().map(|n| n.as_str());

        if Core::set_name(id, name).is_err() {
            return false;
        }

        true
    }

    fn emit(&mut self) -> &mut ContactsEmitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn contact_id(&self, row_index: usize) -> &str {
        self.list[row_index].contact_id.as_str()
    }

    fn name(&self, row_index: usize) -> Option<&str> {
        self.list[row_index].name.as_ref().map(|n| n.as_str())
    }
}