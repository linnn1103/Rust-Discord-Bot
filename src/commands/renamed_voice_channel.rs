///允許動態頻道創建者重新命名他們的頻道

use serenity::{
    all::{ChannelId, Context, CreateEmbed, ResolvedValue},
    builder::{CreateCommand, CreateCommandOption, EditChannel},
    model::{
        application::{CommandOptionType, ResolvedOption},
        colour,
    },
};

pub fn run(options: &[ResolvedOption], channel_id: Option<ChannelId>, ctx: Context) -> CreateEmbed {
    if channel_id.is_none() {
        CreateEmbed::default()
            .description("<a:modcheck:1272777094044188673> 起嗯找不到你擁有的頻道")
    } else {
        if let Some(ResolvedOption {
            value: ResolvedValue::String(channel_name),
            ..
        }) = options.first()
        {
            let channel_id = channel_id.unwrap();
            let builder = EditChannel::new().name(channel_name.to_string());
            tokio::task::spawn(async move {
                if let Err(why) = channel_id.edit(ctx.http, builder).await {
                    eprintln!("Failed to edit channel: {:?}", why);
                }
            });
            CreateEmbed::default()
                .color(colour::Colour::LIGHT_GREY)
                .description("起嗯已經幫你改好名子了".to_string())
        } else {
            CreateEmbed::default()
                .color(colour::Colour::RED)
                .description("發生了不可名狀的錯誤")
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("重新命名")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "頻道名稱",
                "給你的頻道取個個性化的名稱",
            )
            .required(true),
        )
        .description("為你的語音頻道重新命名")
}
