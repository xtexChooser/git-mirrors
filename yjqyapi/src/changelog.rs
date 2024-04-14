use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::Date;

use crate::*;

/// 软件版本更新说明
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct ChangeLog {
    /// 日期
    pub date: Date,
    /// 版本号
    pub version: String,
    /// 更改内容
    pub text: String,
}

impl TryFrom<&str> for ChangeLog {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let (date, value) = value.split_once('【').ok_or_else(|| {
            Error::MalformedHTML("changelog text has no '【'", None)
        })?;
        let (version, value) = value.split_once('】').ok_or_else(|| {
            Error::MalformedHTML("changelog text has no '】'", None)
        })?;
        let text = value.trim();

        if date.len() != 8 {
            return Err(Error::MalformedHTML(
                "Date in changelog must has 8 characters",
                None,
            ));
        }
        let date = Date::from_calendar_date(
            date[0..=3].parse()?,
            TryInto::<time::Month>::try_into(date[4..=5].parse::<u8>()?)?,
            date[6..=7].parse()?,
        )?;
        Ok(Self {
            date,
            version: version.to_owned(),
            text: text.to_owned(),
        })
    }
}

#[async_trait]
pub trait ChangeLogAccess {
    /// 获取软件版本更新说明
    ///
    /// 参考http://qy.yjzqy.net:9090/sc/yjyz/banben.php
    async fn changelog(&self, school: &School) -> Result<Vec<ChangeLog>>;
}

#[async_trait]
impl ChangeLogAccess for QyClient {
    async fn changelog(&self, school: &School) -> Result<Vec<ChangeLog>> {
        let page = self
            .get_page_html(format!("/sc/{}/banben.php", school.0))
            .await?;

        let mut changelog = Vec::new();
        for td in page.select("table tr:not(:first-child) td").unwrap() {
            let td = td.text_contents();
            changelog.push(ChangeLog::try_from(td.as_str())?);
        }

        debug!(?changelog, "Fetched changelog");
        Ok(changelog)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn changelog() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        let s = c.changelog(&"yjyz".into()).await?;
        debug!(?s);
        println!("{:#?}", s);
        Ok(())
    }
}
