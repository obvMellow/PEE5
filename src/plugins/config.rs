use std::fs::File;

use serenity::{json::Value, model::prelude::Message, prelude::Context};

pub async fn run(msg: &Message, ctx: &Context, config: &mut Value) {
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
        _ => {
            let content = _error_constructor(
                &msg.content,
                "Invalid command",
                "expected a valid command",
                -(command.len() as isize + 1),
                None,
                command.len(),
            );
            msg.reply_ping(&ctx.http, content).await.unwrap();
        }
    }
}

async fn set(msg: &Message, ctx: &Context, config: &mut Value) {
    let args = msg.content.split(' ').collect::<Vec<&str>>();

    if args.len() < 3 {
        help(msg, ctx).await;
        return;
    }

    let key = args[2];

    match key {
        "log_channel" => {
            if args.len() < 4 {
                let content = _error_constructor(
                    &msg.content,
                    "No arguments given",
                    "expected an argument",
                    0,
                    None,
                    2,
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            let channel_id = args[3].replace("<", "").replace(">", "").replace("#", "");

            if let Err(_) = channel_id.parse::<u64>() {
                let content = _error_constructor(
                    &msg.content,
                    "Invalid channel id",
                    "expected an unsigned 64-bit integer",
                    -(channel_id.len() as isize + 1),
                    None,
                    channel_id.len(),
                );
                msg.reply_ping(&ctx.http, content).await.unwrap();
                return;
            }

            config.as_object_mut().unwrap().insert(
                "log_channel_id".to_string(),
                Value::String(channel_id.to_string()),
            );

            serde_json::to_writer_pretty(
                File::create(format!("guilds/{}.json", msg.guild_id.unwrap().as_u64())).unwrap(),
                &config,
            )
            .unwrap();

            msg.reply_ping(&ctx.http, format!("Set `{}` to <#{}>", key, channel_id))
                .await
                .unwrap();
        }
        _ => {
            let content = _error_constructor(
                &msg.content,
                "Invalid key",
                "expected a valid key",
                -(key.len() as isize + 1),
                None,
                key.len(),
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

**Arguments:**
    `log_channel` - Sets the log channel
    
    **Example:**
        `!config set log_channel #general`"
    );

    msg.reply_ping(&ctx.http, content).await.unwrap();
}

fn _error_constructor(
    command: &str,
    error_msg: &str,
    reason: &str,
    offset: isize,
    help_msg: Option<&str>,
    upper_arrow_amount: usize,
) -> String {
    let spaces = " ".repeat(command.len());
    let offsetted = " ".repeat((spaces.len() as isize + offset) as usize);
    let mut msg = format!(
        "```
error: {}

    | {}
    | {} {} {}
    \n",
        error_msg,
        command,
        offsetted,
        "^".repeat(upper_arrow_amount),
        reason
    );

    if let Some(help_msg) = help_msg {
        msg.push_str(&format!("=help: {}\n", help_msg));
    }

    msg.push_str("```");

    msg
}
