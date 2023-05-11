use pee5::config::GuildConfig;
use rusqlite::{params, Connection};
use serenity::{
    model::prelude::{ChannelId, GuildId, Message},
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub async fn run(ctx: &Context, config: &GuildConfig, guild_id: GuildId, msg: &Message) {
    let log_channel_id: Option<u64> = config.get_log_channel_id();

    insert_message_to_db(&msg);

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

fn insert_message_to_db(msg: &Message) {
    let conn = Connection::open("pee5.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            author_id INTEGER NOT NULL,
            channel_id INTEGER NOT NULL,
            content TEXT NOT NULL,
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO messages (author_id, channel_id, content)
        VALUES (?1, ?2, ?3)",
        params![msg.author.id.0, msg.channel_id.0, msg.content],
    )
    .unwrap();
}
