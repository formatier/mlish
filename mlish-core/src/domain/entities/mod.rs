crate::inline_mod!(command);
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("data serialization error")]
    MessagePackSerializer(#[from] rmp_serde::encode::Error),
    #[error("data deserialization error")]
    MessagePackDeserializer(#[from] rmp_serde::decode::Error),
    #[error("rpc error")]
    RpcError(#[from] RpcError),
}
