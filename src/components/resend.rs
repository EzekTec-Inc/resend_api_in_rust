use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResendSDKResponse {
    message: String,
    content: String,
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

impl ResendSDK {
    /// Initializes the SDK.
    pub fn init(uri: String, header: String, auth: String) -> Self {
        Self {
            uri,
            auth,
            header,
            body: EmailPayload::default(),
        }
    }
    /// Sets the email payload.
    pub fn with_email_payload(&mut self, email_payload: EmailPayload) -> Self {
        self.body = email_payload;
        self.clone()
    }

    /// Returns the send email of this [`ResendSDK`].
    ///
    /// # Errors [`reqwest::Error`]
    ///
    /// This function will return an error if the api-endpoint is not reachable.
    pub async fn send_email(&self) -> Result<String, anyhow::Error> {
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
                };
                Ok(format!("{:?}", resend_response))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Error: UNAUTHORIZED")),
            _ => Err(anyhow!("Error: All other send ERRORS!.")),
        }
    }
}
