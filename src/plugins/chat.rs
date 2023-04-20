use std::{collections::HashMap, thread, time::Duration};

use colored::Colorize;
use openai_gpt_rs::client::Client as OpenAIClient;
use openai_gpt_rs::response::Content;
use serde_json::Value;
use serenity::{
    model::prelude::{ChannelId, GuildId, Message},
    prelude::{Context, Mentionable},
};

use crate::_save;
use crate::global_config::GlobalConfig;

const CHAT_PATH: &str = "guilds/chats";
const CHAT_COMMANDS: [&str; 3] = ["!end", "!rename", "!clear"];

pub async fn run(msg: &Message, ctx: &Context, config: &mut Value, guild_id: Option<GuildId>) {
    let channel = msg.channel_id;

    if CHAT_COMMANDS.iter().any(|v| msg.content.starts_with(*v)) && guild_id.is_some() {
        match msg.content.split(' ').nth(0) {
            Some("!end") => {
                _end(&msg, channel, &ctx, guild_id, config).await;
                return;
            }
            Some("!rename") => {
                _rename(&msg, channel, &ctx).await;
                return;
            }
            Some("!clear") => {
                _clear(&msg, channel, &ctx).await;
                return;
            }
            _ => (),
        }
    }

    let typing = ctx.http.start_typing(msg.channel_id.0).unwrap();

    let mut context_msg = channel
        .messages(&ctx.http, |builder| builder.limit(100))
        .await
        .unwrap();

    context_msg.reverse();

    let mut context = String::new();

    for msg in context_msg {
        context.push_str(
            format!("Author: {}\nContent: {} \n\n", msg.author.name, msg.content).as_str(),
        );
    }
    context.push_str(
        "# Only include the content of your response, not the author, or \"Content:\" label.",
    );

    let mut context_msg = HashMap::new();
    context_msg.insert("role".to_string(), "assistant".to_string());
    context_msg.insert("content".to_string(), context);

    let mut user_msg = HashMap::new();
    user_msg.insert("role".to_string(), "user".to_string());
    user_msg.insert("content".to_string(), msg.content.clone());

    let mut messages = Vec::new();
    messages.push(context_msg);
    messages.push(user_msg);

    let client = OpenAIClient::new(&GlobalConfig::load("config.json").openai_key);

    let resp = client
        .create_chat_completion(|args| {
            args.max_tokens(1024)
                .messages(messages)
                .model("gpt-3.5-turbo")
        })
        .await
        .unwrap();

    let new_msg = match resp.json.as_object().unwrap().get("error") {
        Some(error) => {
            let error = error
                .as_object()
                .unwrap()
                .get("message")
                .unwrap()
                .as_str()
                .unwrap();

            error.to_string()
        }
        None => resp.get_content(0).await.unwrap(),
    };

    msg.reply(&ctx.http, new_msg).await.unwrap();
    typing.stop().unwrap();
}

async fn _end(
    msg: &Message,
    channel: ChannelId,
    ctx: &Context,
    guild_id: Option<GuildId>,
    config: &mut Value,
) {
    let mut chats =
        std::fs::read_to_string(format!("{}/{}", CHAT_PATH, msg.guild_id.unwrap())).unwrap();
    chats = chats.replace(&msg.channel_id.to_string(), "");
    std::fs::write(format!("{}/{}", CHAT_PATH, msg.guild_id.unwrap()), chats).unwrap();
    channel.delete(&ctx.http).await.unwrap();
    _save(guild_id.unwrap(), config);
}

async fn _rename(msg: &Message, channel: ChannelId, ctx: &Context) {
    let name = msg.content.split(' ').nth(1);
    match name {
        Some(name) => {
            channel.edit(&ctx.http, |c| c.name(name)).await.unwrap();
            msg.reply_ping(&ctx.http, "Channel renamed!").await.unwrap();
        }
        None => {
            let spaces = " ".repeat(msg.content.len() + 1);
            let error_msg = format!(
                "```ansi\n{}: No name provided\n
    {} {} {}
    {} {}{} {}\n
    {}: use !rename <name> to rename the channel (eg. !rename cool-channel)```",
                "error".red().bold(),
                "|".bold(),
                msg.content,
                "__".red().bold(),
                "|".bold(),
                spaces,
                "^^".red().bold(),
                "expected a name".red().bold(),
                "= help".bold()
            );

            msg.reply_ping(&ctx.http, error_msg).await.unwrap();
        }
    }
}

async fn _clear(msg: &Message, channel: ChannelId, ctx: &Context) {
    let messages = channel
        .messages(&ctx.http, |builder| builder.limit(100))
        .await
        .unwrap();

    for msg in messages {
        msg.delete(&ctx.http).await.unwrap();
    }
    let resp = channel
        .say(&ctx.http, format!("{} Cleared!", msg.author.mention()))
        .await
        .unwrap();
    thread::sleep(Duration::from_secs(5));
    resp.delete(&ctx.http).await.unwrap();
}
