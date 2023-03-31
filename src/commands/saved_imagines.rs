use std::fs::{self, File};

use crate::Result;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{interaction::InteractionResponseType, AttachmentType};

use serenity::prelude::Context;

use std::io::Read;

pub async fn run<'a>(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let saved_imagines_dir = format!("saved_imagines/{}", interaction.user.id);

    if !fs::metadata(&saved_imagines_dir).is_ok() {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("You have no saved imagines");
                        message
                    })
            })
            .await?;
    }

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await?;

    let mut files: Vec<AttachmentType> = Vec::new();

    for entry in fs::read_dir(&saved_imagines_dir)? {
        let entry = entry?;
        let path = entry.path();
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let attachment = AttachmentType::Bytes {
            data: bytes.into(),
            filename: entry
                .file_name()
                .into_string()
                .unwrap_or_else(|_| "unknown".into()),
        };
        files.push(attachment);
    }

    let resp = interaction
        .create_followup_message(&ctx.http, |response| {
            response.content("Here I created a thread for your saved imagines");
            response
        })
        .await?;

    let thread = interaction
        .channel_id
        .create_public_thread(&ctx.http, resp, |thread| {
            thread.name(format!("{}'s saved imagines", interaction.user));
            thread
        })
        .await?;

    thread
        .send_message(&ctx.http, |msg| {
            msg.content("Here is your saved imagines (This might take a while)")
        })
        .await?;

    for file in files {
        thread
            .send_files(&ctx.http, vec![file], |message| message)
            .await?;
    }

    thread
        .send_message(&ctx.http, |msg| {
            msg.content(format!("{} Done!", interaction.user))
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("saved_imagines")
        .description("See all of your saved imagines")
}
