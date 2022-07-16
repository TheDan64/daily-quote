use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::Client;
use serde::Serialize;
use structopt::StructOpt;

use std::io::{stdin, Read};

#[derive(Debug, StructOpt)]
enum WebhookCmd {
    #[structopt(name = "send")]
    Send {
        webhook_id: String,
        webhook_token: String,
        message: Option<String>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "discord")]
enum Opt {
    #[structopt(name = "webhook")]
    Webhook(WebhookCmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    dotenv().context("Failed to init dotenv")?;

    match opt {
        Opt::Webhook(WebhookCmd::Send {
            webhook_id,
            webhook_token,
            message,
        }) => execute_webhook_message(&webhook_id, &webhook_token, message).await?,
    }

    Ok(())
}

// Other fields are available that we do not currently support
#[derive(Serialize)]
struct ExecuteWebhookBody {
    content: String,
}

/// https://discord.com/developers/docs/resources/webhook#execute-webhook
async fn execute_webhook_message(
    webhook_id: &str,
    webhook_token: &str,
    message: Option<String>,
) -> Result<()> {
    let message = match message {
        Some(msg) => msg,
        None => readable_to_string(stdin())?,
    };
    let body = ExecuteWebhookBody { content: message };
    let endpoint = format!("https://discord.com/api/webhooks/{webhook_id}/{webhook_token}");
    let _response = Client::new().post(endpoint).json(&body).send().await?;

    Ok(())
}

fn readable_to_string<R: Read>(mut readable: R) -> Result<String> {
    let mut input_string = String::new();

    readable.read_to_string(&mut input_string)?;

    Ok(input_string)
}
