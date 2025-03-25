use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::protocol::Message;
use super::Actor;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkRequest {
    pub request_id: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub timestamp: u64,
    pub status: Option<u16>,
    pub status_text: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub response_headers: Option<HashMap<String, String>>,
    pub response_body: Option<Vec<u8>>,
    pub duration: Option<u64>,
}

impl NetworkRequest {
    pub fn new(request_id: String, url: String, method: String, headers: HashMap<String, String>) -> Self {
        Self {
            request_id,
            url,
            method,
            headers,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            status: None,
            status_text: None,
            content_type: None,
            content_length: None,
            response_headers: None,
            response_body: None,
            duration: None,
        }
    }

    pub fn set_response(
        &mut self,
        status: u16,
        status_text: String,
        headers: HashMap<String, String>,
        body: Option<Vec<u8>>,
    ) {
        self.status = Some(status);
        self.status_text = Some(status_text);
        self.response_headers = Some(headers);
        self.response_body = body;
        self.duration = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
                - self.timestamp,
        );
    }
}

#[derive(Debug)]
pub struct NetworkActor {
    name: String,
    requests: HashMap<String, NetworkRequest>,
    listeners: Vec<String>,
}

impl NetworkActor {
    pub fn new(name: String) -> Self {
        Self {
            name,
            requests: HashMap::new(),
            listeners: Vec::new(),
        }
    }

    pub fn add_request(&mut self, request: NetworkRequest) {
        self.requests.insert(request.request_id.clone(), request);
        self.notify_request_started(&request.request_id);
    }

    pub fn update_request(&mut self, request_id: &str, status: u16, status_text: String, 
        headers: HashMap<String, String>, body: Option<Vec<u8>>) {
        if let Some(request) = self.requests.get_mut(request_id) {
            request.set_response(status, status_text, headers, body);
            self.notify_request_finished(request_id);
        }
    }

    fn notify_request_started(&self, request_id: &str) {
        if let Some(request) = self.requests.get(request_id) {
            // 通知所有监听器新请求开始
            for listener in &self.listeners {
                // TODO: 发送通知
                log::debug!("Notifying {} of request start: {}", listener, request_id);
            }
        }
    }

    fn notify_request_finished(&self, request_id: &str) {
        if let Some(request) = self.requests.get(request_id) {
            // 通知所有监听器请求完成
            for listener in &self.listeners {
                // TODO: 发送通知
                log::debug!("Notifying {} of request finish: {}", listener, request_id);
            }
        }
    }
}

#[async_trait]
impl Actor for NetworkActor {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_name(&self) -> &str {
        "network"
    }

    async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>> {
        match msg.content.get("type").and_then(Value::as_str) {
            Some("startListeners") => {
                self.listeners.push(msg.from.clone());
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "listenersStarted"
                    }),
                }))
            }
            Some("stopListeners") => {
                self.listeners.retain(|listener| *listener != msg.from);
                Ok(Some(Message {
                    from: self.name().to_string(),
                    to: msg.from,
                    content: serde_json::json!({
                        "type": "listenersStopped"
                    }),
                }))
            }
            Some("getRequestContent") => {
                if let Some(id) = msg.content.get("id").and_then(Value::as_str) {
                    if let Some(request) = self.requests.get(id) {
                        Ok(Some(Message {
                            from: self.name().to_string(),
                            to: msg.from,
                            content: serde_json::json!({
                                "type": "requestContent",
                                "id": id,
                                "content": {
                                    "request": {
                                        "url": request.url,
                                        "method": request.method,
                                        "headers": request.headers,
                                    },
                                    "response": {
                                        "status": request.status,
                                        "statusText": request.status_text,
                                        "headers": request.response_headers,
                                        "content": request.response_body,
                                    }
                                }
                            }),
                        }))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
} 