use std::{fmt, time};

use serde::{Deserialize, Serialize};

use crate::{domain::entities::AppError, utils};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataStreamType {
    OneShot,
    Chunk,
}

pub trait RpcPayload: Send {
    fn get_ttl_secs(&self) -> u16;
    fn get_create_at(&self) -> u64;
    fn encode(&self) -> Result<Vec<u8>, AppError>
    where
        Self: Serialize + Sized,
    {
        utils::encode_msg_pack(self)
    }
    fn decode<'data>(encoded: &'data Vec<u8>) -> Result<Self, AppError>
    where
        Self: Deserialize<'data>,
    {
        utils::decode_msg_pack(encoded)
    }
}

const TTL: u16 = time::Duration::from_secs(45).as_secs() as u16;

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcError {
    code: u16,
    info: String,
    timestamp: u64,
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "code: {}, info: {}, timestamp: {}",
            self.code, self.info, self.timestamp
        ))
    }
}

impl std::error::Error for RpcError {}

#[derive(Serialize, Deserialize, Default)]
pub struct TimeCommand {
    caller_sent_at: Option<u64>,
    time_offset: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct RpcMetadata {
    create_at: u64,
    ttl_secs: u16,
}

impl Default for RpcMetadata {
    fn default() -> Self {
        Self {
            create_at: time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_secs: TTL,
        }
    }
}

/**
 *
 * Command-message should be messagepack encoded.
 */
#[derive(Serialize, Deserialize)]
pub struct RpcCallPayload {
    metadata: RpcMetadata,

    pub caller_inbox: String,
    pub data_stream_type: DataStreamType,
}

impl RpcCallPayload {
    pub fn new_oneshot(caller_inbox: String) -> Self {
        Self {
            metadata: Default::default(),
            caller_inbox,
            data_stream_type: DataStreamType::OneShot,
        }
    }

    pub fn new_stream(caller_inbox: String) -> Self {
        Self {
            metadata: Default::default(),
            caller_inbox,
            data_stream_type: DataStreamType::Chunk,
        }
    }
}

impl RpcPayload for RpcCallPayload {
    fn get_ttl_secs(&self) -> u16 {
        self.metadata.ttl_secs
    }

    fn get_create_at(&self) -> u64 {
        self.metadata.create_at
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcResultPayload {
    metadata: RpcMetadata,

    pub responder_inbox: String,
    pub data_stream_type: DataStreamType,
}

impl RpcResultPayload {
    pub fn new_oneshot(responder_inbox: String) -> Self {
        Self {
            metadata: Default::default(),
            responder_inbox,
            data_stream_type: DataStreamType::OneShot,
        }
    }

    pub fn new_stream(responder_inbox: String) -> Self {
        Self {
            metadata: Default::default(),
            responder_inbox,
            data_stream_type: DataStreamType::Chunk,
        }
    }
}

impl RpcPayload for RpcResultPayload {
    fn get_ttl_secs(&self) -> u16 {
        self.metadata.ttl_secs
    }

    fn get_create_at(&self) -> u64 {
        self.metadata.create_at
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcContinueDataPayload {
    metadata: RpcMetadata,

    pub data: Vec<u8>,
}

impl RpcContinueDataPayload {
    pub fn new<D>(data: &D) -> Result<Self, AppError>
    where
        D: Serialize,
    {
        Ok(Self {
            data: utils::encode_msg_pack(&data)?,
            metadata: Default::default(),
        })
    }
}

impl RpcPayload for RpcContinueDataPayload {
    fn get_ttl_secs(&self) -> u16 {
        self.metadata.ttl_secs
    }

    fn get_create_at(&self) -> u64 {
        self.metadata.create_at
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcClosePayload {
    metadata: RpcMetadata,
    error: Option<RpcError>,
}

impl RpcClosePayload {
    pub fn new(error: Option<RpcError>) -> Self {
        Self {
            error,
            metadata: Default::default(),
        }
    }
}

impl RpcPayload for RpcClosePayload {
    fn get_ttl_secs(&self) -> u16 {
        self.metadata.ttl_secs
    }

    fn get_create_at(&self) -> u64 {
        self.metadata.create_at
    }
}
