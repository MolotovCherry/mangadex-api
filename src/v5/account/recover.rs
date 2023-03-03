//! Builder for initiating the account recovery process.
//!
//! <https://api.mangadex.org/swagger.html#/Account/post-account-recover>
//!
//! # Examples
//!
//! ```rust
//! use mangadex_api::v5::MangaDexClient;
//!
//! # async fn run() -> anyhow::Result<()> {
//! let client = MangaDexClient::default();
//!
//! let account_recover_res = client
//!     .account()
//!     .recover()
//!     .email("test@example.com")
//!     .build()?
//!     .send()
//!     .await?;
//!
//! println!("account recovery: {:?}", account_recover_res);
//! # Ok(())
//! # }
//! ```

use derive_builder::Builder;
use serde::Serialize;

use crate::HttpClientRef;
use mangadex_api_schema::NoData;
use mangadex_api_types::error::Result;

/// Recover an account.
///
/// Makes a request to `POST /account/recover`.
#[derive(Debug, Builder, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[builder(setter(into, strip_option))]
pub struct RecoverAccount<'a> {
    /// This should never be set manually as this is only for internal use.
    #[doc(hidden)]
    #[serde(skip)]
    #[builder(pattern = "immutable")]
    pub(crate) http_client: HttpClientRef,

    pub email: &'a str,
}

endpoint! {
    POST "/account/recover",
    #[body] RecoverAccount<'_>,
    #[discard_result] Result<NoData>
}

#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use serde_json::json;
    use url::Url;
    use wiremock::matchers::{body_json, header, method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{HttpClient, MangaDexClient};

    #[tokio::test]
    async fn recover_fires_a_request_to_base_url() -> anyhow::Result<()> {
        let mock_server = MockServer::start().await;
        let http_client = HttpClient::builder()
            .base_url(Url::parse(&mock_server.uri())?)
            .build()?;
        let mangadex_client = MangaDexClient::new_with_http_client(http_client);

        let email: String = SafeEmail().fake();

        Mock::given(method("POST"))
            .and(path_regex(r"/account/recover"))
            .and(header("Content-Type", "application/json"))
            .and(body_json(json!({ "email": email })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"result": "ok"})))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = mangadex_client
            .account()
            .recover()
            .email(email.as_str())
            .build()?
            .send()
            .await?;

        Ok(())
    }
}
