use crate::plugins;
use crate::Result;
use pee5::config::{GuildConfig, IsPlugin};
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::{InteractionResponseType, MessageFlags};
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::prelude::Context;
use std::fs::File;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let config = GuildConfig::from_reader(
        File::open(format!(
            "guilds/{}.json",
            interaction.guild_id.unwrap().as_u64()
        ))
        .unwrap(),
    )
    .unwrap();

    if !config.get_plugins().xp() {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content(
                                "This plugin is disabled. Contact the server owner to enable it.",
                            )
                            .flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await?;

        return Ok(());
    }

    let user_id = interaction.data.options[0].resolved.as_ref().unwrap();

    let user_id = match user_id {
        CommandDataOptionValue::User(user, _) => user.id.0,
        _ => return Ok(()),
    };

    let user = plugins::xp::get_user(user_id);

    let user = match user {
        Some(u) => u.level,
        None => 0,
    };

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content(format!("{} is level {}", interaction.user, user));
                    message
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("level")
        .description("Get a users' level")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to get the level of")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
