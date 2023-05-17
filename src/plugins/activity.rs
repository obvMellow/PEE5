use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;

const ALLOWED_USERS: &[u64] = &[853618501445222420];

pub async fn run(ctx: &Context, msg: &Message) {
    if !msg.content.starts_with("!activity") {
        return;
    }

    if !ALLOWED_USERS.contains(&msg.author.id.0) {
        return;
    }

    let mut args = msg.content.split_whitespace();

    let mode = match args.nth(1) {
        Some(v) => v,
        None => return,
    };

    let activity = args.collect::<Vec<&str>>().join(" ");

    let mode = match mode {
        "playing" => Activity::playing(activity),
        "listening" => Activity::listening(activity),
        "watching" => Activity::watching(activity),
        "competing" => Activity::competing(activity),
        "streaming" => Activity::streaming(activity, "https://github.com/obvMellow/PEE5"),
        _ => return,
    };

    ctx.set_activity(mode).await;
}
