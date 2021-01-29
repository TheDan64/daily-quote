use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
struct SendMessageBody<'a> {
    bot_id: &'a str,
    text: &'a str,
}

pub struct Bot<'s> {
    client: Client,
    id: &'s str,
}

impl<'s> Bot<'s> {
    pub fn new(id: &'s str) -> Self {
        Bot {
            client: Client::new(),
            id,
        }
    }

    pub async fn send_message(&self, text: &'s str) -> Result<()> {
        let body = SendMessageBody {
            bot_id: &self.id,
            text,
        };
        let _resp = self.client.post("https://api.groupme.com/v3/bots/post")
            .body(to_string(&body)?)
            .send()
            .await?;

        Ok(())
    }
}
