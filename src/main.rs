use anyhow::{anyhow, Result};
use dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize)]
pub struct ResendResponse {
    message: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload {
    from: String,
    to: String,
    subject: String,
    html: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendPayload {
    uri: String,
    authorization: String,
    header: String,
    body: Payload,
}

impl ResendPayload {
    pub fn new(uri: String, auth: String, head: String, body: Payload) -> Self {
        Self {
            uri,
            authorization: auth,
            header: head,
            body,
        }
    }

    pub async fn send_email(&self) -> Result<String> {
        let client = reqwest::Client::new();
        let res: reqwest::Response = client
            .post(&self.uri)
            .bearer_auth(&self.authorization)
            .header("Content-Type", &self.header)
            .json(&self.body)
            .send()
            .await?;

        // println!("RESPONSE: {:#?}", &res); // NOTE: Please remove this line as it's only here
        // for testing purposes.

        match res.status() {
            reqwest::StatusCode::OK => {
                let response: serde_json::Value = res.json().await?;
                let resend_response = ResendResponse {
                    message: "Success: Email sent.".to_owned(),
                    content: response.to_string(),
                };

                Ok(format!("{:?}", resend_response))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Error: UNAUTHORIZED")),
            _ => Err(anyhow!("Error: All other ERRORS!.")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok();

    let payload = ResendPayload::new(
        "https://api.resend.com/emails".to_owned(),
        env::var("AUTHORIZATION_API_KEY")?,
        "application/json".to_owned(),
        Payload {
            from: "onboarding@resend.dev".to_owned(),
            to: env::var("TO_EMAIL")?,
            subject: "Demo email from Resend".to_owned(),
            html: "<p>Congrats on sending a <strong>custom email</strong> using <strong>Resend</strong> api</p>"
                .to_owned(),
        },
    );

    let response = payload.send_email().await?;

    println!("{:#?}", response);

    Ok(())
}
