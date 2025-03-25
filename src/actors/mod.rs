use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::protocol::Message;

/// Actor trait定义了所有actors必须实现的基本功能
#[async_trait]
pub trait Actor {
    /// 获取actor的名称
    fn name(&self) -> &str;
    
    /// 获取actor的类型
    fn type_name(&self) -> &str;
    
    /// 处理接收到的消息
    async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>>;
}

/// Root Actor实现
pub mod root {
    use super::*;

    #[derive(Debug)]
    pub struct RootActor {
        name: String,
    }

    impl RootActor {
        pub fn new() -> Self {
            Self {
                name: "root".to_string(),
            }
        }
    }

    #[async_trait]
    impl Actor for RootActor {
        fn name(&self) -> &str {
            &self.name
        }

        fn type_name(&self) -> &str {
            "root"
        }

        async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>> {
            // 处理root actor特定的消息
            match msg.content.get("type").and_then(Value::as_str) {
                Some("listTabs") => {
                    // 返回可用的tabs列表
                    let response = Message {
                        from: self.name().to_string(),
                        to: None,
                        content: serde_json::json!({
                            "type": "tabList",
                            "tabs": [],
                            "selected": 0
                        }),
                    };
                    Ok(Some(response))
                }
                _ => Ok(None),
            }
        }
    }
}

/// Tab Actor实现
pub mod tab {
    use super::*;

    #[derive(Debug)]
    pub struct TabActor {
        name: String,
        title: String,
        url: String,
    }

    impl TabActor {
        pub fn new(name: String, title: String, url: String) -> Self {
            Self { name, title, url }
        }
    }

    #[async_trait]
    impl Actor for TabActor {
        fn name(&self) -> &str {
            &self.name
        }

        fn type_name(&self) -> &str {
            "tab"
        }

        async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>> {
            // 处理tab actor特定的消息
            match msg.content.get("type").and_then(Value::as_str) {
                Some("attach") => {
                    // 处理attach请求
                    let response = Message {
                        from: self.name().to_string(),
                        to: None,
                        content: serde_json::json!({
                            "type": "attached",
                            "threadActor": format!("{}-thread", self.name),
                        }),
                    };
                    Ok(Some(response))
                }
                _ => Ok(None),
            }
        }
    }
} 