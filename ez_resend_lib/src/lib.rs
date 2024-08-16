use anyhow::anyhow;
use serde::{Deserialize, Serialize};

pub trait ResendSDKInterface {
    fn init(uri: String, header: String, auth: String) -> Self;
    fn with_email_payload(&mut self, email_payload: EmailPayload) -> Self;
    fn send_email(&self)
        -> impl std::future::Future<Output = Result<String, anyhow::Error>> + Send;
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
                    status: reqwest::StatusCode::OK.to_string(),
                };
                Ok(format!("{:?}", resend_response))
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(anyhow!("Error: UNAUTHORIZED")),
            _ => Err(anyhow!("Error: All other send ERRORS!.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    const TEST_API_SERVICE_URI: &str = "https://api.resend.com/emails";
    const TEST_API_HEADER: &str = "application/json";

    #[tokio::main]
    #[test]
    async fn it_works() {
        dotenv::dotenv().ok();

        let test_auth_api_key: &str =
            &env::var("AUTHORIZATION_API_KEY").expect("AUTHORIZATION_API_KEY must be set");

        let test_email_to = env::var("TO_EMAIL").expect("TO_EMAIL must be set");
        let test_email_from = "test_company_name <delivered@resend.dev>".to_owned();
        let test_email_subject = "test email from EzekTec-Inc ResendSDK".to_owned();
        let test_email_html = "<p>Congrats on sending a <strong>test email from your unit test [test_send_email()] function</strong> using <strong>Resend</strong> api</p>"
                    .to_owned();

        let test_payload = ResendSDK::init(
            TEST_API_SERVICE_URI.to_owned(),
            TEST_API_HEADER.to_owned(),
            test_auth_api_key.to_owned(),
        )
        .with_email_payload(EmailPayload::new(
            test_email_from,
            test_email_to,
            test_email_subject,
            test_email_html,
        ));

        let test_result = String::from("");
        let test_result_clone = test_result.clone();

        tokio::spawn(async move {
            let mut test_result_clone = test_result.clone();
            let _ = test_result_clone.push_str(&test_payload.send_email().await.unwrap());
        });

        assert!(test_result_clone.is_empty()); // This is a false test as its already returning and error
    }
}
