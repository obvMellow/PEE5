use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::global_config::GlobalConfig;
use crate::Result;
use std::result::Result as StdResult;

use openai_gpt_rs::{
    args::{ImageArgs, ImageResponseFormat, ImageSize},
    client::Client,
    response::Content,
};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Url;
use serenity::builder::CreateComponents;
use serenity::model::channel::AttachmentType;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::{
    application_command::ApplicationCommandInteraction, MessageFlags,
};
use serenity::model::prelude::{
    command::CommandOptionType, interaction::message_component::MessageComponentInteraction,
};
use serenity::{builder::CreateApplicationCommand, model::prelude::component::ButtonStyle};

use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub struct Error {
    pub message: String,
}

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

    let (timestamp, _new, mut file, contents) = should_continue(interaction.user.id.as_u64());

    if !_new && timestamp - contents.replace("\0", "").parse::<i64>().unwrap() < 30 {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .embed(|embed| {
                                embed
                                    .title("You are going too fast!")
                                    .description("You can use this command once every 30 seconds")
                                    .colour(Colour::from_rgb(255, 0, 0))
                            })
                            .flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await
            .unwrap();

        return Ok(());
    }

    file.set_len(0).unwrap();
    file.write_all(timestamp.to_string().as_bytes()).unwrap();

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Generating image..."))
        })
        .await
        .unwrap();

    let url = _generate(&client, |args| {
        args.prompt(_prompt)
            .n(1)
            .size(ImageSize::Small)
            .response_format(ImageResponseFormat::Url)
    })
    .await;

    match url {
        Ok(url) => {
            let image = download(&Url::parse(&url).unwrap()).await;

            let name = format!("{}.png", interaction.user.id.as_u64());

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&name)
                .unwrap();

            file.write_all(&image).unwrap();

            // We drop the file here because it won't get written to disk otherwise and
            // Discord API will be confused.
            drop(file);

            let attachment = AttachmentType::File {
                filename: name.clone(),
                file: &tokio::fs::File::open(&name).await.unwrap(),
            };

            let msg = interaction
                .create_followup_message(&ctx.http, |message| {
                    message
                        .content("Here's your image!")
                        .add_file(attachment)
                        .components(|component| components(component))
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
        }
        Err(error) => {
            interaction
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.embed(|embed| {
                        embed.title("Imagine");
                        embed.description(error.message);
                        embed.color(Colour::from_rgb(255, 0, 0));
                        embed.timestamp(&Timestamp::now());

                        embed
                    });

                    response
                })
                .await?;
        }
    }

    Ok(())
}

