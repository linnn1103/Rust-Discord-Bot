use serenity::{
    all::{CreateEmbed, CreateEmbedAuthor},
    builder::CreateCommand,
    model::{application::ResolvedOption, colour},
};
pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    CreateEmbed::default()
        .author(CreateEmbedAuthor::new("CoolBot"))
        .title("看看起嗯都能做些什麼")
        .color(colour::Colour::DARK_GOLD)
        .thumbnail("https://raw.githubusercontent.com/linnn1103/Rust-Discord-Bot/main/material/Chi-En.jpg?token=GHSAT0AAAAAACVWHQC3AMZXSWMRUOIYFFBOZV3YW6A")
        .description("你好我是起嗯，一個多功能機器人。 
我不具備強大的語言處理能力，不可以與人類進行自然流暢的對話。
我不能回答各類問題，包括科學、歷史、文化、技術等各方面的知識。
此外，我也不能協助進行文本創作，幫助編寫文章、報告或故事。")
        .fields(vec![
            ("向起嗯大神求籤", " `/求籤`", true),
            ("跟起嗯dirty talk", " `/dirty_talk`", true),
            ("看看起嗯被用了幾次", " `/指令計數器 <command name>`", true),
            ("查詢加密貨幣價格", " `/加密貨幣現貨 <code>`", true),
            ("為你的語音頻道重新命名", " `/重新命名 <name>`", true),
            ("向伟大的起嗯总书记宣誓效忠", " `/习近平万岁`", true),
        ])
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("看看起嗯都能做些什麼")
}
