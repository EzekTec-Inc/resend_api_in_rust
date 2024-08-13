use anyhow::anyhow;
use serde::{Deserialize, Serialize};

pub trait ResendSDKInterface {
    fn init(uri: String, header: String, auth: String) -> Self;
    fn with_email_payload(&mut self, email_payload: EmailPayload) -> Self;
    async fn send_email(&self) -> Result<String, anyhow::Error>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendSDKResponse {
    message: String,
    content: String,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmailPayload {
    from: String,
    to: String,
    subject: String,
    html: String,
}
impl EmailPayload {
    pub fn new(from: String, to: String, subject: String, html: String) -> Self {
        Self {
            from,
            to,
            subject,
            html,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendSDK {
    uri: String,
    auth: String,
    header: String,
    body: EmailPayload,
}

impl ResendSDKInterface for ResendSDK {
    /// Initializes the SDK.
    fn init(uri: String, header: String, auth: String) -> Self {
        Self {
            uri,
            auth,
            header,
            body: EmailPayload::default(),
        }
    }
    fn with_email_payload(&mut self, email_payload: EmailPayload) -> Self {
        self.body = email_payload;
        self.clone()
    }
    /// Returns the send email of this [`ResendSDK`].
    ///
    /// # Errors [`anyhow::Error`] // the anyhow::Error is returned instead of returning a
    /// reqwest::Error, which we don't want the user of the library to concern themselves with.
    ///
    /// This function will return an error if the api-endpoint is not reachable.
    async fn send_email(&self) -> Result<String, anyhow::Error> {
        let client = reqwest::Client::new();
        let res: reqwest::Response = client
            .post(&self.uri)
            .bearer_auth(&self.auth)
            .header("Content-Type", &self.header)
            .json(&self.body)
            .send()
            .await?;
        match res.status() {
            reqwest::StatusCode::OK => {
                let response: serde_json::Value = res.json().await?;
                let resend_response = ResendSDKResponse {
                    message: "Success: Email sent.".to_owned(),
                    content: response.to_string(),
                    status: "200".to_owned(),
                };
                Ok(format!("{:?}", resend_response))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Error: UNAUTHORIZED")),
            _ => Err(anyhow!("Error: All other send ERRORS!.")),
        }
    }
}
