use serde::Serialize;

pub fn notify(codes: &[String], slack_url: &str) -> Result<(), Box<std::error::Error>> {
    let cm = codes.join("\n");
    let msg = SlackMessage::new(&cm);
    let body = serde_json::to_string(&msg)?;
    //println!("{}", body);
    let client = reqwest::Client::new();
    // do I care about the response from slack?
    let _res = client.post(slack_url).body(body).send()?;

    Ok(())
}

#[derive(Serialize)]
pub struct SlackMessage {
    pub text: String,
}

impl SlackMessage {
    pub fn new(text: &str) -> SlackMessage {
        SlackMessage {
            text: text.to_string(),
        }
    }
}
