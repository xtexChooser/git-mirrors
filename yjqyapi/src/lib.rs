use std::fmt;

use kuchikiki::{traits::TendrilSink, NodeRef};
use serde::Serialize;
use tracing::debug;
pub use url::Url;

pub mod changelog;
pub mod error;
pub mod infoquery;
pub mod partners;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct School(String);

impl<S: Into<String>> From<S> for School {
    fn from(value: S) -> Self {
        School(value.into())
    }
}

impl ToString for School {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl fmt::Debug for School {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Default)]
pub struct QyClientOptions {
    pub http_client: reqwest::ClientBuilder,
    pub user_agent: Option<String>,
    pub base_url: Option<Url>,
}

#[derive(Clone)]
pub struct QyClient {
    pub http_client: reqwest::Client,
    pub base_url: Url,
}

impl QyClient {
    pub async fn new(options: QyClientOptions) -> Result<Self> {
        let http_client =
            options
                .http_client
                .user_agent(options.user_agent.unwrap_or(format!(
                    "yjqyapi/{}",
                    env!("CARGO_PKG_VERSION")
                )))
                .cookie_store(true)
                .build()?;
        let base_url = options.base_url.unwrap_or_else(|| {
            Url::parse("http://qy.yjzqy.net:9090/").unwrap()
        });
        Ok(Self {
            http_client,
            base_url,
        })
    }

    #[must_use]
    pub fn make_url<S: Into<String>>(&self, path: S) -> Result<Url> {
        Ok(self.base_url.join(&path.into().as_str())?)
    }

    #[must_use]
    pub async fn get_page_html<S: Into<String>>(
        &self,
        path: S,
    ) -> Result<NodeRef> {
        Ok(self
            .request_page_html(
                self.http_client.get(self.make_url(path)?).build()?,
            )
            .await?)
    }

    #[must_use]
    pub async fn post_page_html<S: Into<String>, T: Serialize + ?Sized>(
        &self,
        path: S,
        data: &T,
    ) -> Result<NodeRef> {
        Ok(self
            .request_page_html(
                self.http_client
                    .post(self.make_url(path)?)
                    .form::<T>(data)
                    .build()?,
            )
            .await?)
    }

    #[must_use]
    pub async fn request_page_html(
        &self,
        req: reqwest::Request,
    ) -> Result<NodeRef> {
        debug!(?req);
        let resp = self.http_client.execute(req).await?;
        let body = resp.error_for_status()?.text_with_charset("gb2312").await?;
        Ok(self.parse_html(&body)?)
    }

    #[must_use]
    pub fn parse_html(&self, html: &str) -> Result<NodeRef> {
        Ok(kuchikiki::parse_html().one(html))
    }
}

impl fmt::Debug for QyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("QyClient")
            .field("http_client", &self.http_client)
            .field("base_url", &self.base_url)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_client() -> Result<()> {
        let _ = QyClient::new(Default::default()).await?;
        Ok(())
    }

    #[tokio::test]
    async fn make_url() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        assert_eq!(
            c.make_url("/test")?.as_str(),
            "http://qy.yjzqy.net:9090/test"
        );
        assert_eq!(
            c.make_url("/list/link_qy.php")?.as_str(),
            "http://qy.yjzqy.net:9090/list/link_qy.php"
        );
        assert_eq!(
            c.make_url("/sc/yjyz/banben.php")?.as_str(),
            "http://qy.yjzqy.net:9090/sc/yjyz/banben.php"
        );
        Ok(())
    }

    #[tokio::test]
    async fn get_page_html() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        let _ = c.get_page_html("/list/link_qy.php").await?;
        Ok(())
    }
}
