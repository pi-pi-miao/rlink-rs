use std::convert::TryFrom;

use bytes::{Buf, BufMut, BytesMut};

use crate::api::runtime::{ChannelKey, JobId, TaskId};

pub(crate) mod client;
pub(crate) mod server;

pub(crate) use client::run_subscribe;
pub(crate) use client::subscribe;
pub(crate) use server::publish;
pub(crate) use server::Server;

const BODY_LEN: usize = 18;

#[derive(Clone, Debug)]
pub struct ElementRequest {
    channel_key: ChannelKey,
    batch_pull_size: u16,
}

impl Into<BytesMut> for ElementRequest {
    fn into(self) -> BytesMut {
        let mut buffer = BytesMut::with_capacity(4 + BODY_LEN);
        buffer.put_u32(18); // (4 + 2 + 2) + (4 + 2 + 2) + 2
        buffer.put_u32(self.channel_key.source_task_id.job_id.0);
        buffer.put_u16(self.channel_key.source_task_id.task_number);
        buffer.put_u16(self.channel_key.source_task_id.num_tasks);
        buffer.put_u32(self.channel_key.target_task_id.job_id.0);
        buffer.put_u16(self.channel_key.target_task_id.task_number);
        buffer.put_u16(self.channel_key.target_task_id.num_tasks);
        buffer.put_u16(self.batch_pull_size);

        buffer
    }
}

impl TryFrom<BytesMut> for ElementRequest {
    type Error = anyhow::Error;

    fn try_from(mut buffer: BytesMut) -> Result<Self, Self::Error> {
        let len = buffer.get_u32(); // skip header length
        if len as usize != BODY_LEN {
            return Err(anyhow!(
                "Illegal request body length, expect {}, found {}",
                BODY_LEN,
                len
            ));
        }

        let source_task_id = {
            let job_id = buffer.get_u32();
            let task_number = buffer.get_u16();
            let num_tasks = buffer.get_u16();
            TaskId {
                job_id: JobId(job_id),
                task_number,
                num_tasks,
            }
        };

        let target_task_id = {
            let job_id = buffer.get_u32();
            let task_number = buffer.get_u16();
            let num_tasks = buffer.get_u16();
            TaskId {
                job_id: JobId(job_id),
                task_number,
                num_tasks,
            }
        };

        let batch_pull_size = buffer.get_u16();

        Ok(ElementRequest {
            channel_key: ChannelKey {
                source_task_id,
                target_task_id,
            },
            batch_pull_size,
        })
    }
}

/// Response code
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResponseCode {
    /// unknown code
    Unknown = 0,
    /// for per user data package
    Ok = 1,
    /// after the special batch, then send a finish batch package with the `BatchFinish` code
    BatchFinish = 2,
    /// there is no data in the channel, then send a package with the `Empty` code
    Empty = 3,
    /// parse the Request error, then send a package with the `ParseErr` code
    ParseErr = 4,
    /// read the Request error, then send a package with the `ReadErr` code
    ReadErr = 5,
}

impl From<u8> for ResponseCode {
    fn from(v: u8) -> Self {
        match v {
            1 => ResponseCode::Ok,
            2 => ResponseCode::BatchFinish,
            3 => ResponseCode::Empty,
            4 => ResponseCode::ParseErr,
            5 => ResponseCode::ReadErr,
            _ => ResponseCode::Unknown,
        }
    }
}

impl std::fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCode::Ok => write!(f, "OK"),
            ResponseCode::BatchFinish => write!(f, "BatchFinish"),
            ResponseCode::Empty => write!(f, "Empty"),
            ResponseCode::ParseErr => write!(f, "ParseErr"),
            ResponseCode::ReadErr => write!(f, "ReadErr"),
            ResponseCode::Unknown => write!(f, "Unknown"),
        }
    }
}
