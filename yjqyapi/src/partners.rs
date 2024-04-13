use std::collections::HashMap;

use async_trait::async_trait;

use crate::*;

#[async_trait]
pub trait PartnerSchoolsAccess {
    /// 获取合作伙伴学校（所有使用启业网的学校）
    async fn partner_schools(
        &self,
    ) -> Result<HashMap<SchoolIdentifier, String>>;
}

#[async_trait]
impl PartnerSchoolsAccess for QyClient {
    async fn partner_schools(
        &self,
    ) -> Result<HashMap<SchoolIdentifier, String>> {
        let page = self.get_page_html("/list/link_qy.php").await?;
        let mut partners = HashMap::new();

        for a in page.select("table td a").unwrap() {
            let name = a.text_contents().trim().to_owned();
            let id = a
                .attributes
                .borrow()
                .get("href")
                .ok_or_else(|| {
                    Error::MalformedHTML(
                        "href attr not exist on a elements on link_qy.php",
                        None,
                    )
                })?
                .to_owned();
            if !id.starts_with("http://qy.yjzqy.net:9090/sc/") {
                return Err(Error::MalformedHTML(
                    "Invalid prefix for school index link on link_qy.php",
                    None,
                ));
            }
            let id = id[28..].trim_end_matches('/').to_string();
            partners.insert(SchoolIdentifier(id), name);
        }

        debug!(?partners, "Resolved partner schools");
        Ok(partners)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn partner_schools() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        let s = c.partner_schools().await?;
        debug!(?s);
        assert_eq!(s[&"yjgj".into()], "阳江高级中学");
        assert_eq!(s[&"yjyz".into()], "阳江市第一中学");
        Ok(())
    }
}
