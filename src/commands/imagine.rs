use std::{
    fs::{self, File},
    io::Write,
};

use crate::global_config::GlobalConfig;
use crate::Result;

use openai_gpt_rs::{
    args::{ImageArgs, ImageResponseFormat, ImageSize},
    client::Client,
    response::Content,
};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Url;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::{
    command::CommandOptionType, interaction::message_component::MessageComponentInteraction,
};
use serenity::{builder::CreateApplicationCommand, model::prelude::component::ButtonStyle};

use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::utils::Colour;

const RESPONSE_DESCRIPTION: &str =
    "Here is your image!\n\n**Images are deleted after 24 hours unless saved.**";

const SAVE_DESCRIPTION: &str = "Here is your image!\n\n**Image is saved!**";

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

    let url = _generate(&client, |args| {
        args.prompt(_prompt)
            .n(1)
            .size(ImageSize::Big)
            .response_format(ImageResponseFormat::Url)
    })
    .await;

    let msg = interaction
        .edit_original_interaction_response(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Imagine");
                embed.description(RESPONSE_DESCRIPTION);
                embed.color(Colour::from_rgb(0, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed.image(&url);

                embed
            });

            response.components(|component| {
                component.create_action_row(|row| {
                    row.create_button(|button| {
                        button
                            .label("Retry")
                            .style(ButtonStyle::Primary)
                            .custom_id("imagine_retry")
                    })
                    .create_button(|button| {
                        button
                            .custom_id("imagine_save")
                            .style(ButtonStyle::Secondary)
                            .label("Save")
                    })
                })
            });

            response
        })
        .await?;

    let tmp_name = format!(
        "tmp/{}:{}:{}:{}",
        interaction.guild_id.unwrap(),
        interaction.channel_id,
        interaction.user.id,
        msg.id,
    );
    let mut tmp_file = File::create(tmp_name).unwrap();
    tmp_file
        .write_all(format!("{}\n{}", _prompt, url).as_bytes())
        .unwrap();

    Ok(())
}

pub async fn retry(ctx: &Context, component: &MessageComponentInteraction) -> Result<()> {
    let prompt = fs::read_to_string(format!(
        "tmp/{}:{}:{}:{}",
        component.guild_id.unwrap(),
        component.channel_id,
        component.user.id,
        component.message.id,
    ))
    .unwrap();

    let prompt = prompt.split("\n").collect::<Vec<&str>>()[0];

    let key = GlobalConfig::load("config.json").openai_key;

    let client = Client::new(&key);

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

    let url = _generate(&client, |args| {
        args.prompt(prompt)
            .n(1)
            .size(ImageSize::Big)
            .response_format(ImageResponseFormat::Url)
    })
    .await;

    component
        .edit_original_interaction_response(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Imagine");
                embed.description(RESPONSE_DESCRIPTION);
                embed.color(Colour::from_rgb(0, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed.image(&url);

                embed
            });

            response.components(|component| {
                component.create_action_row(|row| {
                    row.create_button(|button| {
                        button
                            .label("Retry")
                            .style(ButtonStyle::Primary)
                            .custom_id("imagine_retry")
                    })
                    .create_button(|button| {
                        button
                            .custom_id("imagine_save")
                            .style(ButtonStyle::Secondary)
                            .label("Save")
                    })
                })
            });

            response
        })
        .await?;

    let tmp_name = format!(
        "tmp/{}:{}:{}",
        component.guild_id.unwrap(),
        component.channel_id,
        component.user.id,
    );
    let mut tmp_file = File::create(tmp_name).unwrap();
    tmp_file
        .write_all(format!("{}\n{}", prompt, url).as_bytes())
        .unwrap();

    Ok(())
}

pub async fn save(ctx: &Context, component: &MessageComponentInteraction) -> Result<()> {
    let name = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect::<String>();

    let url = fs::read_to_string(format!(
        "tmp/{}:{}:{}",
        component.guild_id.unwrap(),
        component.channel_id,
        component.user.id,
    ))
    .unwrap();

    let url = url.split("\n").collect::<Vec<&str>>()[1];

    let user_id = component.user.id;

    component
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredUpdateMessage)
                .interaction_response_data(|message| {
                    message.content("Saving image...");
                    message
                })
        })
        .await
        .unwrap();

    let client = reqwest::Client::new();

    dbg!(&url);

    let url = Url::parse(url).unwrap();

    let image = client
        .get(url.clone())
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec();

    fs::create_dir_all(format!("saved_imagines/{}", user_id)).unwrap();

    let mut file = File::create(format!("saved_imagines/{}/{}.png", user_id, name)).unwrap();

    file.write_all(&image).unwrap();

    component
        .edit_original_interaction_response(&ctx.http, |response| {
            response.embed(|embed| {
                embed.title("Imagine");
                embed.description(SAVE_DESCRIPTION);
                embed.color(Colour::from_rgb(0, 255, 0));
                embed.timestamp(&Timestamp::now());

                embed.image(&url);

                embed
            });

            response.components(|component| {
                component.create_action_row(|row| {
                    row.create_button(|button| {
                        button
                            .label("Retry")
                            .style(ButtonStyle::Primary)
                            .custom_id("imagine_retry")
                    })
                })
            });

            response
        })
        .await?;

    Ok(())
}

async fn _generate<T>(client: &Client, args: T) -> String
where
    T: FnOnce(&mut ImageArgs) -> &mut ImageArgs,
{
    let resp = client.create_image(args).await.unwrap();

    dbg!(&resp);

    resp.get_content(0).await.unwrap()
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
