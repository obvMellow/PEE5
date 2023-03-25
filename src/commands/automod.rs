use std::fs::File;

use crate::Result;
use serde_json::Value;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::Permissions;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.unwrap();
    let config_file = File::open(format!("guilds/{}.json", guild_id.0)).unwrap();

    let _value = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "value")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let mut value = false;

    if let CommandDataOptionValue::Boolean(value_str) = _value {
        value = *value_str;
    }

    if let Ok(mut config) = serde_json::from_reader::<File, Value>(config_file) {
        config
            .as_object_mut()
            .unwrap()
            .insert("automod".to_string(), value.into());

        serde_json::to_writer_pretty(
            File::create(format!("guilds/{}.json", guild_id.0)).unwrap(),
            &config,
        )
        .unwrap();
    }

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed.title("Success");
                        embed.description(format!("Automod has been enabled."));
                        embed.color(Colour::from_rgb(0, 255, 0))
                    })
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("automod")
        .description("Enable or disable automod.")
        .create_option(|option| {
            option
                .name("value")
                .description("The value to set.")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_GUILD)
}
