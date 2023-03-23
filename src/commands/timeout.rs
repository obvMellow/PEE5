use crate::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::user::User;
use serenity::model::{permissions, Timestamp};
use serenity::prelude::Context;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let _user = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "user")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let mut user = User::default();

    if let CommandDataOptionValue::User(user_id, _) = _user {
        user = user_id.to_owned();
    }

    let guild_id = interaction.guild_id.unwrap();

    let duration_i64 = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "duration")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap();

    let duration =
        Timestamp::from_unix_timestamp(Timestamp::now().unix_timestamp() + duration_i64).unwrap();

    let timeout = guild_id
        .member(&ctx.http, &user)
        .await
        .unwrap()
        .disable_communication_until_datetime(&ctx.http, duration)
        .await;

    match timeout {
        Ok(_) => {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(format!(
                                "{} has been timed out for {} seconds",
                                user, duration_i64
                            ))
                        })
                })
                .await
                .unwrap();

            Ok(())
        }
        Err(e) => {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(format!("{} could not be timed out: {}", user, e))
                        })
                })
                .await
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("timeout")
        .description("Timeouts a user")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to timeout")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("duration")
                .description("The duration of the timeout")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .default_member_permissions(permissions::Permissions::ADMINISTRATOR)
}
