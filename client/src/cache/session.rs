// 缓存ui聊天框的聊天列表
// 实现三级缓存：内存 -> SQLite -> 远程服务器

use core::api::client::session::create_session;
use core::api::client::session::get_session as get_session_from_remote;
use core::api::client::session::get_sessions as get_sessions_from_remote;
use core::api::types::session::CreateSessionRequest;
use core::api::types::session::SessionDetailResponse;
use core::api::types::session::SessionResponse;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use anyhow::Ok;
use anyhow::Result;
use core::request::Page;


// 三级缓存结构
pub struct Cache {
    // 第一级：内存缓存
    memory_cache: Arc<RwLock<Vec<SessionResponse>>>,
    session_detail_cache: Arc<RwLock<HashMap<i64, SessionDetailResponse>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            memory_cache: Arc::new(RwLock::new(vec![])),
            session_detail_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 从三级缓存中获取消息
    /// 1. 首先检查内存缓存
    /// 2. 然后检查SQLite数据库
    /// 3. 最后从远程服务器获取
    pub fn get_sessions(&self, token: &str, _page: Page) -> Result<Vec<SessionResponse>> {
        // 1. 检查内存缓存
        {
            let cache = self.memory_cache.read().unwrap();
            // 对于内存缓存，我们简单地返回所有消息
            // 在实际应用中，可能需要根据分页参数进行处理
            if !cache.is_empty() {
                return Ok(cache.clone());
            }
        }

        // 2. 检查SQLite数据库
        // match self.get_from_db(uid, page){
        //     std::result::Result::Ok(messages) => {
        //         // 将结果存入内存缓存
        //         {
        //             let mut cache = self.memory_cache.write().unwrap();
        //             *cache = messages.clone();
        //         }
        //         // 只有当联系人列表不为空时才使用数据库缓存
        //         if !messages.is_empty() {
        //             log::info!("从SQLite缓存获取列表成功");
        //             return Ok(messages);
        //         }
        //         // 如果联系人列表为空，继续尝试从远程获取
        //         log::info!("SQLite中数据，尝试从远程服务器获取");
        //     }
        //     Err(e) => {
        //         log::error!("从SQLite获取消息失败: {}", e);
        //     }
        // }

        // 3. 从远程服务器获取
        match get_sessions_from_remote(token){
            std::result::Result::Ok(sessions) => {
                // 将结果存入内存缓存和SQLite数据库
                {
                    let mut cache = self.memory_cache.write().unwrap();
                    *cache = sessions.clone();
                }
                
                // 异步保存到SQLite，不阻塞当前操作
                // if let Err(e) = self.save_to_sqlite(uid, &sessions) {
                //     log::error!("保存消息到SQLite失败: {}", e);
                // }
                Ok(sessions)
            }
            Err(e) => {
                log::error!("从远程服务器获取消息失败: {}", e);
                // 如果所有缓存级别都失败，返回空列表
                Ok(vec![])
            }
        }
    }

    pub fn get_session(&self, token: &str, id: i64) -> Result<SessionDetailResponse> {
        // 1. 检查内存缓存
        {
            let cache = self.session_detail_cache.read().unwrap();
            if let Some(detail) = cache.get(&id) {
                return Ok(detail.clone());
            }
        }

        match get_session_from_remote(token, id){
            std::result::Result::Ok(session) => {
                // 将结果存入内存缓存和SQLite数据库
                {
                    let mut cache = self.session_detail_cache.write().unwrap();
                    cache.insert(id, session.clone());
                }
                
                // 异步保存到SQLite，不阻塞当前操作
                // if let Err(e) = self.save_to_sqlite(uid, &sessions) {
                //     log::error!("保存消息到SQLite失败: {}", e);
                // }
                Ok(session)
            }
            Err(e) => {
                log::error!("从远程服务器获session失败: {}", e);
                // 如果所有缓存级别都失败，返回空列表
                Err(e)
            }
        }
    }

    /// 从SQLite数据库获取消息
    // fn get_from_db(&self, uid: i64, page: Page) -> Result<Vec<SessionResponse>> {
    //     let rt = tokio::runtime::Runtime::new()?;
    //     let contacts = rt.block_on(crate::repository::session::select(uid))?;
    //     let contacts = contacts.into_iter().map(|contact| SessionResponse {
    //         id: contact.id,
    //         name: contact.name,
    //     }).collect::<Vec<_>>();
    //     Ok(contacts)
    // }

    /// 将消息保存到SQLite数据库
    // fn save_to_sqlite(&self, uid: i64, sessions: &[SessionResponse]) -> Result<()> {
    //     let rt = tokio::runtime::Runtime::new()?;
    //     let sessions = sessions.iter().map(|session| crate::repository::session::Session {
    //         id: session.id,
    //         uid: uid,
    //         sid: session.id,
    //         name: session.name.clone(),
    //         avatar: None,
    //     }).collect::<Vec<_>>();
    //     rt.block_on(crate::repository::session::save(uid, &sessions));
    //     Ok(())
    // }


    /// 向缓存中添加单条消息
    pub fn add_session(&self, token: &str, payload: CreateSessionRequest) -> Result<SessionResponse> {
        // 添加到内存缓存
        {
            let mut cache = self.memory_cache.write().unwrap();
            let message = SessionResponse{
                id: payload.id,
                name: payload.name.clone()
            };
            cache.push(message);
        }
        // 保存到SQLite
        // if let Err(e) = self.save_to_sqlite(&message){
            // log::error!("保存消息到SQLite失败: {}", e);
        // }
        create_session(token, payload)
    }

}