fn should_continue(id: &u64) -> (i64, bool, File, String) {
    fs::create_dir_all("imagine_requests/").unwrap();
    let s = format!("imagine_requests/{}", id);
    let file_path = Path::new(&s);

    let timestamp = Timestamp::now().unix_timestamp();
    let mut _new = false;

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    if file.metadata().unwrap().len() == 0 {
        file.write_all(timestamp.to_string().as_bytes()).unwrap();
        _new = true;
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    (timestamp, _new, file, contents)
}

pub async fn retry(ctx: &Context, component: &mut MessageComponentInteraction) -> Result<()> {
    let prompt = fs::read_to_string(format!(
        "tmp/{}:{}:{}:{}",
        component.guild_id.unwrap(),
        component.channel_id,
        component.user.id,
        component.message.id,
    ))
    .unwrap();

    let (timestamp, _new, mut file, contents) = should_continue(component.user.id.as_u64());

    component
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredUpdateMessage)
                .interaction_response_data(|message| message.content("Generating image..."))
        })
        .await
        .unwrap();

    if !_new && timestamp - contents.replace("\0", "").parse::<i64>().unwrap() < 30 {
        component
            .create_followup_message(&ctx.http, |message| {
                message
                    .content("You can use this command every 30 seconds.")
                    .flags(MessageFlags::EPHEMERAL)
            })
            .await?;

        return Ok(());
    }

    file.set_len(0).unwrap();
    file.write_all(timestamp.to_string().as_bytes()).unwrap();

    let prompt = prompt.split("\n").collect::<Vec<&str>>()[0];

    let key = GlobalConfig::load("config.json").openai_key;

    let client = Client::new(&key);

    component
        .message
        .edit(&ctx.http, |edit| {
            edit.content("Generating another image...")
        })
        .await?;

    let url = _generate(&client, |args| {
        args.prompt(prompt)
            .n(1)
            .size(ImageSize::Small)
            .response_format(ImageResponseFormat::Url)
    })
    .await;

    match url {
        Ok(url) => {
            let image = download(&Url::parse(&url).unwrap()).await;

            let name = format!("{}.png", component.user.id.as_u64());

            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(&name)
                .unwrap();
            file.write_all(&image).unwrap();
            // We drop the file here because it won't get written to disk otherwise and
            // Discord API will be confused.
            drop(file);

            let attachment = AttachmentType::File {
                filename: name.clone(),
                file: &tokio::fs::File::open(&name).await.unwrap(),
            };

            let attachment_id = component.message.attachments[0].id;

            component
                .message
                .edit(&ctx.http, |edit| {
                    edit.content("Here's your image!")
                        .remove_existing_attachment(attachment_id)
                        .attachment(attachment)
                })
                .await?;

            fs::remove_file(name).unwrap();

            let tmp_name = format!(
                "tmp/{}:{}:{}:{}",
                component.guild_id.unwrap(),
                component.channel_id,
                component.user.id,
                component.message.id,
            );
            let mut tmp_file = File::create(tmp_name).unwrap();
            tmp_file
                .write_all(format!("{}\n{}", prompt, url).as_bytes())
                .unwrap();
        }
        Err(error) => {
            component
                .edit_original_interaction_response(&ctx.http, |response| {
                    response.embed(|embed| {
                        embed.title("Imagine");
                        embed.description(error.message);
                        embed.color(Colour::from_rgb(255, 0, 0));
                        embed.timestamp(&Timestamp::now());

                        embed
                    });

                    response
                })
                .await?;
        }
    }

    Ok(())
}

async fn download(url: &Url) -> Vec<u8> {
    reqwest::get(url.clone())
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec()
}

pub async fn save(ctx: &Context, component: &mut MessageComponentInteraction) -> Result<()> {
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

    let url = Url::parse(url.split("\n").collect::<Vec<&str>>()[1]).unwrap();

    let user_id = component.user.id;

    component
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::DeferredUpdateMessage)
        })
        .await?;

    let image = download(&url).await;

    fs::create_dir_all(format!("saved_imagines/{}", user_id)).unwrap();

    let mut file = File::create(format!("saved_imagines/{}/{}.png", user_id, name)).unwrap();

    file.write_all(&image).unwrap();

    component
        .message
        .edit(&ctx.http, |edit| {
            edit.content("Here's your image!\n\n**Image Saved!**")
                .components(|component| components_without_save(component))
        })
        .await?;

    Ok(())
}

async fn _generate<T>(client: &Client, args: T) -> StdResult<String, Error>
where
    T: FnOnce(&mut ImageArgs) -> &mut ImageArgs,
{
    let resp = client.create_image(args).await;

    match resp {
        Ok(image) => Ok(image.get_content(0).unwrap()),
        Err(error) => Err(Error {
            message: error.message,
        }),
    }
}

fn components(component: &mut CreateComponents) -> &mut CreateComponents {
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
        .create_button(|button| {
            button
                .label("Support ❤️")
                .style(ButtonStyle::Link)
                .url("https://patreon.com/_mellow")
        })
        .create_button(|button| {
            button
                .label("Vote")
                .style(ButtonStyle::Link)
                .url("https://top.gg/bot/1087464844288069722/vote")
        })
    })
}

fn components_without_save(component: &mut CreateComponents) -> &mut CreateComponents {
    component.create_action_row(|row| {
        row.create_button(|button| {
            button
                .label("Retry")
                .style(ButtonStyle::Primary)
                .custom_id("imagine_retry")
        })
        .create_button(|button| {
            button
                .label("Support ❤️")
                .style(ButtonStyle::Link)
                .url("https://patreon.com/_mellow")
        })
        .create_button(|button| {
            button
                .label("Vote")
                .style(ButtonStyle::Link)
                .url("https://top.gg/bot/1087464844288069722/vote")
        })
    })
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
