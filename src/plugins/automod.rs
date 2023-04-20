use std::time::Duration;

use serde_json::Value;
use serenity::{
    model::prelude::Message,
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub async fn run(msg: &Message, ctx: &Context, config: &Value) -> bool {
    let mut deleted = false;

    let automod = config
        .as_object()
        .unwrap()
        .get("automod")
        .unwrap()
        .as_bool()
        .unwrap();

    if automod {
        let blacklisted_words = config
            .as_object()
            .unwrap()
            .get("blacklisted_words")
            .unwrap()
            .as_array()
            .unwrap();

        let mut contained_words: Vec<String> = Vec::new();

        for word in blacklisted_words {
            if msg
                .content
                .contains(word.as_str().unwrap().to_lowercase().trim())
            {
                contained_words.append(&mut vec![word.as_str().unwrap().to_string()]);
            }
        }

        if !contained_words.is_empty() {
            msg.delete(&ctx.http).await.unwrap();

            let msg = msg
                .channel_id
                .send_message(&ctx.http, |message| {
                    message.embed(|embed| {
                        embed
                            .description(format!("{} Watch your language!", msg.author.mention()))
                            .color(Colour::from_rgb(255, 102, 102))
                    })
                })
                .await
                .unwrap();

            deleted = true;

            tokio::time::sleep(Duration::from_secs(5)).await;

            msg.delete(&ctx.http).await.unwrap();
        }
    }

    deleted
}
