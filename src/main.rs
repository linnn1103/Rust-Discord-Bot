use num_format::{Locale, ToFormattedString};

use serenity::{
    all::{ActivityData, Channel, ChannelId, ChannelType, Command, CreateEmbed, UserId},
    async_trait,
    builder::{CreateChannel, CreateInteractionResponse, CreateInteractionResponseMessage},
    model::{
        application::{Interaction, ResolvedOption, ResolvedValue},
        channel::Message,
        colour,
        gateway::Ready,
        user::OnlineStatus,
        voice::VoiceState,
    },
    prelude::*,
};
use std::{collections::HashMap, sync::Arc, env};
mod commands;

mod utils;
use utils::global_data::{CommandCounter, VoiceChannelId, Cryptoprice};

/// 動態頻道的目標頻道ID
const TARGET_CHANNEL_ID: u64 = 1181486481823498290;
/// 創建的頻道會在這個頻道下
const CATEGORY_ID: u64 = 1179997203377434647;
/// 機器人的狀態
const BOT_STATE: &str = "男銅俱樂部";
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("你媽知道你在這裡說這種笑話嗎？") {
            if let Err(why) = msg
                .reply(&ctx.http, "<a:modcheck:1272777094044188673>")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content.contains("习近平") {
            if let Err(why) = msg
                .reply(&ctx.http, "https://memeprod.sgp1.digitaloceanspaces.com/user-resource/532c5d901c3b277097d7e1f4aae3d265.png")
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            
            let counter_lock = {
                let data_read = ctx.data.read().await;
                data_read.get::<CommandCounter>().expect("Expected CommandCounter in TypeMap.").clone()
            };
            {
                let mut counter = counter_lock.write().await;
                let entry = counter.entry(command.data.name.clone()).or_insert(0);
                *entry += 1;
            }
            let content = match command.data.name.as_str() {
                "求籤" => Some(commands::signature::run(&command.data.options())),
                "习近平万岁" => Some(commands::china::run(&command.data.options())),
                "dirty_talk" => Some(commands::dirty::run(&command.data.options())),
                "help" => Some(commands::help::run(&command.data.options())),
                "加密貨幣現貨" => {
                    if let Some(ResolvedOption {
                        value: ResolvedValue::String(symbol),
                        ..
                    }) = &command.data.options().first()
                    {
                        let price = get_cryptocurrency_price(symbol, ctx.clone()).await;
                        Some(commands::get_cryptocurrency_data::run(
                            &command.data.options(), symbol.to_string(), price),
                        )
                    } else {
                        Some(
                            CreateEmbed::default()
                                .color(colour::Colour::RED)
                                .description(
                                    "<a:emm:1272626653532786801> 起嗯找不到你要的這個幣種",
                                ),
                        )
                    }
                },
                "重新命名" => {
                    let user_id = {
                        let data_read = ctx.data.read().await;
                        let voice_channel_id_lock = data_read
                            .get::<VoiceChannelId>()
                            .expect("Expected VoiceChannelId in TypeMap.")
                            .clone();
                        let voice_channel_id = voice_channel_id_lock.read().await;
                        voice_channel_id
                            .iter()
                            .find(|(_, &val)| val == command.user.id)
                            .map(|(&key, _)| key)
                    };
                    Some(commands::renamed_voice_channel::run(
                        &command.data.options(),
                        user_id,
                        ctx.clone(),
                    ))
                }
                "指令計數器" => {
                    if let Some(ResolvedOption {
                        value: ResolvedValue::String(command_name),
                        ..
                    }) = &command.data.options().first()
                    {
                        let command_name = command_name.to_string();
                        let amount = {
                            let data_read = ctx.data.read().await;
                            let command_counter_lock = data_read
                                .get::<CommandCounter>()
                                .expect("Expected CommandCounter in TypeMap.")
                                .clone();
                            let command_counter = command_counter_lock.read().await;
                            command_counter.get(&command_name).map_or(0, |x| *x)
                        };
                        if command_name == "习近平万岁" {
                            Some(commands::command_counter::run(
                                &command.data.options(),
                                format!(
                                    "wc牛逼，`{}`被使用过{}次 <:xi:1272631608322555944>",
                                    command_name.to_string(),
                                    (amount + 1471101659).to_formatted_string(&Locale::en)
                                ),
                            ))
                        } else {
                            Some(commands::command_counter::run(
                                &command.data.options(),
                                format!(
                                    "`{}`被使用過{}次",
                                    command_name.to_string(),
                                    amount.to_formatted_string(&Locale::en)
                                ),
                            ))
                        }
                    } else {
                        Some(
                            CreateEmbed::default()
                                .color(colour::Colour::RED)
                                .description(
                                    "<a:emm:1272626653532786801> 起嗯找不到你要的這個指令",
                                ),
                        )
                    }
                }
                _ => Some(
                    CreateEmbed::default()
                        .color(colour::Colour::RED)
                        .description("<a:emm:1272626653532786801> 起嗯不懂這個指令"),
                ),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().embed(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if let Some(old_state) = old {
            if let Some(channel_id) = old_state.channel_id {
                let voice_channel_id_lock = ctx
                    .data
                    .read()
                    .await
                    .get::<VoiceChannelId>()
                    .expect("Expected VoiceChannelId in TypeMap.")
                    .clone();
                let mut voice_channel_id = voice_channel_id_lock.write().await;
                if voice_channel_id.contains_key(&channel_id) {
                    if let Ok(Channel::Guild(guild_channel)) =
                        channel_id.to_channel(&ctx.http).await
                    {
                        let member_count = guild_channel.members(&ctx.cache).unwrap().len();
                        if member_count == 0 {
                            let _ = channel_id.delete(&ctx.http).await;
                            voice_channel_id.retain(|&id, _| id != channel_id);
                        }
                    }
                }
            }
        }
        if new.channel_id.is_none() {
            return;
        } else if new.channel_id.clone().unwrap() == ChannelId::new(TARGET_CHANNEL_ID) {
            if let Some(guild_id) = new.guild_id {
                let channel_builder = CreateChannel::new(format!(
                    "{}的頻道",
                    new.member.clone().unwrap().nick.unwrap()
                ))
                .kind(ChannelType::Voice)
                .category(CATEGORY_ID);
                match guild_id.create_channel(&ctx.http, channel_builder).await {
                    Ok(channel) => {
                        let voice_channel_id_lock: Arc<RwLock<HashMap<ChannelId, UserId>>> = {
                            let data_read = ctx.data.read().await;
                            data_read
                                .get::<VoiceChannelId>()
                                .expect("Expected VoiceChannelId in TypeMap.")
                                .clone()
                        };
                        {
                            let mut voice_channel_id = voice_channel_id_lock.write().await;
                            if !voice_channel_id.contains_key(&channel.id) {
                                voice_channel_id.insert(channel.id, new.user_id);
                            }
                        };
                        let _ = guild_id
                            .move_member(&ctx.http, new.user_id, channel.id)
                            .await;
                    }
                    Err(why) => println!("Error creating channel: {:?}", why),
                }
            } else {
                println!("Guild ID is None");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let commands = vec![
            commands::signature::register(),
            commands::china::register(),
            commands::dirty::register(),
            commands::help::register(),
            commands::command_counter::register(),
            commands::renamed_voice_channel::register(),
            commands::get_cryptocurrency_data::register(),
        ];

        for command in commands {
            let _ = Command::create_global_command(&ctx.http, command).await;
        }
        ctx.set_presence(
            Some(ActivityData::watching(BOT_STATE)),
            OnlineStatus::Online,
        );
        println!("{} is connected!", ready.user.name);
    }
}

/// 透過 Binance API 獲取加密貨幣的價格
/// 因reqwest獲取資訊時要求block_on，
/// 在一個已經運行的異步環境中嘗試使用 Runtime::block_on，這會導致衝突。
/// 因此將獲取價格的邏輯放在這
async fn get_cryptocurrency_price(symbol: &str, ctx: Context) -> (String,String) {
    let url = format!("https://api3.binance.com/api/v3/ticker/price?symbol={}USDT", symbol.to_uppercase());
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .unwrap();
    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let price = json["price"].as_str().unwrap();
    let old_price = {
        let data_read = ctx.data.read().await;
        let price_reader_lock =
            data_read.get::<Cryptoprice>().expect("Expected CommandCounter in TypeMap.").clone();
    
        let price_reader = price_reader_lock.read().await;
    
        price_reader.get(&symbol.to_uppercase().to_string()).cloned().unwrap_or_else(|| "None".to_string())
    };
    let price_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<Cryptoprice>().expect("Expected CommandCounter in TypeMap.").clone()
    };
    {
        let mut counter = price_lock.write().await;
        let entry = counter.entry(symbol.to_uppercase().to_string()).or_insert("0".to_string());
        *entry = price.to_string();
    }
    (price.to_string(), old_price)
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILDS
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    {
        let mut data = client.data.write().await;

        data.insert::<CommandCounter>(Arc::new(RwLock::new(HashMap::default())));

        data.insert::<VoiceChannelId>(Arc::new(RwLock::new(HashMap::default())));
        
        data.insert::<Cryptoprice>(Arc::new(RwLock::new(HashMap::default())));
    }
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}