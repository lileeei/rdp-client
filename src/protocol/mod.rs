use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;
use anyhow::Result;

/// 基本的JSON数据包特征
pub trait JsonPacketStream {
    fn write_json_packet<T: Serialize>(&mut self, obj: &T) -> Result<()>;
    fn read_json_packet(&mut self) -> Result<Option<serde_json::Value>>;
}

impl JsonPacketStream for TcpStream {
    fn write_json_packet<T: Serialize>(&mut self, obj: &T) -> Result<()> {
        let s = serde_json::to_string(obj)?;
        log::debug!("-> {}", s);
        write!(self, "{}:{}", s.len(), s)?;
        Ok(())
    }

    fn read_json_packet(&mut self) -> Result<Option<serde_json::Value>> {
        let mut buffer = vec![];
        loop {
            let mut buf = [0];
            let byte = match self.read(&mut buf) {
                Ok(0) => return Ok(None), // EOF
                Ok(1) => buf[0],
                Ok(_) => unreachable!(),
                Err(e) => return Err(e.into()),
            };
            match byte {
                b':' => {
                    let packet_len_str = String::from_utf8(buffer)?;
                    let packet_len = packet_len_str.parse::<u64>()?;
                    let mut packet = String::new();
                    self.take(packet_len).read_to_string(&mut packet)?;
                    log::debug!("<- {}", packet);
                    return Ok(Some(serde_json::from_str(&packet)?));
                },
                c => buffer.push(c),
            }
        }
    }
}

/// 基本的Actor描述
#[derive(Debug, Serialize, Deserialize)]
pub struct ActorDescription {
    pub category: String,
    pub type_name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub request: serde_json::Value,
    pub response: serde_json::Value,
}

/// 基本的消息结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub from: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(flatten)]
    pub content: serde_json::Value,
}

/// 错误类型
#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
} 