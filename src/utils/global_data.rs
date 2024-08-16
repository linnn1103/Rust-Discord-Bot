use std::{
    sync::Arc,
    collections::HashMap,
};
use serenity::{
    all::{ChannelId, UserId},
    prelude::*,
};

/// 指令計數器 hashmap <指令名稱, 指令次數>
pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = Arc<RwLock<HashMap<String, u64>>>;
}

/// 動態語音頻道與所有權者 hashmap <動態語音頻道 ID, 權限者 ID>
pub struct VoiceChannelId;

impl TypeMapKey for VoiceChannelId {
    type Value = Arc<RwLock<HashMap<ChannelId, UserId>>>;
}


/// 加密貨幣查詢紀錄 hashmap <查詢幣種, 查詢結果>
pub struct Cryptoprice;

impl TypeMapKey for Cryptoprice {
    type Value = Arc<RwLock<HashMap<String, String>>>;   
}