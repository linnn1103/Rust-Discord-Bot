use serenity::{
    all::CreateEmbed,
    builder::CreateCommand,
    model::{application::ResolvedOption, colour},
};

pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    CreateEmbed::default()
        .color(colour::Colour::RED)
        .description("我是中国人，我爱我的祖国 <:xi:1272631608322555944>")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("习近平万岁").description("向习近平宣誓效忠")
}
