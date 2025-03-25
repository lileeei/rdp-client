use std::net::TcpStream;
use std::collections::HashMap;
use anyhow::Result;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::protocol::{JsonPacketStream, Message};
use crate::actors::{
    Actor,
    console::ConsoleActor,
    debugger::DebuggerActor,
    network::NetworkActor,
    root::RootActor,
    tab::TabActor,
};

pub struct DebugClient {
    stream: TcpStream,
    actors: HashMap<String, Box<dyn Actor + Send>>,
    message_tx: mpsc::Sender<Message>,
}

impl DebugClient {
    /// 创建新的调试客户端连接
    pub async fn connect(host: &str, port: u16) -> Result<Self> {
        let stream = TcpStream::connect((host, port))?;
        stream.set_nonblocking(true)?;
        
        let (tx, mut rx) = mpsc::channel(100);
        
        let mut client = Self {
            stream,
            actors: HashMap::new(),
            message_tx: tx,
        };

        // 初始化基本actors
        client.init_actors();

        // 启动消息处理循环
        let actors = client.actors.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Some(actor) = msg.to.as_ref().and_then(|to| actors.get(to)) {
                    if let Ok(Some(response)) = actor.handle_message(msg).await {
                        // TODO: 发送响应
                        log::debug!("Actor response: {:?}", response);
                    }
                }
            }
        });

        Ok(client)
    }

    /// 初始化基本actors
    fn init_actors(&mut self) {
        // 添加root actor
        let root_actor = RootActor::new();
        self.actors.insert("root".to_string(), Box::new(root_actor));

        // 添加console actor
        let console_actor = ConsoleActor::new(format!("console-{}", Uuid::new_v4()));
        self.actors.insert(console_actor.name().to_string(), Box::new(console_actor));

        // 添加debugger actor
        let debugger_actor = DebuggerActor::new(format!("debugger-{}", Uuid::new_v4()));
        self.actors.insert(debugger_actor.name().to_string(), Box::new(debugger_actor));

        // 添加network actor
        let network_actor = NetworkActor::new(format!("network-{}", Uuid::new_v4()));
        self.actors.insert(network_actor.name().to_string(), Box::new(network_actor));
    }

    /// 发送消息到服务器
    pub async fn send_message(&mut self, msg: Message) -> Result<()> {
        self.stream.write_json_packet(&msg)?;
        Ok(())
    }

    /// 接收服务器消息
    pub async fn receive_message(&mut self) -> Result<Option<Message>> {
        match self.stream.read_json_packet()? {
            Some(value) => {
                let msg: Message = serde_json::from_value(value)?;
                
                // 如果消息有目标actor，转发给对应的actor处理
                if let Some(to) = msg.to.as_ref() {
                    if let Some(actor) = self.actors.get_mut(to) {
                        if let Ok(Some(response)) = actor.handle_message(msg.clone()).await {
                            self.send_message(response).await?;
                        }
                    }
                }
                
                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    /// 创建新的tab
    pub async fn create_tab(&mut self, url: String) -> Result<String> {
        let tab_id = format!("tab-{}", Uuid::new_v4());
        let tab_actor = TabActor::new(
            tab_id.clone(),
            "New Tab".to_string(),
            url,
        );
        self.actors.insert(tab_id.clone(), Box::new(tab_actor));
        Ok(tab_id)
    }

    /// 获取actor
    pub fn get_actor(&self, name: &str) -> Option<&(dyn Actor + Send)> {
        self.actors.get(name).map(|actor| actor.as_ref())
    }

    /// 获取actor（可变）
    pub fn get_actor_mut(&mut self, name: &str) -> Option<&mut (dyn Actor + Send)> {
        self.actors.get_mut(name).map(|actor| actor.as_mut())
    }
} 