mod commands;
mod global_config;
mod plugins;

use colored::Colorize;
use global_config::GlobalConfig;
use pee5::config::{GuildConfig, IsPlugin};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Activity, Guild, Message};
use serenity::prelude::*;
use serenity::utils::Colour;
use std::fs::{self, File};

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
                "remove_warn" => commands::remove_warn::run(&ctx, &command).await,
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
            Command::create_global_application_command(&ctx.http, |command| {
                commands::remove_warn::register(command)
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

            GuildConfig::new(guild.id).to_writer_pretty(file).unwrap();

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
        let mut config = GuildConfig::from_reader(config_file).unwrap();

        let users = config.get_users_mut();

        if users.get(&msg.author.id.as_u64()).is_none() {
            users.insert(msg.author.id.0, 0);
        }

        // Do the logging here
        if config.get_plugins().logging() {
            plugins::logging::run(&ctx, &config, guild_id, &msg).await;
        }

        // Moderate the message here
        let mut deleted = false;

        if config.get_plugins().automod() {
            deleted = plugins::automod::run(&msg, &ctx, &config).await;
        }

        if !deleted {
            // Give the user some xp here
            if config.get_plugins().xp() {
                plugins::xp::run(&msg, &mut config);
            }

            // Afk plugin here
            if config.get_plugins().afk() {
                plugins::afk::run(&msg, &ctx, &mut config).await;
            }

            // Reply if the message is sent in a chat
            let chats = std::fs::read_to_string(format!("{}/{}", CHAT_PATH, msg.guild_id.unwrap()));

            if let Ok(chats) = chats {
                if chats.contains(&msg.channel_id.to_string()) && config.get_plugins().chat() {
                    plugins::chat::run(&msg, &ctx, &config, Some(guild_id)).await;
                }
            }
        }

        if msg.content.starts_with("!config") {
            plugins::config::run(&msg, &ctx, &mut config).await;
        }

        // Save the config file here
        config.save(format!("guilds/{}.json", guild_id)).unwrap();
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

        GuildConfig::new(guild.id).to_writer_pretty(file).unwrap();
    }
}

async fn _dm_msg(ctx: Context, message: Message) {
    let config = GuildConfig::new(0 as u64);
    plugins::chat::run(&message, &ctx, &config, None).await;
}

#[tokio::main]
async fn main() {
    let token = GlobalConfig::load("config.json").discord_token;

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
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
