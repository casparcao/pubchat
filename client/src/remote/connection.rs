use anyhow::Result;

use log::{error, info};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use core::{response::ApiErr};
use pubchat::core::codec::{encode, decode};
use core::api::client::connection_host;
use crate::{cache};
use pubchat::core::message::{Cort, Message, Type}; 


// 使用token建立TCP连接
pub async fn connect_with_token(token: &str) -> Result<(TcpStream, u64, String)> {
    let mut stream = TcpStream::connect(connection_host())
        .await?;
    // 创建连接请求消息
    let connect_request = Message {
        id: 1,
        ts: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        mtype: Type::Cort as i32,
        content: Some(pubchat::core::message::message::Content::Cort(Cort {
            token: token.to_string(),
        })),
    };
    // 发送连接请求
    let encoded = encode(&connect_request)?;
    stream.write_all(&encoded).await?; 
    // 读取连接响应
    let response = decode::<Message, _>(&mut stream).await?;
    // 检查连接响应是否成功并获取用户ID
    let (user_id, uname) = if let Some(pubchat::core::message::message::Content::Cors(resp)) = response.content {
        if resp.code == 0 {
            info!("Connection established successfully, user ID: {}", resp.uid);
            (resp.uid, resp.uname) // 返回用户ID
        } else {
            error!("Connection failed: {}", resp.message);
            return Err(ApiErr::Error(format!("Connection failed: {}", resp.message)).into());
        }
    } else {
        error!("Invalid connection response");
        return Err(ApiErr::Error("Invalid connection response".to_string()).into());
    };
    Ok((stream, user_id, uname))
}

// 接收消息的异步任务
pub async fn receive_messages(mut reader: tokio::net::tcp::OwnedReadHalf, sx: tokio::sync::mpsc::Sender<core::api::types::message::Message>) {
    // 启动接收消息的任务
    tokio::spawn(async move {
        loop {
            match decode::<Message, _>(&mut reader).await {
                Ok(msg) => {
                    log::info!("Received message: {:?}", msg);
                    // 处理接收到的消息
                    match msg.mtype {
                        t if t == Type::Chrs as i32 => {
                            if let Some(pubchat::core::message::message::Content::Chrs(chat)) = msg.content {
                                //存储
                                //聊天缓存
                                //ui
                                if let Some(m) = chat.message {
                                    let msg = core::api::types::message::Message{
                                        id: snowflaker::next_id().unwrap() as i64,
                                        sender: chat.sender as i64,
                                        receiver: chat.receiver as i64,
                                        session: chat.session as i64,
                                        mtype: chat.ctype as i32,
                                        content: format!("{}", m),
                                        timestamp: chat.ts as i64,
                                        uname: chat.uname.clone(),
                                    };
                                    cache::message_cache().async_add_message(chat.session as i64, msg.clone()).await;
                                    let _ = sx.send(msg).await;
                                }else{
                                    error!("Invalid chat message");
                                }
                                
                            }else{
                                error!("Invalid chat message2");
                            }
                        }
                        _ => {
                            // 其他类型消息暂不处理
                            error!("Unhandled message type: {}", msg.mtype);
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