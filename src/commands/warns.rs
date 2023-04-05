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
        let name = format!(
            "guilds/warns/{}/{}/",
            interaction.guild_id.unwrap(),
            user.id,
        );

        let folder = Path::new(&name);

        let mut warn_reasons = Vec::new();
        let mut warn_ids = Vec::new();

        for entry in std::fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let warn = std::fs::read_to_string(&path).unwrap();

            warn_reasons.push(warn);
            warn_ids.push(path.file_name().unwrap().to_str().unwrap().to_string());
        }

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed
                                .title(format!("Warns for {}", user.tag()))
                                .colour(0x00ff00)
                                .fields(
                                    warn_reasons
                                        .iter()
                                        .zip(warn_ids.iter())
                                        .map(|(reason, id)| (format!("ID: {}", id), reason, false)),
                                )
                        })
                    })
            })
            .await?;
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("warns")
        .description("Get a list of warns for a user")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("user")
                .description("The user to get warns for")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
