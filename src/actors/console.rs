use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::protocol::Message;
use super::Actor;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleMessage {
    pub level: String,
    pub text: String,
    pub timestamp: u64,
    pub filename: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
}

#[derive(Debug)]
pub struct ConsoleActor {
    name: String,
    messages: Vec<ConsoleMessage>,
}

impl ConsoleActor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: ConsoleMessage) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &[ConsoleMessage] {
        &self.messages
    }
}

#[async_trait]
impl Actor for ConsoleActor {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_name(&self) -> &str {
        "console"
    }

    async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>> {
        match msg.content.get("type").and_then(Value::as_str) {
            Some("startListeners") => {
                // 开始监听控制台消息
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "listenersStarted",
                        "nativeConsoleAPI": true,
                    }),
                }))
            }
            Some("getCachedMessages") => {
                // 返回缓存的消息
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "cachedMessages",
                        "messages": self.messages,
                    }),
                }))
            }
            Some("evaluateJS") => {
                // 处理JavaScript表达式评估
                if let Some(expr) = msg.content.get("expr").and_then(Value::as_str) {
                    Ok(Some(Message {
                        from: self.name().to_string(),
                        to: msg.from,
                        content: serde_json::json!({
                            "type": "evaluationResult",
                            "result": self.evaluate_js(expr)?,
                        }),
                    }))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

impl ConsoleActor {
    fn evaluate_js(&self, expr: &str) -> Result<Value> {
        // TODO: 实现JavaScript表达式评估
        // 这里需要集成一个JavaScript引擎，比如v8或deno_core
        Ok(serde_json::json!({
            "type": "undefined",
            "value": null
        }))
    }
} 