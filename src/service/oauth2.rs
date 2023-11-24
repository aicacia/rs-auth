use anyhow::Result;
use futures::join;
use oauth2::{
  basic::BasicClient, url::Url, AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
  TokenUrl,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::application::get_application_config;

async fn create_authorization(
  pool: &Pool<Postgres>,
  application_id: Uuid,
  client_uri: &str,
  scopes: &[String],
  redirect_url: &str,
) -> Result<(Url, CsrfToken)> {
  let client = create_client(pool, application_id, client_uri, redirect_url).await?;
  let mut authorization_request = client.authorize_url(CsrfToken::new_random);
  for scope in scopes {
    authorization_request = authorization_request.add_scope(Scope::new(scope.to_owned()));
  }
  Ok(authorization_request.url())
}

async fn create_client(
  pool: &Pool<Postgres>,
  application_id: Uuid,
  client_uri: &str,
  redirect_url: &str,
) -> Result<
  oauth2::Client<
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::basic::BasicTokenType,
    oauth2::StandardTokenIntrospectionResponse<
      oauth2::EmptyExtraTokenFields,
      oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    oauth2::StandardErrorResponse<oauth2::RevocationErrorResponseType>,
  >,
> {
  let client_id_key = format!("oauth2.{}.client_id", client_uri);
  let client_secret_key = format!("oauth2.{}.client_secret", client_uri);
  let client_auth_url_key = format!("oauth2.{}.auth_url", client_uri);
  let client_token_url_key = format!("oauth2.{}.token_url", client_uri);

  let (client_id_str, client_secret_str, client_auth_url_str, client_token_url_str) = join!(
    async {
      get_application_config(pool, application_id, &client_id_key)
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    },
    async {
      get_application_config(pool, application_id, &client_secret_key)
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    },
    async {
      get_application_config(pool, application_id, &client_auth_url_key)
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    },
    async {
      get_application_config(pool, application_id, &client_token_url_key)
        .await
        .as_str()
        .unwrap_or_default()
        .to_owned()
    },
  );

  let client_id = ClientId::new(client_id_str);
  let client_secret = ClientSecret::new(client_secret_str);
  let auth_url = AuthUrl::new(client_auth_url_str)?;
  let token_url = TokenUrl::new(client_token_url_str)?;

  let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_owned())?);

  Ok(client)
}
