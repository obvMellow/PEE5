use std::fs::File;

use crate::Result;
use pee5::config::GuildConfig;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            InteractionResponseType,
        },
    },
    prelude::Context,
};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let reason = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "reason")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::String(reason) = reason {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!("Thinking..."));
                        message
                    })
            })
            .await?;

        let config_file = format!("guilds/{}.json", interaction.guild_id.unwrap());

        let config_file = match File::open(config_file) {
            Ok(v) => v,
            Err(e) => panic!("Error creating config file: {}", e),
        };

        let mut config = GuildConfig::from_reader(config_file).unwrap();

        config
            .get_afk_mut()
            .insert(interaction.user.id.0, reason.to_string());

        config
            .to_writer_pretty(
                File::create(format!("guilds/{}.json", interaction.guild_id.unwrap())).unwrap(),
            )
            .unwrap();

        interaction
            .edit_original_interaction_response(&ctx.http, |response| {
                response.embed(|embed| {
                    embed.title("AFK").description(format!(
                        "{} Set your AFK status to: {}",
                        interaction.user, reason
                    ))
                })
            })
            .await?;
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("afk")
        .description("Set your AFK status")
        .create_option(|option| {
            option
                .name("reason")
                .description("The reason for your AFK status")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
