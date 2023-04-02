use crate::Result;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
    utils::Colour,
};

const HELP_MESSAGE: &str = "PEE5 is a *blazingly fast* Discord bot written in Rust. If you find a bug, please report it on the GitHub repository.";

const HELP: &str = "Display this message.";
const IMAGINE: &str = "Turn your imagination into an image using DALL-E.";
const ASK_GPT: &str = "Ask GPT-3 a question.";
const SAVED_IMAGINES: &str = "Display all of your saved images.";
const AVATAR: &str = "Get a user's avatar.";
const AUTOMOD: &str = "Enable or disable automod.";
const BLACKLIST_WORD: &str = "Blacklist a word.";
const ADD_ROLE: &str = "Add a role to a user.";
const REMOVE_ROLE: &str = "Remove a role from a user.";
const TIMEOUT: &str = "Timeout a user for a certain time.";
const XP: &str = "Display your XP.";
const CONFIG: &str = "Configure the bot.";
const PURGE: &str = "Deletes certain amount of messages.";

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed
                            .title("Help")
                            .description(HELP_MESSAGE)
                            .field("/help", HELP, true)
                            .field("/imagine", IMAGINE, true)
                            .field("/ask-gpt", ASK_GPT, true)
                            .field("/saved-imagines", SAVED_IMAGINES, true)
                            .field("/avatar", AVATAR, true)
                            .field("/automod", AUTOMOD, true)
                            .field("/blacklist-word", BLACKLIST_WORD, true)
                            .field("/add-role", ADD_ROLE, true)
                            .field("/remove-role", REMOVE_ROLE, true)
                            .field("/timeout", TIMEOUT, true)
                            .field("/xp", XP, true)
                            .field("/config", CONFIG, true)
                            .field("/purge", PURGE, true)
                            .color(Colour::from_rgb(0, 255, 0))
                    })
                })
        })
        .await?;

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("Display help message.")
}