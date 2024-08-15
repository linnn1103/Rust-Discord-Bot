use rand::Rng;
use serenity::{all::CreateEmbed, builder::CreateCommand, model::{application::ResolvedOption, colour}};
pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    let (rng, col) = match rand::thread_rng().gen_range(1..=65) {
        1..=5 => ("大吉", colour::Colour::DARK_GOLD),
        6..=15 => ("吉", colour::Colour::DARKER_GREY),
        16..=30 => ("小吉", colour::Colour::DARK_GREEN),
        31..=45 => ("小凶", colour::Colour::DARK_GREY),
        46..=55 => ("凶", colour::Colour::RED),
        56..=60 => ("大凶", colour::Colour::DARK_RED),
        61..=63 => ("起嗯大神不想給你籤", colour::Colour::DARK_PURPLE),
        _ => ("你是個GAY", colour::Colour::DARK_PURPLE),
    };
    CreateEmbed::default().color(col).description(rng.to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("求籤").description("向起嗯大神抽個籤看看你的運氣")
}
