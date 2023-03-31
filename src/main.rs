mod commands;
mod global_config;

use std::fs::{self, File};

use colored::Colorize;
use global_config::GlobalConfig;
use serde_json::Value;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::*;
use serenity::utils::Colour;

pub type Result<T> = std::result::Result<T, SerenityError>;

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
                }),
            )
            .unwrap();

            println!(
                "{} config file for guild {}",
                "Created".green().bold(),
                guild.id
            );
        }

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
                                })
                            })
                            .await
                            .unwrap();
                    }
                }
            }
            None => {}
        }

        // Moderate the message here
        let mut deleted = false;

        if config
            .as_object()
            .unwrap()
            .get("automod")
            .unwrap()
            .as_bool()
            .unwrap()
        {
            let blacklisted_words = config
                .as_object()
                .unwrap()
                .get("blacklisted_words")
                .unwrap()
                .as_array()
                .unwrap();

            let mut contained_words: Vec<String> = Vec::new();

            for word in blacklisted_words {
                if msg.content.contains(word.as_str().unwrap()) {
                    contained_words.append(&mut vec![word.as_str().unwrap().to_string()]);
                }
            }

            if !contained_words.is_empty() {
                msg.delete(&ctx.http).await.unwrap();

                msg.channel_id
                    .send_message(&ctx.http, |message| {
                        message.embed(|embed| {
                            embed
                                .title("Message deleted")
                                .field("Sender", &msg.author, true)
                                .field("Channel", msg.channel_id.mention(), true)
                                .field("Content", &msg.content, false)
                                .field(
                                    "Reason",
                                    format!(
                                        "Message contained blacklisted words: {:#?}",
                                        contained_words
                                    ),
                                    false,
                                )
                                .color(Colour::from_rgb(255, 102, 102))
                        })
                    })
                    .await
                    .unwrap();

                deleted = true;
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
        }

        // Save the config file here
        let config_file = File::create(format!("guilds/{}.json", guild_id)).unwrap();

        serde_json::to_writer_pretty(config_file, &config).unwrap();
    }
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
