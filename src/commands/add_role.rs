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

    let add_role = guild_id
        .member(&ctx.http, &member)
        .await
        .unwrap()
        .add_role(&ctx.http, role.as_ref().unwrap())
        .await;

    match add_role {
        Ok(_) => {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed
                                    .title("Role Added")
                                    .description(format!(
                                        "Added role {} to {}",
                                        role.unwrap(),
                                        member
                                    ))
                                    .color(Colour::ROHRKATZE_BLUE)
                            })
                        })
                })
                .await?;
        }
        Err(e) => {
            interaction
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.embed(|embed| {
                                embed.title("Error");
                                embed.description(format!("Error: {}", e));
                                embed.color(Colour::RED)
                            })
                        })
                })
                .await?;
        }
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_role")
        .description("Adds a role to a member")
        .create_option(|option| {
            option
                .name("member")
                .description("The member to add the role to")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("role")
                .description("The role to add to the user")
                .kind(CommandOptionType::Role)
                .required(true)
        })
        .default_member_permissions(Permissions::MANAGE_ROLES)
}
