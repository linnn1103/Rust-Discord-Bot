use std::{
    sync::Arc,
    collections::HashMap,
};
use serenity::{
    all::{ChannelId, UserId},
    prelude::*,
};
pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = Arc<RwLock<HashMap<String, u64>>>;
}
pub struct VoiceChannelId;

impl TypeMapKey for VoiceChannelId {
    type Value = Arc<RwLock<HashMap<ChannelId, UserId>>>;
}
