use crate::{
    db::{DBTable, Database},
    errors::HErr,
};
use chainmail::{block::*, errors::Error as ChainError};
use herald_common::{sealed, sig, sign, Push, Signed, UserId, UserMeta};
use rusqlite::{params, NO_PARAMS};

#[derive(Default)]
pub(crate) struct ContactKeys {}

pub(crate) fn key_deprecated(k: Signed<sign::PublicKey>) -> Result<(), HErr> {
    Ok(())
}

pub(crate) fn key_registered(k: Signed<sign::PublicKey>) -> Result<(), HErr> {
    Ok(())
}

impl DBTable for ContactKeys {
    fn create_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("../sql/contact_keys/create_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn drop_table() -> Result<(), HErr> {
        let db = Database::get()?;
        db.execute(
            include_str!("../sql/contact_keys/drop_table.sql"),
            NO_PARAMS,
        )?;
        Ok(())
    }

    fn exists() -> Result<bool, HErr> {
        let db = Database::get()?;
        let mut stmt = db.prepare(include_str!("../sql/contact_keys/table_exists.sql"))?;
        Ok(stmt.exists(NO_PARAMS)?)
    }

    fn reset() -> Result<(), HErr> {
        let mut db = Database::get()?;
        let tx = db.transaction()?;
        tx.execute(
            include_str!("../sql/contact_keys/drop_table.sql"),
            NO_PARAMS,
        )?;
        tx.execute(
            include_str!("../sql/contact_keys/create_table.sql"),
            NO_PARAMS,
        )?;
        tx.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;