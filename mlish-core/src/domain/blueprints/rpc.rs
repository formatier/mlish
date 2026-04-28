use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::domain::entities::{AppError, RpcResultPayload};

pub trait RpcExecutor {
    fn new_inbox(&self) -> String;

    fn call<I, M, C>(
        &self,
        caller_inbox: &I,
        method: &M,
        payload: &C,
    ) -> impl Future<Output = RpcResultPayload>
    where
        I: Display,
        M: Display,
        C: Serialize;

    fn call_stream<I, M, C>(
        &self,
        caller_inbox: &I,
        method: &M,
        payload: &C,
    ) -> impl Future<Output = RpcResultPayload>
    where
        I: Display,
        M: Display,
        C: Serialize;

    fn bind() {
        todo!("add bind stream and bind one shot to bind_* method");
    }

    fn listen_oneshot_result<'instance, 'data, I, T>(
        &'instance self,
        inbox_id: &'instance I,
        ttl_secs: u16,
    ) -> impl Future<Output = Result<T, AppError>>
    where
        I: Display,
        T: Deserialize<'data>,
        'data: 'instance;

    fn listen_stream_result<'instance, 'data, I, T, S>(
        &'instance self,
        id: &'instance I,
    ) -> impl Future<Output = T>
    where
        I: Display,
        T: tokio_stream::StreamExt<Item = Result<S, AppError>>,
        S: Deserialize<'data>,
        'data: 'instance;
}
