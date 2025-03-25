# Remote Debug Protocol Client (rdp-client)

> ⚠️ **警告：项目开发中** ⚠️
> 
> 本项目目前处于积极开发阶段，API和功能可能会发生重大变更。建议仅用于测试和学习目的，请谨慎用于生产环境。

一个用Rust实现的远程调试协议客户端，支持与遵循Firefox DevTools Protocol的调试服务器进行通信。本项目参考了 [Servo](https://github.com/servo/servo) 浏览器引擎中的 devtools 实现，采用相同的 Actor 架构设计和消息传递机制。

## 功能特性

- 基于TCP的JSON协议实现
- 异步消息处理
- Actor系统架构
- 支持多种调试功能：
  - Tab管理
  - 调试会话控制
  - 消息收发
  - Actor生命周期管理

## 项目结构

```
rdp-client/
├── src/
│   ├── protocol/    # 协议实现
│   ├── client/      # 客户端核心实现
│   ├── actors/      # Actor系统实现
│   └── main.rs      # 程序入口
├── Cargo.toml       # 项目配置和依赖
└── README.md        # 项目文档
```

## 核心组件

### Protocol

协议层实现了基本的JSON数据包传输功能：

- `JsonPacketStream` trait：定义了JSON数据包的读写接口
- `Message` 结构：定义了基本的消息格式
- `ActorDescription`：描述Actor的能力和方法

### Client

客户端层管理与调试服务器的连接：

- 连接管理
- 消息发送和接收
- Actor注册和查找
- 异步消息处理

### Actors

Actor系统实现了不同类型的调试功能：

- `RootActor`：管理全局状态和Tab列表
- `TabActor`：管理单个Tab的调试会话
- 可扩展的Actor trait系统

## 快速开始

### 环境要求

- Rust 1.56.0 或更高版本
- Cargo包管理器

### 安装

```bash
# 克隆项目
git clone [项目地址]
cd rdp-client

# 构建项目
cargo build
```

### 运行

```bash
# 设置日志级别并运行
RUST_LOG=debug cargo run
```

### 配置

默认配置：
- 调试服务器地址：127.0.0.1
- 端口：6000

可以通过修改 `main.rs` 中的相关参数来更改这些配置。

## 使用示例

```rust
use rdp_client::{DebugClient, Message};

#[tokio::main]
async fn main() -> Result<()> {
    // 创建客户端实例
    let mut client = DebugClient::connect("127.0.0.1", 6000).await?;

    // 发送连接请求
    let init_msg = Message {
        from: "root".to_string(),
        to: None,
        content: serde_json::json!({
            "type": "connect",
            "traits": ["browser"]
        }),
    };
    client.send_message(init_msg).await?;

    // 处理响应
    while let Some(msg) = client.receive_message().await? {
        println!("Received: {:?}", msg);
    }

    Ok(())
}
```

## 扩展开发

### 添加新的Actor

1. 在 `actors/` 目录下创建新的模块
2. 实现 `Actor` trait
3. 在 `DebugClient` 中注册新的Actor类型

示例：
```rust
#[async_trait]
impl Actor for MyNewActor {
    fn name(&self) -> &str {
        &self.name
    }

    fn type_name(&self) -> &str {
        "myNewActor"
    }

    async fn handle_message(&mut self, msg: Message) -> Result<Option<Message>> {
        // 实现消息处理逻辑
    }
}
```

## 待实现功能

- [ ] 控制台消息处理
- [ ] 网络请求监控
- [ ] 性能分析
- [ ] 断点管理
- [ ] 变量查看和修改
- [ ] 调用栈分析

## 贡献指南

欢迎提交Issue和Pull Request！

1. Fork项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建Pull Request

## 许可证

[MIT License](LICENSE)

## 联系方式

- 作者：[lileeei]
- 邮箱：[lileiwell1993@gmail.com]
