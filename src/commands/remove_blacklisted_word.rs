use std::fs::File;

use crate::Result;
use pee5::config::GuildConfig;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{application_command::{
                ApplicationCommandInteraction, CommandDataOptionValue,
            }, InteractionResponseType},
        },
        Permissions,
    },
    prelude::Context, utils::Colour,
};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
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

    let mut config = GuildConfig::from_reader(
        File::open(format!("guilds/{}.json", interaction.guild_id.unwrap().0)).unwrap(),
    )
    .unwrap();

    let blacklist = config.get_blacklisted_words_mut();

    blacklist.retain(|word| word != &value);

    config
        .save(format!("guilds/{}.json", interaction.guild_id.unwrap().0))
        .unwrap();

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed.title("Success");
                        embed.description(format!("The word \"{}\" has been removed from blacklist.", value));
                        embed.color(Colour::from_rgb(102, 255, 102));
                        embed
                    })
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove_blacklisted_word")
        .description("Remove a blacklisted word from server.")
        .create_option(|option| {
            option
                .name("value")
                .description("The word to blacklist.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_GUILD)
}
