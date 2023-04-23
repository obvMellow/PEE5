use std::fs::File;

use pee5::config::GuildConfig;
use serenity::{model::prelude::Message, prelude::Context};

use crate::error_constructor;

pub async fn run(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let author = &msg.author;
    let member = msg.guild_id.unwrap().member(&ctx, author.id).await.unwrap();
    let guild = msg
        .guild_id
        .unwrap()
        .to_partial_guild(&ctx.http)
        .await
        .unwrap();

    if !guild
        .user_permissions_in(
            &msg.channel(&ctx.http).await.unwrap().guild().unwrap(),
            &member,
        )
        .unwrap()
        .manage_guild()
    {
        let content = error_constructor!("You do not have permission to use this command```");
        msg.reply_ping(&ctx.http, content).await.unwrap();
        return;
    }

    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 2 {
        help(msg, ctx).await;
        return;
    }

    let command = args[1];

    match command {
        "set" => {
            set(msg, ctx, config).await;
        }
        _ => {
            let content =
                error_constructor!(config command, "Invalid command", "expected a valid command");
            msg.reply_ping(&ctx.http, content).await.unwrap();
        }
    }
}

async fn set(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 3 {
        help(msg, ctx).await;
        return;
    }

    let key = args[2];

    match key {
        "log_channel" => {
            if args.len() < 4 {
                let content = error_constructor!(
                    format!("!config set {}", key),
                    "__",
                    "Missing argument",
                    "expected a channel mention"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            let channel_id = args[3].replace("<", "").replace(">", "").replace("#", "");

            if let Err(_) = channel_id.parse::<u64>() {
                let content = error_constructor!(
                    format!("!config set {}", key),
                    channel_id,
                    "Invalid argument",
                    "expected a channel mention"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            config
                .get_log_channel_id_mut()
                .replace(channel_id.parse::<u64>().unwrap());

            serde_json::to_writer_pretty(
                File::create(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64())).unwrap(),
                &config,
            )
            .unwrap();

            msg.reply_ping(&ctx.http, format!("Set `{}` to <#{}>", key, channel_id))
                .await
                .unwrap();
        }
        _ => {
            let content =
                error_constructor!(config set key, "Invalid argument", "expected a valid argument");
            msg.reply_ping(&ctx.http, content).await.unwrap();
            return;
        }
    }
}

async fn help(msg: &Message, ctx: &Context) {
    let content = format!(
        "**!config [KEYWORD] [ARGUMENTS]**
    
**Keywords:**
    `set` - Sets a config value

**Arguments:**
    `log_channel` - Sets the log channel
    
    **Example:**
        `!config set log_channel #general`"
    );

    msg.reply_ping(&ctx.http, content).await.unwrap();
}
