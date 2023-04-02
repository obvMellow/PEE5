use crate::Result;
use serde_json::Value;
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
    utils::Colour,
};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let _user = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "user")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::User(user, _) = _user {
        let server = match interaction
            .data
            .options
            .iter()
            .find(|option| option.name == "server")
        {
            Some(v) => v
                .value
                .as_ref()
                .unwrap_or_else(|| &Value::Bool(true))
                .as_bool()
                .unwrap(),
            None => true,
        };

        let mut url = user.face();

        if server {
            url = interaction
                .guild_id
                .unwrap()
                .member(&ctx.http, user)
                .await
                .unwrap()
                .avatar_url()
                .unwrap_or_else(|| user.face());
        }

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed
                                .title(format!("Here is {}'s avatar", user.name))
                                .image(url)
                                .color(Colour::from_rgb(0, 255, 0))
                        });
                        message
                    })
            })
            .await?;
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("avatar")
        .description("Get a user's avatar")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to get the avatar of")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("server")
                .description("Whether to get the avatar from the server or not")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}
