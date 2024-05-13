use std::fs::File;

use crate::{plugins, Result};
use pee5::config::{GuildConfig, IsPlugin};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::{InteractionResponseType, MessageFlags};
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let config = GuildConfig::from_reader(
        File::open(format!("guilds/{}.json", interaction.guild_id.unwrap().0)).unwrap(),
    )
    .unwrap();

    if !config.get_plugins().xp() {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content("This plugin is disabled")
                            .flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await?;

        return Ok(());
    }

    let xp = plugins::xp::get_user(interaction.user.id.0);

    let xp = match xp {
        Some(u) => u.xp,
        None => 0,
    };

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed.description(format!("{} has {} xp", interaction.user, xp));
                        embed.color(Colour::from_rgb(0, 255, 0));

                        embed
                    });

                    message
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("xp").description("See your current xp")
}
