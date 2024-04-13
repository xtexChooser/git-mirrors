use async_trait::async_trait;
use kuchikiki::{ElementData, NodeDataRef};
use serde::{Deserialize, Serialize};

use crate::*;

/// 信息查询
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct InfoQuery {
    pub id: String,
    pub text: String,
}

impl TryFrom<NodeDataRef<ElementData>> for InfoQuery {
    type Error = Error;

    fn try_from(value: NodeDataRef<ElementData>) -> Result<Self> {
        let id = value
            .attributes
            .borrow()
            .get("value")
            .ok_or_else(|| {
                Error::MalformedHTML("value attr not available", None)
            })?
            .to_owned();
        let text = value.text_contents().trim().to_owned();
        Ok(Self { id, text })
    }
}

impl fmt::Display for InfoQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} ({})", self.id, self.text))
    }
}

/// 信息查询所需要的列
#[derive(
    Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct InfoQueryInputColumn {
    pub id: String,
    pub text: String,
}

impl fmt::Display for InfoQueryInputColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} ({})", self.id, self.text))
    }
}

impl TryFrom<NodeDataRef<ElementData>> for InfoQueryInputColumn {
    type Error = Error;

    fn try_from(value: NodeDataRef<ElementData>) -> Result<Self> {
        let id = value
            .attributes
            .borrow()
            .get("id")
            .ok_or_else(|| Error::MalformedHTML("id attr not available", None))?
            .to_owned();
        let text = value.text_contents().trim().to_owned();
        Ok(Self { id, text })
    }
}

/// 信息查询
///
/// 参考http://qy.yjzqy.net:9090/sc/yjyz/stu_chaxun.php
#[async_trait]
pub trait InfoQueryAccess {
    /// 获取所有信息查询
    async fn info_queries(&self, school: &School) -> Result<Vec<InfoQuery>>;

    /// 获取指定信息查询需要的输入参数
    async fn info_query_input_columns(
        &self,
        school: &School,
        query: &str,
    ) -> Result<Vec<InfoQueryInputColumn>>;
}

#[async_trait]
impl InfoQueryAccess for QyClient {
    async fn info_queries(&self, school: &School) -> Result<Vec<InfoQuery>> {
        let page = self
            .get_page_html(format!("/sc/{}/stu_chaxun.php", school.0))
            .await?;

        let mut infoquery = Vec::new();
        for opt in page.select("table select[id=\"xmid\"] option").unwrap() {
            infoquery.push(InfoQuery::try_from(opt)?);
        }

        debug!(?infoquery, "Fetched info queries");
        Ok(infoquery)
    }

    async fn info_query_input_columns(
        &self,
        school: &School,
        id: &str,
    ) -> Result<Vec<InfoQueryInputColumn>> {
        let page = self
            .post_page_html(
                format!("/sc/{}/stu_chaxun.php", school.0),
                &[("xmid", id)],
            )
            .await?;

        let mut columns = Vec::new();
        for el in page
            .select("table tr:nth-child(2) table tbody tr:nth-child(3) select")
            .unwrap()
        {
            columns.push(InfoQueryInputColumn::try_from(el)?);
        }

        debug!(?columns, "Fetched info query input columns");
        Ok(columns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn info_queries() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        let s = c.info_queries(&"yjyz".into()).await?;
        debug!(?s);
        println!("{:#?}", s);
        Ok(())
    }

    #[tokio::test]
    async fn info_query_input_columns() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        let sc = "yjyz".into();
        let s = c.info_queries(&sc).await?[0].to_owned();
        println!("{:?}", s);
        let cols = c.info_query_input_columns(&sc, &s.id).await?;
        println!("{:#?}", cols);
        Ok(())
    }
}
