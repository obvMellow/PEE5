mod commands;
mod global_config;

use std::collections::HashMap;
use std::fs::{self, File};
use std::time::Duration;

use colored::Colorize;
use global_config::GlobalConfig;
use openai_gpt_rs::client::Client as OpenAIClient;
use openai_gpt_rs::response::Content;
use serde_json::Value;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Activity, ChannelId, Guild, GuildId, Message};
use serenity::prelude::*;
use serenity::utils::Colour;

pub type Result<T> = std::result::Result<T, SerenityError>;

const CHAT_PATH: &str = "guilds/chats";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            println!(
                "{} {:#?}",
                "Received Command Interaction:".green().bold(),
                command
            );

            let result: Result<()> = match command.data.name.as_str() {
                "echo" => commands::echo::run(&ctx, &command).await,
                "timeout" => commands::timeout::run(&ctx, &command).await,
                "config" => commands::config::run(&ctx, &command).await,
                "add_role" => commands::add_role::run(&ctx, &command).await,
                "remove_role" => commands::remove_role::run(&ctx, &command).await,
                "automod" => commands::automod::run(&ctx, &command).await,
                "blacklist_word" => commands::blacklist_word::run(&ctx, &command).await,
                "xp" => commands::xp::run(&ctx, &command).await,
                "imagine" => commands::imagine::run(&ctx, &command).await,
                "saved_imagines" => commands::saved_imagines::run(&ctx, &command).await,
                "avatar" => commands::avatar::run(&ctx, &command).await,
                "ask_gpt" => commands::ask_gpt::run(&ctx, &command).await,
                "help" => commands::help::run(&ctx, &command).await,
                "afk" => commands::afk::run(&ctx, &command).await,
                "purge" => commands::purge::run(&ctx, &command).await,
                "warn" => commands::warn::run(&ctx, &command).await,
                "warns" => commands::warns::run(&ctx, &command).await,
                "chat" => commands::chat::run(&ctx, &command).await,
                _ => Ok(()),
            };

            if let Err(why) = result {
                eprintln!(
                    "{} An error occured on a slash command: {:?}",
                    "Error".red().bold(),
                    why
                );

                command
                    .create_followup_message(&ctx.http, |message| {
                        message.embed(|embed| {
                            embed
                                .title("Error")
                                .field("Error Message", why, false)
                                .color(Colour::RED)
                        })
                    })
                    .await
                    .unwrap();
            }
        }

        if let Interaction::MessageComponent(component) = &interaction {
            println!(
                "{} {:#?}",
                "Received Component Interaction:".green().bold(),
                component
            );

            let result: Result<()> = match component.data.custom_id.as_str() {
                "imagine_retry" => commands::imagine::retry(&ctx, &component).await,
                "imagine_save" => commands::imagine::save(&ctx, &component).await,
                _ => Ok(()),
            };

            if let Err(why) = result {
                eprintln!(
                    "{} An error occured on a component: {:?}",
                    "Error".red().bold(),
                    why
                );
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        // Create application commands here
        let commands = vec![
            Command::create_global_application_command(&ctx.http, |command| {
                commands::echo::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::timeout::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::config::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::add_role::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::remove_role::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::automod::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::blacklist_word::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::xp::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::imagine::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::saved_imagines::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::avatar::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::ask_gpt::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::help::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::afk::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::purge::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::warn::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::warns::register(command)
            })
            .await
            .unwrap(),
            Command::create_global_application_command(&ctx.http, |command| {
                commands::chat::register(command)
            })
            .await
            .unwrap(),
        ];

        // Create guild config files here
        let guilds = ready.guilds;

        for guild in guilds {
            let config_file = format!("guilds/{}.json", guild.id);

            if path_exists(&config_file) {
                continue;
            }

            let file = match File::create(config_file) {
                Ok(v) => v,
                Err(e) => panic!("Error creating config file: {}", e),
            };

            serde_json::to_writer_pretty(
                file,
                &serde_json::json!({
                    "id": guild.id,
                    "users": {},
                    "automod": false,
                    "blacklisted_words": [],
                    "afk": {},
                }),
            )
            .unwrap();

            println!(
                "{} config file for guild {}",
                "Created".green().bold(),
                guild.id
            );
        }

        ctx.set_activity(Activity::playing("DM to chat with me!"))
            .await;

        println!(
            "{} Registered commands: {:#?}, Connected to {}",
            "Ready".green().bold(),
            commands
                .iter()
                .map(|command| command.name.clone())
                .collect::<Vec<String>>(),
            ready.user.name
        );
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.guild_id.is_none() {
            _dm_msg(ctx, msg).await;
            return;
        }

        // Add the message author to the config file if they aren't already
        let guild_id = msg.guild_id.unwrap();

        let config_file = File::open(format!("guilds/{}.json", guild_id)).unwrap();
        let mut config: Value = serde_json::from_reader(&config_file).unwrap();

        let users = config.as_object_mut().unwrap().get_mut("users").unwrap();

        if users
            .as_object()
            .unwrap()
            .get(&msg.author.id.to_string())
            .is_none()
        {
            users
                .as_object_mut()
                .unwrap()
                .insert(msg.author.id.to_string(), 0.into());
        }

        // Do the logging here
        let log_channel_id: Option<u64> = match config.as_object().unwrap().get("log_channel_id") {
            Some(v) => Some(v.as_str().unwrap().parse().unwrap()),
            None => None,
        };

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

        logging(&msg);

        // Moderate the message here
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
                                .description(format!(
                                    "{} Watch your language!",
                                    msg.author.mention()
                                ))
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

        // Give the user some xp here
        if !deleted {
            let users = config.as_object_mut().unwrap().get_mut("users").unwrap();

            let xp_gain = 100;

            let xp = users
                .as_object_mut()
                .unwrap()
                .get_mut(&msg.author.id.to_string())
                .unwrap();

            *xp = (xp.as_u64().unwrap() + xp_gain).into();

            // Check if the message mentions a user who is afk
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
                                        .description(format!(
                                            "{} is afk: {}",
                                            mention.mention(),
                                            reason
                                        ))
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

            // Reply if the message is sent in a chat
            let chats = std::fs::read_to_string(format!("{}/{}", CHAT_PATH, msg.guild_id.unwrap()));

            dbg!(&chats);

            if let Ok(chats) = chats {
                if chats.contains(&msg.channel_id.to_string()) {
                    _chat(msg, ctx, &mut config, Some(guild_id)).await;
                }
            }
        }

        // Save the config file here
        _save(guild_id, &mut config);
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild) {
        let config_file = format!("guilds/{}.json", guild.id);

        if path_exists(&config_file) {
            return;
        }

        let file = match File::create(config_file) {
            Ok(v) => v,
            Err(e) => panic!("Error creating config file: {}", e),
        };

        serde_json::to_writer_pretty(
            file,
            &serde_json::json!({
                "id": guild.id,
                "users": {},
                "automod": false,
                "blacklisted_words": [],
                "afk": {},
            }),
        )
        .unwrap();
    }
}

