use std::{
    fs::{self, File},
    io::Write,
};

use crate::global_config::GlobalConfig;
use crate::Result;

use openai_gpt_rs::{
    args::{ImageArgs, ImageResponseFormat, ImageSize},
    client::Client,
};
use serde_json::Value;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::{
    command::CommandOptionType, interaction::message_component::MessageComponentInteraction,
};
use serenity::{
    builder::{CreateApplicationCommand, CreateButton},
    model::prelude::component::ButtonStyle,
};

use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let _prompt = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "prompt")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let key = GlobalConfig::load("config.json").openai_key;

    let client = Client::new(&key);

    let args = ImageArgs::new(
        _prompt,
        Some(1),
        Some(ImageSize::Big),
        Some(ImageResponseFormat::Url),
    );

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("Generating images...");
                    message
                })
        })
        .await
        .unwrap();

    let url = _generate(&client, &args).await;

    interaction
        .create_followup_message(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Imagine");
                embed.description("Here is your image!");
                embed.color(Colour::from_rgb(0, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed.image(url);

                embed
            });

            response.components(|component| {
                component.create_action_row(|row| {
                    row.add_button(
                        CreateButton::default()
                            .label("Retry")
                            .style(ButtonStyle::Primary)
                            .custom_id("imagine_retry")
                            .to_owned(),
                    )
                })
            });

            response
        })
        .await?;

    let tmp_name = format!(
        "tmp/{}:{}:{}",
        interaction.guild_id.unwrap(),
        interaction.channel_id,
        interaction.user.id,
    );
    let mut tmp_file = File::create(tmp_name).unwrap();
    tmp_file.write_all(_prompt.as_bytes()).unwrap();

    Ok(())
}

pub async fn retry(ctx: &Context, component: &MessageComponentInteraction) -> Result<()> {
    let prompt = fs::read_to_string(format!(
        "tmp/{}:{}:{}",
        component.guild_id.unwrap(),
        component.channel_id,
        component.user.id,
    ))
    .unwrap();

    let key = GlobalConfig::load("config.json").openai_key;

    let client = Client::new(&key);

    let args = ImageArgs::new(
        &prompt,
        Some(1),
        Some(ImageSize::Big),
        Some(ImageResponseFormat::Url),
    );

    component
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredUpdateMessage)
                .interaction_response_data(|message| {
                    message.content("Generating image...");
                    message
                })
        })
        .await
        .unwrap();

    component
        .edit_original_interaction_response(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Imagine");
                embed.description("Generating image...");
                embed.color(Colour::from_rgb(255, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed
            })
        })
        .await?;

    let url = _generate(&client, &args).await;

    component
        .edit_original_interaction_response(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Imagine");
                embed.description("Here is your image!");
                embed.color(Colour::from_rgb(0, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed.image(url);

                embed
            });

            response.components(|component| {
                component.create_action_row(|row| {
                    row.add_button(
                        CreateButton::default()
                            .label("Retry")
                            .style(ButtonStyle::Primary)
                            .custom_id("imagine_retry")
                            .to_owned(),
                    )
                })
            });

            response
        })
        .await?;

    Ok(())
}

async fn _generate(client: &Client, args: &ImageArgs) -> String {
    let resp = client.create_image(&args).await.unwrap();

    let json: Value = resp.resp.json().await.unwrap();

    let url = json
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap()[0]
        .as_object()
        .unwrap()
        .get("url")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    url
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("imagine")
        .description("Turn your imagination into an AI generated image")
        .create_option(|option| {
            option
                .name("prompt")
                .description("Prompt for the AI to generate an image from")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
