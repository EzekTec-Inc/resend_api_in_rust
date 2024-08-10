use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::env;

//NOTE: This could be success or it could emit an error. The only reason of wrapping it here is to simplify the response on the main handler.
//type ServiceResponse = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type ServiceResponse = anyhow::Result<()>;

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
    pub uri: String,
    pub auth: String,
    pub header: String,
    pub body: EmailPayload,
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
    pub fn with_email_payload(&mut self, email_payload: EmailPayload) -> ResendSDK {
        self.body = email_payload;
        self.clone()
        //Self { ..self.clone() }
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

#[tokio::main]
async fn main() -> ServiceResponse {
    dotenv::dotenv().ok();

    // NOTE: Please modify these variables to suite your needs, or adopt a different approach.
    let api_service_uri = "https://api.resend.com/emails".into();
    let api_header = "application/json".into();
    let auth_api_key = env::var("AUTHORIZATION_API_KEY")?;

    let email_to = env::var("TO_EMAIL")?;
    let email_from = "your_company_name <delivered@resend.dev>".into();
    let email_subject = "Demo email from Resend".into();
    let email_html = "<p>Congrats on sending a <strong>custom email</strong> using <strong>Resend</strong> api</p>"
                .into();

    let payload = ResendSDK::init(api_service_uri, api_header, auth_api_key).with_email_payload(
        EmailPayload::new(email_from, email_to, email_subject, email_html),
    );

    let response = payload.send_email().await?;

    println!("{:#?}", response);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_send_email() {
        dotenv::dotenv().ok();

        let test_api_service_uri = "https://api.resend.com/emails".to_owned();
        let test_api_header = "application/json".to_owned();
        let test_auth_api_key =
            env::var("AUTHORIZATION_API_KEY").expect("AUTHORIZATION_API_KEY must be set");

        let test_email_to = env::var("TO_EMAIL").expect("TO_EMAIL must be set");
        let test_email_from = "your_company_name <delivered@resend.dev>".to_owned();
        let test_email_subject = "Demo email from Resend".to_owned();
        let test_email_html = "<p>Congrats on sending a <strong>test email from your unit test [test_send_email()] function</strong> using <strong>Resend</strong> api</p>"
                    .to_owned();
        let test_payload =
            ResendSDK::init(test_api_service_uri, test_auth_api_key, test_api_header)
                .with_email_payload(EmailPayload::new(
                    test_email_from,
                    test_email_to,
                    test_email_subject,
                    test_email_html,
                ));

        let test_response = test_payload.send_email().await;

        assert!(!test_response
            .expect("test_send_email() Error unwrapping response")
            .is_empty());
    }
}
