use super::*;
use heraldcore::errors::HErr;

impl Users {
    pub(super) fn inner_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            let lock = shared::user_data().read();
            let inner = none!(lock.get(&user.id));
            let old_matched = user.matched;
            user.matched = self
                .filter
                .as_ref()
                .map(|filter| inner.matches(&filter))
                .unwrap_or(true);
            if user.matched != old_matched {
                self.model.data_changed(ix, ix);
            }
        }
    }

    pub(crate) fn handle_update(
        &mut self,
        update: UserUpdate,
    ) {
        match update {
            UserUpdate::NewUser(data) => {
                let new_user = UserIndex {
                    matched: self
                        .filter
                        .as_ref()
                        .map(|filter| data.matches(filter))
                        .unwrap_or(true),
                    id: data.id,
                };

                let pos = match self.list.binary_search(&new_user) {
                    Ok(_) => return, // this should never happen
                    Err(pos) => pos,
                };

                self.model.begin_insert_rows(pos, pos);
                self.list.push(new_user);
                user_data().write().insert(data.id, data);
                self.model.end_insert_rows();
            }
            UserUpdate::ReqResp(uid, accepted) => {
                if accepted {
                    println!("PLACEHOLDER: {} accepted your user request", uid);
                } else {
                    println!("PLACEHOLDER: {} did not accept your user request", uid);
                }
            }
            UserUpdate::UserChanged(uid, update) => {
                crate::user_push(uid, update);
            }
        }
    }
}

impl crate::Loadable for Users {
    type Error = HErr;

    fn try_load(&mut self) -> Result<(), HErr> {
        let users = user::all()?;

        let mut lock = shared::user_data().write();
        lock.reserve(users.len());
        for user in users {
            let id = user.id;
            lock.insert(id, user);
            self.list.push(UserIndex { id, matched: true });
        }
        drop(lock);

        self.loaded = true;

        Ok(())
    }

    fn loaded(&self) -> bool {
        self.loaded
    }
}
