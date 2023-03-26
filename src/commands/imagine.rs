use crate::global_config::GlobalConfig;
use crate::Result;

use openai_gpt_rs::{
    args::{ImageArgs, ImageResponseFormat, ImageSize},
    client::Client,
};
use serde_json::Value;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;

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

    interaction
        .create_followup_message(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Images");
                embed.description("Here is your image!");
                embed.color(Colour::from_rgb(0, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed.image(url);

                embed
            })
        })
        .await?;

    Ok(())
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
