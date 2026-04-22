crate::inline_mod!(command);

use std::fmt::{self, Debug, Display, Formatter};
use time::macros::*;

pub enum IError<'rpc_info> {
    DatabaseError,
    DataSerializationError(DataSerializationError),
    ValidatorError(validator::ValidationErrors),
    RpcError {
        code: u16,
        info: &'rpc_info str,
        timestamp: u64,
    },
}

pub enum DataSerializationError {
    SerializeMsgPackError(rmp_serde::encode::Error),
    DeserializeMsgPackError(rmp_serde::decode::Error),
}

impl<'rpc_info> IError<'rpc_info> {
    pub fn from_database_error() -> Self {
        Self::DatabaseError
    }

    pub fn from_message_pack_data_serialization_error(error: rmp_serde::encode::Error) -> Self {
        Self::DataSerializationError(DataSerializationError::SerializeMsgPackError(error))
    }

    pub fn from_message_pack_data_deserialization_error(error: rmp_serde::decode::Error) -> Self {
        Self::DataSerializationError(DataSerializationError::DeserializeMsgPackError(error))
    }

    pub fn from_validator_error(error: validator::ValidationErrors) -> Self {
        Self::ValidatorError(error)
    }

    pub fn new_rpc_error(code: u16, info: &'rpc_info str, timestamp: u64) -> Self {
        Self::RpcError {
            code,
            info,
            timestamp,
        }
    }
}

impl<'rpc_info> Display for IError<'rpc_info> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseError => write!(f, "Database error"),
            Self::DataSerializationError(error) => match error {
                DataSerializationError::SerializeMsgPackError(error) => {
                    write!(f, "Data serialization error: {}", error)
                }
                DataSerializationError::DeserializeMsgPackError(error) => {
                    write!(f, "Data deserialization error: {}", error)
                }
            },
            Self::ValidatorError(error) => write!(f, "Validator error: {}", error),
            Self::RpcError {
                code,
                info,
                timestamp,
            } => {
                let format =
                    format_description!("[year] [month] [day] [hour]:[minute]:[second] UTC+0");
                let date =
                    time::UtcDateTime::from_unix_timestamp(*timestamp as i64).map_err(|e| {
                        todo!("Log error to monitor server");
                        fmt::Error::default()
                    })?;
                write!(
                    f,
                    "RPC error: code={}, info={}, timestamp={}",
                    code,
                    info,
                    date.format(format).map_err(|e| {
                        todo!("Log error to monitor server");
                        fmt::Error::default()
                    })?,
                )
            }
        }
    }
}

impl Debug for IError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let self_impl: &dyn Display = self;
        self_impl.fmt(f)
    }
}
