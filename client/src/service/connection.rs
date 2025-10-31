use anyhow::Result;

use log::{error, info};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use core::{proto::message::{ConnectRequest, Message, Type}, response::ApiErr};
use core::proto::codec::{encode, decode};

use crate::service::connection_host;


// 使用token建立TCP连接
pub async fn connect_with_token(token: &str) -> Result<(TcpStream, u64)> {
    let mut stream = TcpStream::connect(connection_host())
        .await?;
    // 创建连接请求消息
    let connect_request = Message {
        id: 1,
        ts: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        r#type: Type::ConnectRequest as i32,
        content: Some(core::proto::message::message::Content::ConnectRequest(ConnectRequest {
            token: token.to_string(),
        })),
    };
    // 发送连接请求
    let encoded = encode(&connect_request)?;
    stream.write_all(&encoded).await?; 
    // 读取连接响应
    let response = decode::<Message, _>(&mut stream).await?;
    // 检查连接响应是否成功并获取用户ID
    let user_id = if let Some(core::proto::message::message::Content::ConnectResponse(resp)) = response.content {
        if resp.code == 0 {
            info!("Connection established successfully, user ID: {}", resp.uid);
            resp.uid // 返回用户ID
        } else {
            error!("Connection failed: {}", resp.message);
            return Err(ApiErr::Error(format!("Connection failed: {}", resp.message)).into());
        }
    } else {
        error!("Invalid connection response");
        return Err(ApiErr::Error("Invalid connection response".to_string()).into());
    };
    Ok((stream, user_id))
}

// 接收消息的异步任务
pub async fn receive_messages(mut reader: tokio::net::tcp::OwnedReadHalf) {
    // 启动接收消息的任务
    tokio::spawn(async move {
        loop {
            match decode::<Message, _>(&mut reader).await {
                Ok(msg) => {
                    // 处理接收到的消息
                    match msg.r#type {
                        t if t == Type::Chat as i32 => {
                            if let Some(core::proto::message::message::Content::Chat(chat_req)) = msg.content {
                                //存储
                                //聊天缓存
                                //ui
                            }else{
                                error!("Invalid chat message");
                            }
                        }
                        _ => {
                            // 其他类型消息暂不处理
                            error!("Unhandled message type: {}", msg.r#type);
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                }
            }
        }
    });
}