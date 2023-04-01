use crate::Result;
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

        let new_nick = format!(
            "[AFK] {}",
            interaction
                .member
                .clone()
                .unwrap()
                .nick
                .unwrap_or(interaction.user.name.clone())
                .to_string(),
        );

        interaction
            .member
            .clone()
            .unwrap()
            .edit(&ctx.http, |edit| edit.nickname(new_nick))
            .await?;

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
