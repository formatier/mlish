use std::{fmt::Display, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::domain::{blueprints::RpcExecutor, entities::AppError};

pub struct RpcPipe<E> {
    rpc_executor: E,
}

impl<E> RpcPipe<E>
where
    E: RpcExecutor,
{
    pub fn new(executor: E) -> Self {
        Self {
            rpc_executor: executor,
        }
    }

    pub async fn call_oneshot_result_oneshot<'instance, 'data, M, P, T>(
        &'instance self,
        method: &'instance M,
        params: Option<&P>,
    ) -> Result<T, AppError>
    where
        M: Display,
        P: Serialize,
        T: Deserialize<'data>,
        'data: 'instance,
    {
        todo!("impl")
    }

    pub async fn call_oneshot_result_stream<'instance, 'data, M, P, T, S>(
        &'instance self,
        method: &'instance M,
        params: Option<&P>,
    ) -> Result<T, AppError>
    where
        M: Display,
        P: Serialize,
        T: tokio_stream::StreamExt<Item = Result<S, AppError>>,
        S: Deserialize<'data>,
        'data: 'instance,
    {
        todo!("impl")
    }

    pub async fn call_stream_result_oneshot<'instance, 'data, M, R, P, T>(
        &'instance self,
        method: &'instance M,
        params: Option<&R>,
    ) -> Result<T, AppError>
    where
        M: Display,
        R: tokio_stream::StreamExt<Item = Result<P, AppError>>,
        P: Serialize,
        T: Deserialize<'data>,
        'data: 'instance,
    {
        todo!("impl")
    }

    pub async fn call_stream_result_stream<'instance, 'data, M, R, P, T, S>(
        &'instance self,
        method: &'instance M,
        params: Option<&R>,
    ) -> Result<T, AppError>
    where
        M: Display,
        R: tokio_stream::StreamExt<Item = Result<P, AppError>>,
        P: Serialize,
        T: tokio_stream::StreamExt<Item = Result<S, AppError>>,
        S: Deserialize<'data>,
        'data: 'instance,
    {
        todo!("impl")
    }
}

pub struct RpcCommandBuilder<E> {
    pipe: Arc<RpcPipe<E>>,
}

impl<E> RpcCommandBuilder<E>
where
    E: RpcExecutor,
{
    pub fn new(pipe: Arc<RpcPipe<E>>) -> Self {
        Self { pipe }
    }
}
