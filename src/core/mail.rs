use anyhow::Result;
use chrono::Datelike;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport};
use sqlx::{Pool, Postgres};

use crate::service::application::get_application_config;

pub async fn create_mailer(pool: &Pool<Postgres>, application_id: i32) -> Result<SmtpTransport> {
  let relay = get_application_config(pool, application_id, "mail.relay")
    .await
    .as_str()
    .unwrap_or_default()
    .to_owned();
  if relay.is_empty() {
    Ok(SmtpTransport::unencrypted_localhost())
  } else {
    let username = get_application_config(pool, application_id, "mail.username")
      .await
      .as_str()
      .unwrap_or_default()
      .to_owned();
    let password = get_application_config(pool, application_id, "mail.password")
      .await
      .as_str()
      .unwrap_or_default()
      .to_owned();
    let creds = Credentials::new(username, password);
    let mailer = SmtpTransport::relay(&relay)?.credentials(creds).build();
    Ok(mailer)
  }
}

pub fn send_mail<F>(pool: Pool<Postgres>, application_id: i32, msg_builder: F)
where
  F: Fn() -> Result<lettre::Message>,
{
  let msg = match msg_builder() {
    Ok(msg) => msg,
    Err(e) => {
      log::error!("Failed to send email: {}", e);
      return;
    }
  };

  let _ = tokio::spawn(async move {
    match create_mailer(&pool, application_id).await {
      Ok(mailer) => match mailer.send(&msg) {
        Ok(_) => (),
        Err(e) => {
          log::error!("Failed to send email: {}", e);
        }
      },
      Err(e) => {
        log::error!("Failed to build email: {}", e);
      }
    }
  });
}

pub fn mail_html(html: String) -> String {
  format!(
    r#"
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Your Beautiful Email</title>
  <style>
    body, p, h1, h2, h3, h4, h5, h6 {{
      margin: 0;
      padding: 0;
    }}
    body {{
      font-family: Arial, sans-serif;
      background-color: #f7f7f7;
    }}
    .container {{
      max-width: 768px;
      margin: 3rem auto;
      padding: 1rem;
      background-color: #ffffff;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    }}
    .content {{
      color: black;
      font-size: 1rem;
    }}
    code {{
        padding: 0.0.5rem 0.1rem;
        background-color: #ddd;
    }}
    .footer {{
      text-align: center;
      margin-top: 1rem;
      color: grey;
      font-size: 0.75rem;
    }}
  </style>
</head>
<body>
  <div class="container">
    <div class="content">{}</div>
    <div class="footer">
      <p>If you have any questions, contact us at support@aicacia.com.</p>
      <p>&copy; {}. All rights reserved.</p>
    </div>
  </div>
</body>
</html>"#,
    html,
    chrono::Utc::now().year()
  )
}
