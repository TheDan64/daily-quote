pub mod bot;

use std::env;
use std::io::{stdin, Read};

use anyhow::{Context, Result};
use dotenv::dotenv;
use structopt::StructOpt;

use crate::bot::Bot;

#[derive(Debug, StructOpt)]
enum BotCmd {
    /// Either provide a string or else read from stdin
    #[structopt(name = "send")]
    Send {
        message: Option<String>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "groupme")]
enum Opt {
    #[structopt(name = "bot")]
    Bot(BotCmd),
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    dotenv().context("Failed to init dotenv")?;

    match opt {
        Opt::Bot(BotCmd::Send { message }) => send_bot_message(message).await?,
    }

    Ok(())
}

async fn send_bot_message(message: Option<String>) -> Result<()> {
    let bot_id = env::var("BOT_ID").context("Did not find envvar BOT_ID")?;
    let bot = Bot::new(&bot_id);
    let message = match message {
        Some(msg) => msg,
        None => readable_to_string(stdin())?,
    };
    bot.send_message(&message).await?;

    Ok(())
}

fn readable_to_string<R: Read>(mut readable: R) -> Result<String> {
    let mut input_string = String::new();

    readable.read_to_string(&mut input_string)?;

    Ok(input_string)
}
