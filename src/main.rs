mod commands;
mod global_config;

use std::fs::{self, File};

use colored::Colorize;
use global_config::GlobalConfig;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub type Result<T> = std::result::Result<T, SerenityError>;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!(
                "{} {:#?}",
                "Received Command Interaction:".green().bold(),
                command
            );

            let result: Result<()> = match command.data.name.as_str() {
                "echo" => commands::echo::run(&ctx, &command).await,
                "timeout" => commands::timeout::run(&ctx, &command).await,
                _ => Ok(()),
            };

            if let Err(why) = result {
                eprintln!(
                    "{} An error occured on a slash command: {:?}",
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
