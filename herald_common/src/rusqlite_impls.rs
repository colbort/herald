use crate::{time::Time, types::*};
use rusqlite::{types as sql_types, ToSql};
use std::convert::TryFrom;

impl ToSql for UserId {
    fn to_sql(&self) -> Result<sql_types::ToSqlOutput, rusqlite::Error> {
        use sql_types::*;
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(
            self.as_str().as_bytes(),
        )))
    }
}

impl sql_types::FromSql for UserId {
    fn column_result(value: sql_types::ValueRef) -> sql_types::FromSqlResult<Self> {
        UserId::try_from(value.as_str()?).map_err(|_| sql_types::FromSqlError::InvalidType)
    }
}

impl ToSql for Time {
    fn to_sql(&self) -> Result<sql_types::ToSqlOutput, rusqlite::Error> {
        use sql_types::*;

        Ok(ToSqlOutput::Owned(Value::Integer((*self).into())))
    }
}

impl sql_types::FromSql for Time {
    fn column_result(value: sql_types::ValueRef) -> sql_types::FromSqlResult<Self> {
        Ok(value.as_i64()?.into())
    }
}
