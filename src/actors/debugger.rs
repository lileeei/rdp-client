use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::protocol::Message;
use super::Actor;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub line: u32,
    pub column: u32,
    pub source_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Breakpoint {
    pub id: String,
    pub location: Location,
    pub condition: Option<String>,
    pub enabled: bool,
}

#[derive(Debug)]
pub struct DebuggerActor {
    name: String,
    breakpoints: HashMap<String, Breakpoint>,
    paused: bool,
    current_frame: Option<Location>,
}

impl DebuggerActor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            breakpoints: HashMap::new(),
            paused: false,
            current_frame: None,
        }
    }

    pub fn add_breakpoint(&mut self, breakpoint: Breakpoint) {
        self.breakpoints.insert(breakpoint.id.clone(), breakpoint);
    }

    pub fn remove_breakpoint(&mut self, id: &str) -> Option<Breakpoint> {
        self.breakpoints.remove(id)
    }

    pub fn set_paused(&mut self, paused: bool, location: Option<Location>) {
        self.paused = paused;
        self.current_frame = location;
    }
}

#[async_trait]
impl Actor for DebuggerActor {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_name(&self) -> &str {
        "debugger"
    }

    async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>> {
        match msg.content.get("type").and_then(Value::as_str) {
            Some("setBreakpoint") => {
                if let Ok(breakpoint) = serde_json::from_value(msg.content.get("breakpoint").unwrap_or(&Value::Null).clone()) {
                    self.add_breakpoint(breakpoint);
                    Ok(Some(Message {
                        from: self.name().to_string(),
                        to: msg.from,
                        content: serde_json::json!({
                            "type": "breakpointAdded",
                            "breakpoints": self.breakpoints.values().collect::<Vec<_>>(),
                        }),
                    }))
                } else {
                    Ok(None)
                }
            }
            Some("removeBreakpoint") => {
                if let Some(id) = msg.content.get("id").and_then(Value::as_str) {
                    if let Some(breakpoint) = self.remove_breakpoint(id) {
                        Ok(Some(Message {
                            from: self.name().to_string(),
                            to: msg.from,
                            content: serde_json::json!({
                                "type": "breakpointRemoved",
                                "id": breakpoint.id,
                            }),
                        }))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            Some("pause") => {
                self.set_paused(true, None);
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "paused",
                        "why": { "type": "clientRequest" },
                    }),
                }))
            }
            Some("resume") => {
                self.set_paused(false, None);
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "resumed",
                    }),
                }))
            }
            Some("stepOver") => {
                // TODO: 实现单步执行
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "stepped",
                        "why": { "type": "stepOver" },
                    }),
                }))
            }
            Some("stepIn") => {
                // TODO: 实现步入
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "stepped",
                        "why": { "type": "stepIn" },
                    }),
                }))
            }
            Some("stepOut") => {
                // TODO: 实现步出
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "stepped",
                        "why": { "type": "stepOut" },
                    }),
                }))
            }
            Some("frames") => {
                // TODO: 实现调用栈帧获取
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "frames",
                        "frames": [],
                    }),
                }))
            }
            _ => Ok(None),
        }
    }
} 