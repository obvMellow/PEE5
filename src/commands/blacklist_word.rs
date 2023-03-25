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

    let mut value = String::new();

    if let CommandDataOptionValue::String(value_str) = _value {
        value = value_str.to_owned();
    }

    if let Ok(mut config) = serde_json::from_reader::<File, Value>(config_file) {
        let blacklist = config
            .as_object_mut()
            .unwrap()
            .get_mut("blacklisted_words")
            .unwrap()
            .as_array_mut()
            .unwrap();

        blacklist.push(Value::String(value.clone()));

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
                        embed.description(format!("The word \"{}\" has been blacklisted.", value));
                        embed.color(Colour::from_rgb(0, 255, 0))
                    })
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("blacklist_word")
        .description("Blacklist a word from being used in the server.")
        .create_option(|option| {
            option
                .name("value")
                .description("The word to blacklist.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_GUILD)
}
