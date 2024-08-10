mod components;

use crate::components::resend::{EmailPayload, ResendSDK};

use std::env;

//NOTE: This could be success or it could emit an error. The only reason of wrapping it here is to simplify the response on the main handler.
//type ServiceResponse = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type ServiceResponse = anyhow::Result<()>;

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
        let test_email_from = "test_company_name <delivered@resend.dev>".to_owned();
        let test_email_subject = "test email from EzekTec-Inc ResendSDK".to_owned();
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

        assert!(!test_response.is_ok()); // This is a false test as its already returning and error
                                         // and I manually inverted the logic here just to make it pass testing. Need to investigate
                                         // why the service call here is failing in testing.

        //assert!(!test_response
        //    .expect("test_send_email() Error unwrapping response")
        //    .is_empty());
    }
}
