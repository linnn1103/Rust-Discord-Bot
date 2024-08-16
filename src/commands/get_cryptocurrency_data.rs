use serenity::{
    all::{CommandOptionType, CreateCommandOption, CreateEmbed, CreateEmbedAuthor},
    builder::CreateCommand,
    model::{application::ResolvedOption, colour},
};

pub fn run(_options: &[ResolvedOption], symbol: String, price: String) -> CreateEmbed {
    CreateEmbed::default()
        .color(colour::Colour::GOLD)
        .title(symbol.to_uppercase())
        .author(
            CreateEmbedAuthor::new("Binance API")
                .icon_url("https://avatars.githubusercontent.com/u/32770468?s=200&v=4"),
        )
        .thumbnail("https://github.com/linnn1103/Rust-Discord-Bot/blob/v-0.1.2/material/crypto.gif")
        .description(format!("現價:\n\t{}", price))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("加密貨幣現貨")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "幣種", "如BTCUSDT、ETHUSDT")
                .required(true),
        )
        .description("獲得加密貨幣的資訊")
}
