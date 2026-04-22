use rmp_serde::Serializer;
use serde::Serialize;
use std::{collections::HashMap, fmt::Display, sync::Arc};
use tokio::sync::Mutex;

use crate::shared::domain::entities::{CommandMessage, IError};

pub struct RpcPipe {
    pub commands: Arc<Mutex<HashMap<uuid::Uuid, RpcCommand>>>,
}

pub struct RpcCommand {}

impl RpcPipe {
    pub async fn call<M, P>(&self, method: M, params: Option<P>) -> Result<CommandMessage, IError>
    where
        M: Display,
        P: Serialize,
    {
        let parsed_params = match params {
            Some(p) => {
                let mut buf = Vec::new();
                vec![];
                p.serialize(&mut Serializer::new(&mut buf))
                    .map_err(|err| IError::from_message_pack_data_serialization_error(err))?;
                Some(buf)
            }
            None => None,
        };

        let commands_id = uuid::Uuid::now_v7();
        let message =
            CommandMessage::new_oneshot_call(&commands_id, method.to_string(), parsed_params);
        let mut commands = self.commands.lock().await;
        let rpc_command = RpcCommand {};
        commands.insert(commands_id, rpc_command);
        Ok(message)
    }

    pub fn poll_ttl(&self) {}
}
