use anyhow::Result;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode::Accepted;
use hyper_native_tls::NativeTlsClient;
use serde::Serialize;
use serde_json::to_string;

#[derive(Serialize)]
struct SendMessageBody<'a> {
    bot_id: &'a str,
    text: &'a str,
}

pub struct Bot<'s> {
    id: &'s str,
}

impl<'s> Bot<'s> {
    pub fn new(id: &'s str) -> Self {
        Bot {
            id,
        }
    }

    pub fn send_message(&self, text: &'s str) -> Result<()> {
        let ssl = NativeTlsClient::new()?;
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        let body = SendMessageBody {
            bot_id: &self.id,
            text,
        };

        dbg!(&to_string(&body)?);

        let resp = client.post("https://api.groupme.com/v3/bots/post")
            .body(&to_string(&body)?)
            .send()?;

        resp.status == Accepted;
        Ok(())
    }
}
