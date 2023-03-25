use std::fs::File;

use crate::Result;
use serde_json::Value;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let config_file =
        File::open(format!("guilds/{}.json", interaction.guild_id.unwrap().0)).unwrap();
    let config = serde_json::from_reader::<File, Value>(config_file).unwrap();

    let xp = config
        .as_object()
        .unwrap()
        .get("users")
        .unwrap()
        .as_object()
        .unwrap()
        .get(&interaction.user.id.to_string())
        .unwrap()
        .as_u64()
        .unwrap();

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
