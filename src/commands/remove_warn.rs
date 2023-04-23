use std::path::Path;

use crate::Result;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            InteractionResponseType,
        },
    },
    prelude::Context,
    utils::Colour,
};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let user = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "user")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::User(user, _) = user {
        let warn_id = interaction
            .data
            .options
            .iter()
            .find(|option| option.name == "warn_id")
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap();

        let name = format!(
            "guilds/warns/{}/{}/",
            interaction.guild_id.unwrap(),
            user.id,
        );

        let folder = Path::new(&name);

        match std::fs::remove_file(folder.join(&warn_id)) {
            Ok(_) => {
                interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.embed(|embed| {
                                    embed
                                        .title(format!("Removed warn from {}", user.tag()))
                                        .field("ID", warn_id, false)
                                })
                            })
                    })
                    .await?;
            }
            Err(_) => {
                interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.embed(|embed| {
                                    embed
                                        .title(format!("Failed to remove warn from {}", user.tag()))
                                        .field("ID", warn_id, false)
                                        .colour(Colour::RED)
                                })
                            })
                    })
                    .await?;
            }
        };
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove_warn")
        .description("Remove a warn from a user")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("user")
                .description("The user to remove the warn from")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("warn_id")
                .description("The ID of the warn to remove")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
