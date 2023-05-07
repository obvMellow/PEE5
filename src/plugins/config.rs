use pee5::config::{GuildConfig, Plugins};
use serenity::{
    model::prelude::{
        component::ButtonStyle, interaction::message_component::MessageComponentInteraction,
        Message, MessageFlags,
    },
    prelude::Context,
    utils::Colour,
};

use crate::error_constructor;
use crate::Result;

pub async fn run(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let author = &msg.author;
    let member = msg.guild_id.unwrap().member(&ctx, author.id).await.unwrap();
    let guild = msg
        .guild_id
        .unwrap()
        .to_partial_guild(&ctx.http)
        .await
        .unwrap();

    if !guild
        .user_permissions_in(
            &msg.channel(&ctx.http).await.unwrap().guild().unwrap(),
            &member,
        )
        .unwrap()
        .manage_guild()
    {
        let content = error_constructor!("You do not have permission to use this command```");
        msg.reply_ping(&ctx.http, content).await.unwrap();
        return;
    }

    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 2 {
        help(msg, ctx).await;
        return;
    }

    let command = args[1];

    match command {
        "set" => {
            set(msg, ctx, config).await;
        }
        "enable-plugin" => {
            enable_plugin(msg, ctx, config).await;
        }
        "disable-plugin" => {
            disable_plugin(msg, ctx, config).await;
        }
        "reset" => {
            reset(msg, ctx).await;
        }
        "show" => {
            show(msg, ctx, config).await;
        }
        _ => {
            let content =
                error_constructor!(config command, "Invalid command", "expected a valid command");
            msg.reply_ping(&ctx.http, content).await.unwrap();
        }
    }
}

pub async fn show(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let args = msg.content.split(' ').collect::<Vec<&str>>();

    let mut log_channel = "None".to_string();

    if config.get_log_channel_id().is_some() {
        log_channel = format!("<#{}>", config.get_log_channel_id().unwrap());
    }

    if args.len() < 3 {
        msg.channel_id
            .send_message(&ctx.http, |message| {
                message
                    .embed(|embed| {
                        embed
                            .title("Here's the server configuration!")
                            .field(
                                "Plugins",
                                config
                                    .get_plugins()
                                    .iter()
                                    .map(|plugin| plugin.to_str())
                                    .collect::<Vec<&str>>()
                                    .join(", "),
                                false,
                            )
                            .field(
                                "Blacklisted Words",
                                config.get_blacklisted_words().join(", "),
                                false,
                            )
                            .field("Log Channel", log_channel, true)
                    })
                    .flags(MessageFlags::EPHEMERAL)
            })
            .await
            .unwrap();
        return;
    }
}

pub async fn reset(msg: &Message, ctx: &Context) {
    msg.channel_id
        .send_message(&ctx.http, |message| {
            message
                .embed(|embed| {
                    embed
                        .title("Are you sure?")
                        .description("The configuration will be resetted.")
                        .colour(Colour::from_rgb(255, 255, 50))
                })
                .components(|components| {
                    components.create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label("Yes")
                                .style(ButtonStyle::Success)
                                .custom_id("reset_yes")
                        })
                        .create_button(|button| {
                            button
                                .label("No")
                                .style(ButtonStyle::Danger)
                                .custom_id("reset_no")
                        })
                    })
                })
        })
        .await
        .unwrap();
}

pub async fn reset_yes(ctx: &Context, component: &mut MessageComponentInteraction) -> Result<()> {
    let guild_id = *component.guild_id.unwrap().as_u64();

    std::fs::remove_file(format!("guilds/{}.json", guild_id)).unwrap();

    GuildConfig::new(guild_id)
        .save(format!(
            "guilds/{}.json",
            component.guild_id.unwrap().as_u64()
        ))
        .unwrap();

    component
        .message
        .edit(&ctx.http, |edit| {
            edit.embed(|embed| {
                embed
                    .title("Reset successful")
                    .description("The configuration has been resetted.")
                    .colour(Colour::from_rgb(50, 255, 50))
            })
            .components(|components| components)
        })
        .await?;

    Ok(())
}

pub async fn reset_no(ctx: &Context, component: &mut MessageComponentInteraction) -> Result<()> {
    component
        .message
        .edit(&ctx.http, |edit| {
            edit.embed(|embed| {
                embed
                    .title("Reset cancelled")
                    .description("The configuration will not be resetted.")
                    .colour(Colour::from_rgb(255, 50, 50))
            })
            .components(|components| components)
        })
        .await?;

    Ok(())
}

