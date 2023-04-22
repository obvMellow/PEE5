use pee5::config::GuildConfig;
use serenity::{
    model::prelude::Message,
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub async fn run(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let afk = config.get_afk_mut();

    for mention in &msg.mentions {
        if let Some(reason) = afk.get(&mention.id.0) {
            msg.channel_id
                .send_message(&ctx.http, |message| {
                    message
                        .embed(|embed| {
                            embed
                                .description(format!("{} is afk: {}", mention.mention(), reason))
                                .color(Colour::from_rgb(255, 255, 102))
                        })
                        .content(&msg.author)
                })
                .await
                .unwrap();
        }
    }

    // Check if the user is afk
    if let Some(_) = afk.get(&msg.author.id.0) {
        afk.remove(&msg.author.id.0);

        msg.channel_id
            .send_message(&ctx.http, |message| {
                message
                    .embed(|embed| {
                        embed
                            .title("I removed your afk.")
                            .color(Colour::from_rgb(102, 255, 102))
                    })
                    .content(&msg.author)
            })
            .await
            .unwrap();
    }
}
