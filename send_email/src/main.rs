use ez_resend_lib::{EmailPayload, ResendSDK, ResendSDKInterface};
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
