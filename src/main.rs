mod protocol;
mod client;
mod actors;

use anyhow::Result;
use client::DebugClient;
use protocol::Message;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();

    // 连接到调试服务器
    let mut client = DebugClient::connect("127.0.0.1", 6000).await?;

    // 发送初始化消息
    let init_msg = Message {
        from: "root".to_string(),
        to: None,
        content: serde_json::json!({
            "type": "connect",
            "traits": ["browser"]
        }),
    };

    client.send_message(init_msg).await?;

    // 创建一个新的tab
    let tab_id = client.create_tab("https://example.com".to_string()).await?;
    log::info!("Created new tab with ID: {}", tab_id);

    // 设置断点示例
    let breakpoint_msg = Message {
        from: "root".to_string(),
        to: Some(format!("debugger-{}", tab_id)),
        content: serde_json::json!({
            "type": "setBreakpoint",
            "breakpoint": {
                "id": "bp1",
                "location": {
                    "line": 10,
                    "column": 1,
                    "source_id": "main.js"
                },
                "condition": null,
                "enabled": true
            }
        }),
    };
    client.send_message(breakpoint_msg).await?;

    // 监听网络请求示例
    let network_msg = Message {
        from: "root".to_string(),
        to: Some(format!("network-{}", tab_id)),
        content: serde_json::json!({
            "type": "startListeners"
        }),
    };
    client.send_message(network_msg).await?;

    // 模拟网络请求
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    
    let request = actors::network::NetworkRequest::new(
        "req1".to_string(),
        "https://api.example.com/data".to_string(),
        "GET".to_string(),
        headers,
    );

    if let Some(network_actor) = client.get_actor_mut(&format!("network-{}", tab_id)) {
        if let Some(actor) = network_actor.downcast_mut::<actors::network::NetworkActor>() {
            actor.add_request(request);
        }
    }

    // 等待并处理响应
    while let Some(msg) = client.receive_message().await? {
        log::info!("Received message: {:?}", msg);
        
        // 根据消息类型处理不同的响应
        if let Some(msg_type) = msg.content.get("type").and_then(|v| v.as_str()) {
            match msg_type {
                "connected" => {
                    log::info!("Successfully connected to debug server");
                }
                "breakpointAdded" => {
                    log::info!("Breakpoint added successfully");
                }
                "paused" => {
                    log::info!("Execution paused at breakpoint");
                }
                "networkRequest" => {
                    log::info!("Network request detected");
                }
                _ => {
                    log::debug!("Unhandled message type: {}", msg_type);
                }
            }
        }
    }

    Ok(())
}
