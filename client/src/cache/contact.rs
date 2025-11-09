use core::api::types::contact::ContactResponse;
// 实现好友列表的三级缓存：内存 -> SQLite -> 远程服务器
use std::sync::Arc;
use std::sync::RwLock;
use anyhow::Result;
use core::api::client::contact::get_contacts as get_contacts_from_remote;

/// 三级缓存结构
pub struct Cache {
    // 第一级：内存缓存
    memory_cache: Arc<RwLock<Vec<ContactResponse>>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            memory_cache: Arc::new(RwLock::new(vec![])),
        }
    }

    /// 从三级缓存中获取好友列表
    /// 1. 首先检查内存缓存
    /// 2. 然后检查SQLite数据库
    /// 3. 最后从远程服务器获取
    pub fn get_contacts(&self, token: &str) -> Result<Vec<ContactResponse>> {
        // 1. 检查内存缓存
        {
            let cache = self.memory_cache.read().unwrap();
            if !cache.is_empty(){
                return Ok(cache.clone());
            }
        }

        // 2. 检查SQLite数据库
        // match self.get_from_db(uid){
        //     Ok(contacts) => {
        //         // 将结果存入内存缓存
        //         {
        //             let mut cache = self.memory_cache.write().unwrap();
        //             cache.insert(uid, contacts.clone());
        //         }
        //         // 只有当联系人列表不为空时才使用数据库缓存
        //         if !contacts.is_empty() {
        //             log::info!("从SQLite缓存获取联系人列表成功");
        //             return Ok(contacts);
        //         }
        //         // 如果联系人列表为空，继续尝试从远程获取
        //         log::info!("SQLite中无联系人数据，尝试从远程服务器获取");
        //     }
        //     Err(e) => {
        //         log::error!("从SQLite获取联系人列表失败: {}", e);
        //     }
        // }

        // 3. 从远程服务器获取
        match self.get_from_remote(token){
            Ok(contacts) => {
                // 将结果存入内存缓存和SQLite数据库
                {
                    let mut cache = self.memory_cache.write().unwrap();
                    *cache = contacts.clone();
                }
                
                // 保存到SQLite
                // if let Err(e) = self.save_to_db(uid, &contacts) {
                //     log::error!("保存联系人列表到SQLite失败: {}", e);
                // }
                // log::info!("从远程服务器获取联系人列表成功");
                Ok(contacts)
            }
            Err(e) => {
                log::error!("从远程服务器获取联系人列表失败: {}", e);
                // 如果所有缓存级别都失败，返回空列表
                Ok(vec![])
            }
        }
    }

    /// 从SQLite数据库获取好友列表
    // fn get_from_db(&self, uid: i64) -> Result<Vec<ContactResponse>> {
    //     let rt = tokio::runtime::Runtime::new()?;
    //     let contacts = rt.block_on(crate::repository::contact::select(uid))?;
    //     Ok(contacts)
    // }

    /// 将好友列表保存到SQLite数据库
    // fn save_to_db(&self, uid: i64, contacts: &[ContactResponse]) -> Result<()> {
    //     let rt = tokio::runtime::Runtime::new()?;
    //     rt.block_on(crate::repository::contact::save(uid, contacts))
    // }

    /// 从远程服务器获取好友列表
    fn get_from_remote(&self, token: &str) -> Result<Vec<ContactResponse>> {
        log::info!("从远程服务器获取联系人列表");
        get_contacts_from_remote(token)
    }

}