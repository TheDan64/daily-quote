use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode::Accepted;
use hyper_native_tls::NativeTlsClient;
use rustc_serialize::json;

pub struct Bot(i64);

impl Bot {

}

#[derive(RustcEncodable)]
struct SendMessageBody {
    bot_id: String,
    text: String,
}

pub fn send_message(bot_id: String, text: String) -> bool {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let body = SendMessageBody {
        bot_id: bot_id,
        text: text,
    };

    println!{"{}", json::encode(&body).unwrap()}
    let resp = client.post("https://api.groupme.com/v3/bots/post")
                     .body(&json::encode(&body).unwrap())
                     .send()
                     .unwrap();

    resp.status == Accepted
}
