use serenity::{
    all::CreateEmbed,
    builder::{CreateCommand, CreateCommandOption},
    model::{
        application::{CommandOptionType, ResolvedOption},
        colour,
    },
};

pub fn run(_options: &[ResolvedOption], amount: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(colour::Colour::DARK_GREEN)
        .description(amount)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("指令計數器")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "指令名稱", "要查看的指令名稱")
                .add_string_choice("help", "help")
                .add_string_choice("习近平万岁", "习近平万岁")
                .add_string_choice("dirty_talk", "dirty_talk")
                .add_string_choice("求籤", "求籤")
                .add_string_choice("重新命名", "重新命名")
                .add_string_choice("指令計數器", "指令計數器")
                .required(true),
        )
        .description("讓起嗯看看你玩了多少次")
}
