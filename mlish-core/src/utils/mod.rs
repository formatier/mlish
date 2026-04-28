use serde::{Deserialize, Serialize};

use crate::domain::entities::AppError;

#[macro_export]
macro_rules! inline_mod {
    ($($name:ident), + $(,)?) => {
        $(
            mod $name; pub use $name::*;
        )+
    };
}

pub fn encode_msg_pack<T>(data: &T) -> Result<Vec<u8>, AppError>
where
    T: Serialize + Sized,
{
    let mut buf = Vec::new();
    data.serialize(&mut rmp_serde::Serializer::new(&mut buf))
        .map_err(|err| err.into())
        .map(move |_| buf)
}

pub fn decode_msg_pack<'data, T>(encoded: &'data Vec<u8>) -> Result<T, AppError>
where
    T: Deserialize<'data>,
{
    T::deserialize(&mut rmp_serde::Deserializer::new(std::io::Cursor::new(
        encoded,
    )))
    .map_err(|err| err.into())
}