fn _save(guild_id: GuildId, config: &mut Value) {
    let config_file = File::create(format!("guilds/{}.json", guild_id)).unwrap();

    serde_json::to_writer_pretty(config_file, &config).unwrap();
}

async fn _chat(msg: Message, ctx: Context, config: &mut Value, guild_id: Option<GuildId>) {
    let channel = msg.channel_id;

    if msg.content.starts_with("!end") && guild_id.is_some() {
        let mut chats =
            std::fs::read_to_string(format!("{}/{}", CHAT_PATH, msg.guild_id.unwrap())).unwrap();

        chats = chats.replace(&msg.channel_id.to_string(), "");
        std::fs::write(format!("{}/{}", CHAT_PATH, msg.guild_id.unwrap()), chats).unwrap();

        channel.delete(&ctx.http).await.unwrap();

        _save(guild_id.unwrap(), config);
        return;
    }

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
    context.push_str("Only include the content of your response, not the author.");

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
}

async fn _dm_msg(ctx: Context, message: Message) {
    let mut config = Value::Null;
    _chat(message, ctx, &mut config, None).await;
}

fn logging(msg: &Message) {
    let mut log_msg = String::from(&msg.content);

    if msg.embeds.len() > 0 {
        log_msg.push_str(&format!(
            "\n\n{} {:?}",
            "Embed:".yellow().bold(),
            msg.embeds
        ));
    }

    println!(
        "{} {} {} {} {} {}",
        "Message:".green().bold(),
        log_msg,
        "\nfrom".green().bold(),
        msg.author.tag(),
        "\nin".green().bold(),
        msg.channel_id
    );
}

#[tokio::main]
async fn main() {
    let token = GlobalConfig::load("config.json").discord_token;

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("{} Client error: {:?}", "Error".red().bold(), why);
    }
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
