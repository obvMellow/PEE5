use crate::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Context;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let msg = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "message")
        .unwrap()
        .value
        .as_ref()
        .unwrap();

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content(msg))
        })
        .await
        .unwrap();

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("echo")
        .description("Echoes back the message you send")
        .create_option(|option| {
            option
                .name("message")
                .description("The message to echo")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
