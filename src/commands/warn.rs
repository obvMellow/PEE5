use std::{fs::File, io::Write, path::Path};

use crate::Result;
use rand::{distributions::Alphanumeric, Rng};
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::{CommandOptionType, CommandType},
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
                InteractionResponseType,
            },
        },
        Permissions,
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
        let warn_id = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();

        let name = format!(
            "guilds/warns/{}/{}/",
            interaction.guild_id.unwrap(),
            user.id,
        );

        let folder = Path::new(&name);

        std::fs::create_dir_all(folder).unwrap();

        let mut warn = File::create(folder.join(&warn_id)).unwrap();

        let reason = interaction
            .data
            .options
            .iter()
            .find(|option| option.name == "reason")
            .unwrap()
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap();

        warn.write_all(reason.as_bytes()).unwrap();

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|data| {
                        data.embed(|embed| {
                            embed.title("Warned user");
                            embed.field("Reason", reason, true);
                            embed.colour(Colour::BLUE);
                            embed
                        })
                    })
            })
            .await?;
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("warn")
        .description("Warn a user")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("user")
                .description("The user to warn")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("reason")
                .description("The reason for the warning")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .default_member_permissions(Permissions::MODERATE_MEMBERS)
}
