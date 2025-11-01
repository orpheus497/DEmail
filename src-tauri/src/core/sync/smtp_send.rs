use crate::core::accounts::get_refresh_token;
use crate::error::DEmailError;
use lettre::{
    transport::smtp::authentication::Mechanism, AsyncSmtpTransport, AsyncTransport, Message,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, RefreshToken, TokenResponse,
};

pub async fn send_email(
    message: Message,
    provider: &str,
    access_token: &AccessToken,
) -> Result<(), DEmailError> {
    let (smtp_server, smtp_user) = match provider {
        "google" => ("smtp.gmail.com", ""), // User email will be used
        "microsoft" => ("smtp.office365.com", ""),
        _ => {
            return Err(DEmailError::Smtp(lettre::transport::smtp::Error::from(
                "Unsupported provider".to_string(),
            )))
        }
    };

    let mailer = AsyncSmtpTransport::<tokio::io::DuplexStream>::starttls_relay(smtp_server)
        .map_err(|e| DEmailError::Smtp(e))?
        .credentials(lettre::transport::smtp::authentication::Credentials::new(
            smtp_user.to_string(),
            access_token.secret().to_string(),
        ))
        .authentication(vec![Mechanism::XOAuth2])
        .build();

    mailer
        .send(message)
        .await
        .map_err(|e| DEmailError::Smtp(e))?;

    Ok(())
}
