use pee5::config::GuildConfig;
use serenity::model::prelude::Message;

pub fn run(msg: &Message, config: &mut GuildConfig) {
    let users = config.get_users_mut();

    let xp_gain: usize = 100;

    let xp = users.get_mut(&msg.author.id.0).unwrap();

    *xp = (*xp + xp_gain).into();
}
