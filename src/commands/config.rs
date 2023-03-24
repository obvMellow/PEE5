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

    let _key = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "key")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let _value = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "value")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let mut key = String::new();
    let mut value = String::new();

    if let CommandDataOptionValue::String(key_str) = _key {
        key = key_str.to_owned();
    }

    if let CommandDataOptionValue::String(value_str) = _value {
        value = value_str.to_owned();
    }

    if let Ok(mut config) = serde_json::from_reader::<File, Value>(config_file) {
        config
            .as_object_mut()
            .unwrap()
            .insert(key.clone(), Value::String(value.clone()));

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
                        embed
                            .title("Configuration")
                            .description(format!("Set `{}` to `{}`", key, value))
                            .colour(Colour::LIGHT_GREY)
                    })
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("config")
        .description("Configure the bot")
        .create_option(|option| {
            option
                .name("key")
                .description("The key to set")
                .kind(CommandOptionType::String)
                .add_string_choice("Moderator Role ID", "moderator_role_id")
                .add_string_choice("Admin Role ID", "admin_role_id")
                .required(true)
        })
        .create_option(|option| {
            option
                .name("value")
                .description("The value to set")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
