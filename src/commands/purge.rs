use crate::Result;
use serenity::model::application::interaction::MessageFlags;
use serenity::model::permissions::Permissions;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType, CommandType},
        interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    prelude::Context,
};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let amount = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "amount")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap();

    let channel = interaction
        .channel_id
        .to_channel(&ctx.http)
        .await
        .unwrap()
        .guild()
        .unwrap();

    let messages = channel
        .messages(&ctx.http, |retriever| retriever.limit(amount as u64))
        .await
        .unwrap();

    let mut ids = Vec::new();

    for message in messages {
        ids.push(message.id);
    }

    channel.delete_messages(&ctx.http, &ids).await?;

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(format!("Deleted {} messages", amount))
                        .flags(MessageFlags::EPHEMERAL)
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("purge")
        .description("Purge messages")
        .kind(CommandType::ChatInput)
        .create_option(|option| {
            option
                .name("amount")
                .description("The amount of messages to purge")
                .min_int_value(2)
                .max_int_value(100)
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_MESSAGES)
}
