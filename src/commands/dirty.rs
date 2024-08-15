//Kami Sqmf 提供的dirty talk
use rand::Rng;
use serenity::{
    all::CreateEmbed,
    builder::CreateCommand,
    model::{application::ResolvedOption, colour},
};
pub fn run(_options: &[ResolvedOption]) -> CreateEmbed {
    let dirty_talk = [
        "妳怎麼那麼騷",
        "好緊喔 好舒服",
        "我是你的小母狗",
        "拜託幹壞我好不好",
        "嗯 好舒服喔 寶貝太深了喔",
        "頂到點了啦 我還要啦",
        "你不要這樣看我啦 你眼神這樣很壞",
        "受不了了 腦袋一片空白",
        "很敏感啦 快壞掉了",
        "無止境的呻吟哈哈哈哈哈",
        "幹死妳這個小騷貨",
        "屁股翹這麼高多欠幹啊",
        "阿平時不是很清高？還不是在被我抓在這邊操",
        "如果賴清德知道你在這邊被我幹......他會怎麼想妳？",
        "屁股自己翹高一點",
        "妳的家人和朋友知道妳這麼淫蕩嗎？",
        "我要把妳幹到懷孕！",
        "反正妳被很多人操過了吧？那套子也不用了",
        "生來就是給我幹的",
        "自己看看自己多騷",
        "不用力操妳妳不長記性",
        "爽不爽？爽要說出來啊",
        "幹壞妳這個小騷貨",
        "給我夾緊",
        "好想在賴清德面前操妳",
        "我和賴清德誰幹妳比較爽？",
        "在你懷孕前我會每天幹妳",
        "妳搖完了嗎？",
        "其他人有這樣幹過妳嗎？嗯？",
        "求我幹妳啊",
        "我要你叫出來",
        "是不是沒有人這樣幹過妳？",
        "小母狗叫出來呀",
        "來幫主人吃吃",
        "下面那麼濕是不是等我幹壞你",
        "今天還不幹死你",
        "小母狗只能被我幹喔",
        "要不要看看妳自己現在多淫蕩",
    ];
    let rng = rand::thread_rng().gen_range(0..dirty_talk.len());
    CreateEmbed::default()
        .color(colour::Colour::MEIBE_PINK)
        .description(dirty_talk[rng].to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("dirty_talk")
        .description("跟起嗯來點dirty talk")
        .nsfw(true)
}
