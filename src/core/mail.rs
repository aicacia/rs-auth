use anyhow::Result;
use chrono::Datelike;
use futures::join;
use lettre::{
  message::header::ContentType, transport::smtp::authentication::Credentials, Message,
  SmtpTransport, Transport,
};
use sqlx::{Pool, Postgres};

use crate::service::application::{get_application_config, get_application_uri};

pub fn send_support_mail(
  pool: Pool<Postgres>,
  application_id: i32,
  username: String,
  email: String,
  subject: String,
  body: String,
) {
  _ = tokio::spawn(async move {
    match send_mail(
      pool,
      application_id,
      "mail.support.email".to_owned(),
      "mail.support.name".to_owned(),
      email,
      username,
      subject,
      body,
    )
    .await
    {
      Ok(_) => {}
      Err(e) => {
        log::error!("Failed to send support mail: {}", e);
      }
    }
  })
}

async fn send_mail(
  pool: Pool<Postgres>,
  application_id: i32,
  from_email_key: String,
  from_name_key: String,
  to_email: String,
  to_name: String,
  subject: String,
  body: String,
) -> Result<()> {
  let (from_email, from_name, uri, support_email) = join!(
    async {
      get_application_config(&pool, application_id, &from_email_key)
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    },
    async {
      get_application_config(&pool, application_id, &from_name_key)
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    },
    get_application_uri(&pool, application_id),
    async {
      get_application_config(&pool, application_id, "mail.support.email")
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    }
  );

  let msg = Message::builder()
    .from(format!("{} <{}>", from_name, from_email).parse()?)
    .to(format!("{} <{}>", to_name, to_email).parse()?)
    .subject(subject)
    .header(ContentType::TEXT_HTML)
    .body(mail_html(&body, &support_email, &uri))?;

  let mailer = create_mailer(&pool, application_id).await?;
  mailer.send(&msg)?;

  Ok(())
}

async fn create_mailer(pool: &Pool<Postgres>, application_id: i32) -> Result<SmtpTransport> {
  let relay = get_application_config(pool, application_id, "mail.relay")
    .await
    .as_str()
    .unwrap_or_default()
    .to_owned();
  if relay.is_empty() {
    Ok(SmtpTransport::unencrypted_localhost())
  } else {
    let (username, password) = join!(
      async {
        get_application_config(pool, application_id, "mail.username")
          .await
          .as_str()
          .unwrap_or_default()
          .to_owned()
      },
      async {
        get_application_config(pool, application_id, "mail.password")
          .await
          .as_str()
          .unwrap_or_default()
          .to_owned()
      }
    );
    let creds = Credentials::new(username, password);
    let mailer = SmtpTransport::relay(&relay)?.credentials(creds).build();
    Ok(mailer)
  }
}

fn mail_html(html: &str, support_email: &str, uri: &str) -> String {
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
    <div class="content">{0}</div>
    <div class="footer">
      <p>If you have any questions, contact us at <a href="mailto:{1}">{1}</a>.</p>
      <p>&copy; {3} <a target="_blank" href="{2}">{2}</a>. All rights reserved.</p>
    </div>
  </div>
</body>
</html>"#,
    html,
    support_email,
    uri,
    chrono::Utc::now().year()
  )
}
