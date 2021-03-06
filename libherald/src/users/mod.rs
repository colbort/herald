use crate::{
    err, ffi,
    interface::{UsersEmitter as Emitter, UsersList as List, UsersTrait as Interface},
    none, spawn,
};
use herald_common::UserId;
use heraldcore::network;
use heraldcore::user::{self, UserBuilder};
use search_pattern::SearchPattern;
use std::convert::TryInto;

pub(crate) mod shared;
use shared::*;
mod imp;
mod trait_imp;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
/// Thin wrapper around a `UserId`,
/// with an additional field to facilitate filtering
/// in the UI.
pub struct UserIndex {
    pub(crate) id: UserId,
    pub(crate) matched: bool,
}

/// A wrapper around a vector of `User`s, with additional
/// fields to facilitate interaction with Qt.
pub struct Users {
    emit: Emitter,
    model: List,
    filter: Option<SearchPattern>,
    filter_regex: bool,
    list: Vec<UserIndex>,
    loaded: bool,
}

impl Interface for Users {
    fn new(
        emit: Emitter,
        model: List,
    ) -> Users {
        // this should *really* never fail
        let filter = SearchPattern::new_normal("".into()).ok();

        Users {
            emit,
            model,
            list: Vec::new(),
            filter,
            filter_regex: false,
            loaded: false,
        }
    }

    /// Adds a user by their `id`
    fn add(
        &mut self,
        id: ffi::UserId,
    ) -> ffi::ConversationId {
        let id = err!(id.as_str().try_into(), ffi::NULL_CONV_ID.to_vec());
        let (data, _) = err!(UserBuilder::new(id).add(), ffi::NULL_CONV_ID.to_vec());

        let pairwise_conversation = data.pairwise_conversation;

        let user = UserIndex {
            matched: self
                .filter
                .as_ref()
                .map(|filter| data.matches(filter))
                .unwrap_or(true),
            id: data.id,
        };

        let pos = match self.list.binary_search(&user) {
            Ok(_) => return pairwise_conversation.to_vec(),
            Err(pos) => pos,
        };

        self.model.begin_insert_rows(pos, pos);
        self.list.insert(pos, user);

        {
            shared::user_data().write().insert(data.id, data);
        }

        self.model.end_insert_rows();

        spawn!(
            err!(network::send_user_req(id, pairwise_conversation)),
            ffi::NULL_CONV_ID.to_vec()
        );

        pairwise_conversation.to_vec()
    }

    /// Returns user id.
    fn user_id(
        &self,
        row_index: usize,
    ) -> ffi::UserIdRef {
        none!(self.list.get(row_index), "").id.as_str()
    }

    fn index_by_id(
        &self,
        uid: ffi::UserId,
    ) -> i64 {
        let uid = &err!(uid.as_str().try_into(), -1);
        self.list
            .iter()
            .position(|UserIndex { id, .. }| id == uid)
            .map(|n| n as i64)
            .unwrap_or(-1)
    }

    fn matched(
        &self,
        row_index: usize,
    ) -> bool {
        none!(self.list.get(row_index), true).matched
    }

    fn filter(&self) -> &str {
        self.filter.as_ref().map(SearchPattern::raw).unwrap_or("")
    }

    fn set_filter(
        &mut self,
        pattern: String,
    ) {
        if pattern.is_empty() {
            self.clear_filter();
            return;
        }

        self.filter = match self.filter.take() {
            Some(mut pat) => {
                err!(pat.set_pattern(pattern));
                Some(pat)
            }
            None => SearchPattern::new_normal(pattern).ok(),
        };

        self.emit.filter_changed();

        self.inner_filter();
    }

    /// Indicates whether regex search is activated
    fn filter_regex(&self) -> bool {
        self.filter_regex
    }

    /// Sets filter mode
    fn set_filter_regex(
        &mut self,
        use_regex: bool,
    ) {
        self.filter = match self.filter.take() {
            Some(mut filter) => {
                if use_regex {
                    err!(filter.regex_mode());
                    Some(filter)
                } else {
                    err!(filter.normal_mode());
                    Some(filter)
                }
            }
            None => None,
        };

        self.filter_regex = use_regex;
        self.emit.filter_regex_changed();
        self.inner_filter();
    }

    /// Toggles filter mode
    ///
    /// Returns new value.
    fn toggle_filter_regex(&mut self) -> bool {
        let toggled = !self.filter_regex;
        self.set_filter_regex(toggled);
        toggled
    }

    fn emit(&mut self) -> &mut Emitter {
        &mut self.emit
    }

    fn row_count(&self) -> usize {
        self.list.len()
    }

    fn clear_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            if !user.matched {
                user.matched = true;
                self.model.data_changed(ix, ix);
            }
        }

        if self.filter_regex {
            self.filter = SearchPattern::new_regex("".to_owned()).ok();
        } else {
            self.filter = SearchPattern::new_normal("".to_owned()).ok();
        }

        self.emit.filter_changed();
    }
}
