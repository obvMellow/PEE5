use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use crate::Result;
use pee5::config::{GuildConfig, IsPlugin};
use rand::{distributions::Alphanumeric, Rng};
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
                MessageFlags,
            },
            ChannelType, PermissionOverwrite, PermissionOverwriteType, RoleId, UserId,
        },
        Permissions,
    },
    prelude::{Context, Mentionable},
};

const CHAT_PATH: &str = "guilds/chats";
const BOT_ID: u64 = 1087464844288069722;

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let config = GuildConfig::from_reader(
        File::open(format!(
            "guilds/{}.json",
            interaction.guild_id.unwrap().as_u64()
        ))
        .unwrap(),
    )
    .unwrap();

    if !config.get_plugins().chat() {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content("This server has chat disabled.")
                            .flags(MessageFlags::EPHEMERAL)
                    })
            })
            .await?;
        return Ok(());
    }

    let private = interaction.data.options.iter().any(|option| {
        option.name == "private" && option.value.as_ref().unwrap().as_bool().unwrap_or_default()
    });

    let channel_name = "chat-".to_string()
        + &rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect::<String>();

    let priv_perms = vec![
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(interaction.user.id),
        },
        PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Role(RoleId(interaction.guild_id.unwrap().0)),
        },
        PermissionOverwrite {
            allow: Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Member(UserId(BOT_ID)),
        },
    ];

    let channel = match private {
        true => {
            interaction
                .guild_id
                .unwrap()
                .create_channel(&ctx.http, |channel| {
                    channel
                        .name(channel_name)
                        .kind(ChannelType::Text)
                        .permissions(priv_perms)
                })
                .await?
        }
        false => {
            interaction
                .guild_id
                .unwrap()
                .create_channel(&ctx.http, |channel| {
                    channel.name(channel_name).kind(ChannelType::Text)
                })
                .await?
        }
    };

    std::fs::create_dir_all(CHAT_PATH)?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("{}/{}", CHAT_PATH, interaction.guild_id.unwrap()))
        .unwrap();
    file.write_all(channel.id.as_u64().to_string().as_bytes())?;

    channel.send_message(&ctx.http, |message| {
        message
            .content(format!("Hello, {}!\n\nYou can chat with me here.\n\nCommands:
        **!end**: Ends the conversation and deletes the channel.
        **!rename [NAME]**: Changes the channels name. If the name includes spaces, only the word before the first space will be used.
        **!clear**: Clears the messages in the channel. Use this command if the bot starts saying you requested too many tokens.", interaction.user.mention()))
    }).await?;

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(format!(
                            "Created a channel for chatting with the bot: {}",
                            channel.mention()
                        ))
                        .flags(MessageFlags::EPHEMERAL)
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("chat")
        .description("Creates a channel for chatting with the bot")
        .create_option(|option| {
            option
                .name("private")
                .kind(CommandOptionType::Boolean)
                .description("If the channel should be private or not")
                .required(false)
        })
}
