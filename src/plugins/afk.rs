use serde_json::Value;
use serenity::{
    model::prelude::Message,
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub async fn run(msg: &Message, ctx: &Context, config: &mut Value) {
    let afk = config.as_object_mut().unwrap().get_mut("afk").unwrap();

    for mention in &msg.mentions {
        if afk
            .as_object()
            .unwrap()
            .get(&mention.id.to_string())
            .is_some()
        {
            let reason = afk
                .as_object()
                .unwrap()
                .get(&mention.id.to_string())
                .unwrap()
                .as_object()
                .unwrap()
                .get("reason")
                .unwrap()
                .as_str()
                .unwrap();

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
    if afk
        .as_object()
        .unwrap()
        .get(&msg.author.id.to_string())
        .is_some()
    {
        afk.as_object_mut()
            .unwrap()
            .remove(&msg.author.id.to_string());

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
