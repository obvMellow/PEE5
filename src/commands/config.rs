use crate::Result;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::Permissions;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed
                            .title("This command is deprecated")
                            .description("Please use the `!config` command instead")
                            .colour(Colour::RED)
                    })
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("config")
        .description("[Deprecated] Configure the bot")
        .create_option(|option| {
            option
                .name("key")
                .description("The key to set")
                .kind(CommandOptionType::String)
                .add_string_choice("Log Channel ID", "log_channel_id")
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
