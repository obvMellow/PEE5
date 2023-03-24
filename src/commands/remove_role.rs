use crate::Result;
use serenity::builder::CreateApplicationCommand;
use serenity::model::guild::Role;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::user::User;
use serenity::model::Permissions;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let guild_id = interaction.guild_id.unwrap();

    let _member = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "member")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let _role = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "role")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let mut member = User::default();
    let mut role: Option<Role> = None;

    if let CommandDataOptionValue::User(user_id, _) = _member {
        member = user_id.to_owned();
    }

    if let CommandDataOptionValue::Role(role_id) = _role {
        role = Some(role_id.to_owned());
    }

    let roles = guild_id.member(&ctx.http, &member).await.unwrap().roles;

    let does_have_role = roles.contains(&role.as_ref().unwrap().id);

    if !does_have_role {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|embed| {
                            embed.title("Error");
                            embed.description(format!(
                                "{} does not have the role {}",
                                member,
                                role.as_ref().unwrap()
                            ));
                            embed.color(Colour::RED);

                            embed
                        });

                        message
                    })
            })
            .await?;

        return Ok(());
    }

    let remove_role = guild_id
        .member(&ctx.http, &member)
        .await
        .unwrap()
        .remove_role(&ctx.http, role.as_ref().unwrap())
        .await;

    match remove_role {
        Ok(_) => {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed
                                    .title("Role removed")
                                    .description(format!(
                                        "Removed role {} from {}",
                                        role.clone().unwrap(),
                                        member
                                    ))
                                    .colour(Colour::BLITZ_BLUE)
                            })
                        })
                })
                .await
                .unwrap();
        }
        Err(_) => {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed
                                    .title("Error")
                                    .description("Failed to remove role")
                                    .colour(Colour::RED)
                            })
                        })
                })
                .await
                .unwrap();
        }
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove_role")
        .description("Remove a role from a member")
        .create_option(|option| {
            option
                .name("member")
                .description("The member to remove the role from")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("role")
                .description("The role to remove from the member")
                .kind(CommandOptionType::Role)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_ROLES)
}
