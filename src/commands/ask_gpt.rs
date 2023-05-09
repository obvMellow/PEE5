use crate::global_config::GlobalConfig;
use crate::Result;
use openai_gpt_rs::{chat::Message as ChatMessage, client::Client, response::Content};
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{
            command::CommandOptionType,
            component::ButtonStyle,
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
                InteractionResponseType,
            },
        },
        Timestamp,
    },
    prelude::Context,
    utils::Colour,
};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) -> Result<()> {
    let question = interaction
        .data
        .options
        .iter()
        .find(|option| option.name == "question")
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    if let CommandDataOptionValue::String(question) = question {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content(format!("Thinking..."));
                        message
                    })
            })
            .await?;

        let key = GlobalConfig::load("config.json").openai_key;
        let client = Client::new(&key);

        let message = ChatMessage {
            role: "user".to_string(),
            content: question.to_string(),
        };

        let resp = client
            .create_chat_completion(|args| args.messages(vec![message]).max_tokens(2048).n(1))
            .await
            .unwrap();

        let content = resp.get_content(0).unwrap();

        interaction
            .edit_original_interaction_response(&ctx.http, |response| {
                response.embed(|embed| {
                    embed
                        .title(format!(r"\> {}", question))
                        .description(format!("```{}```", content))
                        .colour(Colour::from_rgb(69, 69, 69))
                        .footer(|footer| {
                            footer
                                .text("Powered by OpenAI GPT-3")
                                .icon_url("https://cdn.iconscout.com/icon/premium/png-512-thumb/openai-1523664-1290202.png")
                        })
                        .timestamp(Timestamp::now())
                }).components(|components| {
                    components.create_action_row(|row| {
                        row.create_button(|button| {
                            button
                                .label("Support ❤️")
                                .style(ButtonStyle::Link)
                                .url("https://patreon.com/_mellow")
                        }).create_button(|button| {
                            button
                                .label("Vote")
                                .style(ButtonStyle::Link)
                                .url("https://top.gg/bot/1087464844288069722/vote")
                        })
                    })
                })
            })
            .await?;
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ask_gpt")
        .description("Ask GPT-3 a question")
        .create_option(|option| {
            option
                .name("question")
                .description("The question to ask GPT-3")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
