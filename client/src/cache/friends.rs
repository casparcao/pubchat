// 实现好友列表的三级缓存：内存 -> SQLite -> 远程服务器
use std::sync::Arc;
use std::sync::RwLock;
use anyhow::Result;
use crate::service::friend::{get_friends, FriendResponse};

/// 三级缓存结构
pub struct Cache {
    // 第一级：内存缓存
    memory_cache: Arc<RwLock<Option<Vec<FriendResponse>>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            memory_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// 从三级缓存中获取好友列表
    /// 1. 首先检查内存缓存
    /// 2. 然后检查SQLite数据库
    /// 3. 最后从远程服务器获取
    pub fn get_friends(&self, token: &str) -> Result<Vec<FriendResponse>> {
        // 1. 检查内存缓存
        {
            let cache = self.memory_cache.read().unwrap();
            if let Some(friends) = cache.as_ref() {
                return Ok(friends.clone());
            }
        }

        // 2. 检查SQLite数据库
        match self.get_from_sqlite(){
            Ok(friends) => {
                // 将结果存入内存缓存
                {
                    let mut cache = self.memory_cache.write().unwrap();
                    *cache = Some(friends.clone());
                }
                return Ok(friends);
            }
            Err(e) => {
                log::error!("从SQLite获取好友列表失败: {}", e);
            }
        }

        // 3. 从远程服务器获取
        match self.get_from_remote(token){
            Ok(friends) => {
                // 将结果存入内存缓存和SQLite数据库
                {
                    let mut cache = self.memory_cache.write().unwrap();
                    *cache = Some(friends.clone());
                }
                
                // 保存到SQLite
                if let Err(e) = self.save_to_sqlite(&friends) {
                    log::error!("保存好友列表到SQLite失败: {}", e);
                }
                
                Ok(friends)
            }
            Err(e) => {
                log::error!("从远程服务器获取好友列表失败: {}", e);
                // 如果所有缓存级别都失败，返回空列表
                Ok(vec![])
            }
        }
    }

    /// 从SQLite数据库获取好友列表
    fn get_from_sqlite(&self) -> Result<Vec<FriendResponse>> {
        let rt = tokio::runtime::Runtime::new()?;
        let friends = rt.block_on(crate::repository::friend::select_friends())?;
        Ok(friends)
    }

    /// 将好友列表保存到SQLite数据库
    fn save_to_sqlite(&self, friends: &[FriendResponse]) -> Result<()> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(crate::repository::friend::save_friends(friends))
    }

    /// 从远程服务器获取好友列表
    fn get_from_remote(&self, token: &str) -> Result<Vec<FriendResponse>> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(get_friends(token))
    }

    /// 清除好友列表缓存
    pub fn invalidate_cache(&self) {
        // 清除内存缓存
        {
            let mut cache = self.memory_cache.write().unwrap();
            *cache = None;
        }
        
        // 注意：在实际应用中，您可能还需要清除SQLite中的相关数据
        // 或者标记为需要更新
    }
}