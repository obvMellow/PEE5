use std::fs;

use crate::Result;
use reqwest::Url;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::{Context, Mentionable},
};
use std::str::FromStr;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let url = interaction.data.options[0]
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    if !url.starts_with("https://youtu") {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("Error: Invalid URL")
                    })
            })
            .await?;
        return Ok(());
    }

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Downloading video (This can take a while)...")
                })
        })
        .await?;

    let url = match Url::from_str(url) {
        Ok(u) => u,
        Err(e) => {
            interaction
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(format!("Error: {}", e))
                })
                .await?;
            return Ok(());
        }
    };

    let video = match rustube::Video::from_url(&url).await {
        Ok(v) => v,
        Err(e) => {
            interaction
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(format!("Error: {}", e))
                })
                .await?;
            return Ok(());
        }
    };

    let stream = video.best_quality().unwrap();

    let path_to_video = match stream.download().await {
        Ok(v) => v,
        Err(e) => {
            interaction
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.content(format!("Error: {}", e))
                })
                .await?;
            return Ok(());
        }
    };

    interaction
        .create_followup_message(&ctx.http, |response| {
            response
                .embed(|embed| embed.title("Here's your video!"))
                .add_file(&path_to_video)
                .content(interaction.user.mention())
        })
        .await?;

    fs::remove_file(path_to_video)?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("download_video")
        .description("Download a video from YouTube")
        .create_option(|option| {
            option
                .name("url")
                .description("The URL of the video to download")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