async fn set(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 3 {
        help(msg, ctx).await;
        return;
    }

    let key = args[2];

    match key {
        "log_channel" => {
            if args.len() < 4 {
                let content = error_constructor!(
                    format!("!config set {}", key),
                    "__",
                    "Missing argument",
                    "expected a channel mention"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            let channel_id = args[3].replace("<", "").replace(">", "").replace("#", "");

            if let Err(_) = channel_id.parse::<u64>() {
                let content = error_constructor!(
                    format!("!config set {}", key),
                    channel_id,
                    "Invalid argument",
                    "expected a channel mention"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            config
                .get_log_channel_id_mut()
                .replace(channel_id.parse::<u64>().unwrap());

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Set `{}` to <#{}>", key, channel_id))
                .await
                .unwrap();
        }
        _ => {
            let content =
                error_constructor!(config set key, "Invalid argument", "expected a valid argument");
            msg.reply_ping(&ctx.http, content).await.unwrap();
            return;
        }
    }
}

async fn enable_plugin(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 3 {
        help(msg, ctx).await;
        return;
    }

    let plugin = args[2];

    match plugin {
        "afk" => {
            let plugins = config.get_plugins_mut();

            if plugins.contains(&Plugins::Afk) {
                let content = error_constructor!(
                    "!config enable-plugin",
                    plugin,
                    "Plugin already enabled",
                    "expected a disabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.push(Plugins::Afk);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Enabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "automod" => {
            let plugins = config.get_plugins_mut();

            if plugins.contains(&Plugins::Automod) {
                let content = error_constructor!(
                    "!config enable-plugin",
                    plugin,
                    "Plugin already enabled",
                    "expected a disabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.push(Plugins::Automod);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Enabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "chat" => {
            let plugins = config.get_plugins_mut();

            if plugins.contains(&Plugins::Chat) {
                let content = error_constructor!(
                    "!config enable-plugin",
                    plugin,
                    "Plugin already enabled",
                    "expected a disabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.push(Plugins::Chat);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Enabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "logging" => {
            let plugins = config.get_plugins_mut();

            if plugins.contains(&Plugins::Logging) {
                let content = error_constructor!(
                    "!config enable-plugin",
                    plugin,
                    "Plugin already enabled",
                    "expected a disabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.push(Plugins::Logging);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Enabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "xp" => {
            let plugins = config.get_plugins_mut();

            if plugins.contains(&Plugins::Xp) {
                let content = error_constructor!(
                    "!config enable-plugin",
                    plugin,
                    "Plugin already enabled",
                    "expected a disabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.push(Plugins::Xp);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Enabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        _ => {
            let content = error_constructor!(
                config enable plugin plugin,
                "Invalid argument",
                "expected a valid argument"
            );
            msg.reply_ping(&ctx.http, content).await.unwrap();
            return;
        }
    }
}

async fn disable_plugin(msg: &Message, ctx: &Context, config: &mut GuildConfig) {
    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 3 {
        help(msg, ctx).await;
        return;
    }

    let plugin = args[2];

    match plugin {
        "afk" => {
            let plugins = config.get_plugins_mut();

            if !plugins.contains(&Plugins::Afk) {
                let content = error_constructor!(
                    "!config disable-plugin",
                    plugin,
                    "Plugin already disabled",
                    "expected an enabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.retain(|&x| x != Plugins::Afk);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Disabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "automod" => {
            let plugins = config.get_plugins_mut();

            if !plugins.contains(&Plugins::Automod) {
                let content = error_constructor!(
                    "!config disable-plugin",
                    plugin,
                    "Plugin already disabled",
                    "expected an enabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.retain(|&x| x != Plugins::Automod);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Disabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "chat" => {
            let plugins = config.get_plugins_mut();

            if !plugins.contains(&Plugins::Chat) {
                let content = error_constructor!(
                    "!config disable-plugin",
                    plugin,
                    "Plugin already disabled",
                    "expected an enabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.retain(|&x| x != Plugins::Chat);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Disabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "logging" => {
            let plugins = config.get_plugins_mut();

            if !plugins.contains(&Plugins::Logging) {
                let content = error_constructor!(
                    "!config disable-plugin",
                    plugin,
                    "Plugin already disabled",
                    "expected an enabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.retain(|&x| x != Plugins::Logging);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Disabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        "xp" => {
            let plugins = config.get_plugins_mut();

            if !plugins.contains(&Plugins::Xp) {
                let content = error_constructor!(
                    "!config disable-plugin",
                    plugin,
                    "Plugin already disabled",
                    "expected an enabled plugin"
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            plugins.retain(|&x| x != Plugins::Xp);

            config
                .save(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64()))
                .unwrap();

            msg.reply_ping(&ctx.http, format!("Disabled plugin `{}`", plugin))
                .await
                .unwrap();
        }
        _ => {
            let content = error_constructor!(
                config disable plugin plugin,
                "Invalid argument",
                "expected a valid argument"
            );
            msg.reply_ping(&ctx.http, content).await.unwrap();
            return;
        }
    }
}

async fn help(msg: &Message, ctx: &Context) {
    let content = format!(
        "**!config [KEYWORD] [ARGUMENTS]**
    
**Keywords:**
    `set` - Sets a config value
    `enable-plugin` - Enables a plugin
    `disable-plugin` - Disables a plugin
    `reset` - Resets the configurations to default
    `show` - Shows the current configurations

**Arguments:**
    `log_channel` - Sets the log channel
    
    **Example:**
        `!config set log_channel #general`
        
    `afk` - Enables the AFK plugin
    `automod` - Enables the Automod plugin
    `chat` - Enables the Chat plugin
    `logging` - Enables the Logging plugin
    `xp` - Enables the XP plugin
    
    **Example:**
        `!config enable-plugin afk`"
    );

    msg.reply_ping(&ctx.http, content).await.unwrap();
}
