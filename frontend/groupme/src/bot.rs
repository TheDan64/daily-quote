use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode::Accepted;
use hyper_native_tls::NativeTlsClient;
use rustc_serialize::json;
use std::borrow::Cow;

#[derive(RustcEncodable)]
struct SendMessageBody<'a> {
    bot_id: &'a str,
    text: &'a str,
}

pub struct Bot<'a> {
    id: Cow<'a, str>
}

impl<'a> Bot<'a> {
    pub fn new<T>(id: T) -> Self where T: Into<Cow<'a, str>> {
        Bot {
            id: id.into(),
        }
    }

    pub fn send_message<T>(&self, text: T) -> bool where T: Into<Cow<'a, str>> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        let body = SendMessageBody {
            bot_id: &self.id,
            text: &text.into(),
        };

        println!{"{}", json::encode(&body).unwrap()}
        let resp = client.post("https://api.groupme.com/v3/bots/post")
            .body(&json::encode(&body).unwrap())
            .send()
            .unwrap();

        resp.status == Accepted
    }
}
