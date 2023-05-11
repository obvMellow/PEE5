use pee5::config::GuildConfig;
use rusqlite::{params, Connection};
use serenity::{
    model::{
        prelude::{ChannelId, GuildId, Message, MessageId},
        Timestamp,
    },
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub struct SimplifiedMessage {
    pub id: u64,
    pub author_id: u64,
    pub channel_id: u64,
    pub content: String,
    pub unix_timestamp: i64,
}

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

pub async fn run_delete(
    ctx: &Context,
    config: &GuildConfig,
    guild_id: GuildId,
    channel_id: ChannelId,
    msg: MessageId,
) {
    let log_channel_id: Option<u64> = config.get_log_channel_id();

    if log_channel_id.is_none() {
        return;
    }

    let log_channel_id = log_channel_id.unwrap();

    let msg = match get_message_from_db(msg) {
        Some(msg) => msg,
        None => return,
    };

    for channel in guild_id.channels(&ctx.http).await.unwrap() {
        if channel.0.as_u64().to_owned() == log_channel_id {
            ChannelId::from(log_channel_id)
                .send_message(&ctx.http, |message| {
                    message.embed(|embed| {
                        embed
                            .title("Message deleted")
                            .field("Sender", format!("<@{}>", msg.author_id), true)
                            .field("Channel", channel_id.mention(), true)
                            .field("Content", &msg.content, false)
                            .color(Colour::from_rgb(255, 102, 102))
                            .footer(|footer| {
                                footer.text(format!(
                                    "User ID: {} | Message ID: {} | Channel ID: {}",
                                    msg.author_id, msg.id, channel_id
                                ))
                            })
                            .timestamp(Timestamp::from_unix_timestamp(msg.unix_timestamp).unwrap())
                    })
                })
                .await
                .unwrap();
        }
    }
}

fn get_message_from_db(message_id: MessageId) -> Option<SimplifiedMessage> {
    let conn = Connection::open("pee5.db").unwrap();

    let mut stmt = conn
        .prepare("SELECT * FROM messages WHERE id = ?1")
        .unwrap();

    let message_iter = stmt
        .query_map(params![message_id.0], |row| {
            Ok(SimplifiedMessage {
                id: row.get(0)?,
                author_id: row.get(1)?,
                channel_id: row.get(2)?,
                content: row.get(3)?,
                unix_timestamp: row.get(4)?,
            })
        })
        .unwrap();

    for message in message_iter {
        if let Ok(message) = message {
            return Some(message);
        }
    }

    None
}

fn insert_message_to_db(msg: &Message) {
    let conn = Connection::open("pee5.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY,
            author_id INTEGER NOT NULL,
            channel_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            unix_timestamp INTEGER NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO messages (id, author_id, channel_id, content, unix_timestamp)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            msg.id.0,
            msg.author.id.0,
            msg.channel_id.0,
            msg.content,
            msg.timestamp.unix_timestamp()
        ],
    )
    .unwrap();
}
