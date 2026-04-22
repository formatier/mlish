use std::time;

use serde::{Deserialize, Serialize};

const TTL: u16 = time::Duration::from_secs(45).as_secs() as u16;

// Command-message should be messagepack encoded.
#[derive(Serialize, Deserialize)]
pub struct CommandMessage {
    id: String,
    command: Command,
    method: Option<String>,
    parsed_params: Option<Vec<u8>>,
    data_sending_type: Option<DataSendingType>,
    create_at: u128,
    ttl_secs: u16,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TimeCommand {
    sender_sent_at: Option<u128>,
    time_offset: Option<i32>,
}

impl CommandMessage {
    pub fn new_oneshot_call(
        id: &uuid::Uuid,
        method: String,
        parsed_params: Option<Vec<u8>>,
    ) -> Self {
        let mut instance = Self::default();
        instance.id = id.to_string();
        instance.method = Some(method);
        instance.parsed_params = parsed_params;
        instance.data_sending_type = Some(DataSendingType::OneShot);
        instance
    }

    pub fn new_stream_call(id: &uuid::Uuid, method: Option<String>) -> Self {
        let mut instance = Self::default();
        instance.id = id.to_string();
        instance.method = method;
        instance.data_sending_type = Some(DataSendingType::Chunk);
        instance
    }

    pub fn new_oneshot_result(id: &uuid::Uuid, parsed_params: Option<Vec<u8>>) -> Self {
        let mut instance = Self::default();
        instance.id = id.to_string();
        instance.command = Command::Result;
        instance.parsed_params = parsed_params;
        instance.data_sending_type = Some(DataSendingType::OneShot);
        instance
    }

    pub fn new_stream_result(id: &uuid::Uuid) -> Self {
        let mut instance = Self::default();
        instance.id = id.to_string();
        instance.command = Command::Result;
        instance.data_sending_type = Some(DataSendingType::Chunk);
        instance
    }

    pub fn new_error(id: &uuid::Uuid, parsed_params: Option<Vec<u8>>) -> Self {
        let mut instance = Self::default();
        instance.id = id.to_string();
        instance.command = Command::Error;
        instance.parsed_params = parsed_params;
        instance
    }

    pub fn new_continue_data(id: &uuid::Uuid, parsed_params: Vec<u8>) -> Self {
        let mut instance = Self::default();
        instance.id = id.to_string();
        instance.command = Command::ContinueData;
        instance.parsed_params = Some(parsed_params);
        instance
    }
}

impl Default for CommandMessage {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            command: Command::Call,
            method: None,
            parsed_params: None,
            data_sending_type: None,
            create_at: time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            ttl_secs: TTL,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Call,
    Result,
    Error,
    Ack,
    ClockSync,
    ContinueData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSendingType {
    OneShot,
    Chunk,
}
