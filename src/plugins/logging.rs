use pee5::config::GuildConfig;
use serenity::{
    model::prelude::{ChannelId, GuildId, Message},
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub async fn run(ctx: &Context, config: &GuildConfig, guild_id: GuildId, msg: &Message) {
    let log_channel_id: Option<u64> = config.get_log_channel_id();

    match log_channel_id {
        Some(log_channel_id) => {
            for channel in guild_id.channels(&ctx.http).await.unwrap() {
                if channel.0.as_u64().to_owned() == log_channel_id {
                    ChannelId::from(log_channel_id)
                        .send_message(&ctx.http, |message| {
                            message.embed(|embed| {
                                embed
                                    .title("Message sent")
                                    .field("Sender", &msg.author, true)
                                    .field("Channel", msg.channel_id.mention(), true)
                                    .field("Content", &msg.content, false)
                                    .color(Colour::from_rgb(102, 255, 102))
                                    .footer(|footer| {
                                        footer.text(format!(
                                            "User ID: {} | Message ID: {} | Channel ID: {}",
                                            msg.author.id, msg.id, msg.channel_id
                                        ))
                                    })
                                    .timestamp(&msg.timestamp)
                            })
                        })
                        .await
                        .unwrap();
                }
            }
        }
        None => {}
    }
}
