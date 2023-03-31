use crate::global_config::GlobalConfig;
use crate::Result;
use openai_gpt_rs::{args::ChatArgs, client::Client, response::Content};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
            InteractionResponseType,
        },
    },
    prelude::Context,
};
use std::collections::HashMap;

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

        let mut messages: Vec<HashMap<String, String>> = Vec::new();
        let mut message: HashMap<String, String> = HashMap::new();
        message.insert("role".to_string(), "user".to_string());
        message.insert("content".to_string(), question.to_string());

        messages.push(message);

        let args = ChatArgs::new(messages, Some(2048), Some(1), None, None, None, None);
        let content = client
            .create_chat_completion(&args)
            .await
            .unwrap()
            .get_json()
            .await
            .unwrap();

        dbg!(&content);

        let content = content
            .as_object()
            .unwrap()
            .get("choices")
            .unwrap()
            .as_array()
            .unwrap()
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()
            .get("message")
            .unwrap()
            .as_object()
            .unwrap()
            .get("content")
            .unwrap()
            .as_str()
            .unwrap();

        interaction
            .edit_original_interaction_response(&ctx.http, |response| {
                response.content(content);
                response
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
