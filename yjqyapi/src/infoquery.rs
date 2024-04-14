use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

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
pub struct InfoQueryColumn {
    pub id: String,
    pub text: String,
}

impl fmt::Display for InfoQueryColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} ({})", self.id, self.text))
    }
}

impl TryFrom<NodeDataRef<ElementData>> for InfoQueryColumn {
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

/// 信息查询结果
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoQueryResult {
    /// 查询结果标题，格式为“符合条件信息(1)”，每次查询结果从1开始编号
    pub index: String,
    /// 查询结果
    pub values: HashMap<String, String>,
}

impl PartialOrd for InfoQueryResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for InfoQueryResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl Index<&String> for InfoQueryResult {
    type Output = String;

    fn index(&self, index: &String) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<&String> for InfoQueryResult {
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        self.values.get_mut(index).unwrap()
    }
}

impl IntoIterator for InfoQueryResult {
    type Item = (String, String);
    type IntoIter = <HashMap<String, String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
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
    ) -> Result<Vec<InfoQueryColumn>>;

    /// 进行信息查询
    async fn do_info_query<'a, I, V>(
        &self,
        school: &School,
        query: &str,
        input: I,
    ) -> Result<Vec<InfoQueryResult>>
    where
        I: IntoIterator + Send,
        I::Item: Borrow<(&'a InfoQueryColumn, V)>,
        V: AsRef<str>;
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
    ) -> Result<Vec<InfoQueryColumn>> {
        let page = self
            .post_page_html(
                format!("/sc/{}/stu_chaxun.php", school.0),
                [("xmid", id)],
            )
            .await?;

        let mut columns = Vec::new();
        for el in page
            .select("table tr:nth-child(2) table tbody tr:nth-child(3) select")
            .unwrap()
        {
            columns.push(InfoQueryColumn::try_from(el)?);
        }

        debug!(?columns, "Fetched info query input columns");
        Ok(columns)
    }

    async fn do_info_query<'a, I, V>(
        &self,
        school: &School,
        query: &str,
        input: I,
    ) -> Result<Vec<InfoQueryResult>>
    where
        I: IntoIterator + Send,
        I::Item: Borrow<(&'a InfoQueryColumn, V)>,
        V: AsRef<str>,
    {
        let mut params = HashMap::from([
            ("xmid".to_owned(), query.to_owned()),
            ("chaxun".to_owned(), "查询".to_owned()),
            ("guanxi".to_owned(), "1".to_owned()),
        ]);
        for input in input.into_iter() {
            let (col, val) = input.borrow();
            params.insert(col.id.to_owned(), col.text.to_owned());
            params.insert(format!("{}_inf", &col.id), val.as_ref().to_owned());
        }
        let page = self
            .post_page_html(format!("/sc/{}/stu_chaxun.php", school.0), params)
            .await?;
        let trs = page
            .select("body>table>tbody>tr:nth-child(3)>td>table>tbody>tr")
            .unwrap()
            .collect::<Vec<_>>();
        let mut results = Vec::new();

        if trs.is_empty() {
            return Err(Error::MalformedHTML(
                "No result rows found in info query response",
                None,
            ));
        }

        let first_tds =
            trs[0].as_node().select("td").unwrap().collect::<Vec<_>>();
        if first_tds.len() == 2
            && first_tds[0].text_contents().contains("错误信息：")
        {
            let message = first_tds[1].text_contents().trim().to_owned();
            if message.contains("找不到匹配的信息") {
                return Ok(results);
            }
            return Err(Error::RemoteError(message));
        }

        let mut current = None;
        for tr in trs {
            let tds = tr.as_node().select("td").unwrap().collect::<Vec<_>>();
            match tds.len() {
                1 => {
                    let td = tds[0].text_contents().trim().to_owned();
                    if td.is_empty() {
                        // do push here instead of at title processing, so the last record will be pushed
                        if let Some(current) = current.take() {
                            results.push(current);
                        }
                    } else {
                        if current.is_some() {
                            return Err(Error::MalformedHTML(
                                "Two title row without a space delimiter",
                                None,
                            ));
                        }
                        current = Some(InfoQueryResult {
                            index: td,
                            values: HashMap::new(),
                        });
                    }
                }
                2 => {
                    let key = tds[0].text_contents().trim().to_owned();
                    let value = tds[1].text_contents().trim().to_owned();
                    let current = current.as_mut().ok_or_else(|| Error::MalformedHTML(
                        "A result row found in info query response is placed before a result title",
                        Some(format!("key: {}, value: {}", key, value))
                    ))?;
                    if key.is_empty() {
                        // filter empty keys
                        continue;
                    }
                    current.values.insert(key, value);
                }
                _ => {
                    return Err(Error::MalformedHTML(
                        "A result row found in info query response has unexpected count of tds",
                        Some(tr.text_contents()),
                    ));
                }
            }
        }

        Ok(results)
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

    #[tokio::test]
    async fn do_info_query() -> Result<()> {
        let c = QyClient::new(Default::default()).await?;
        let sc = "yjyz".into();
        let s = c.info_queries(&sc).await?[0].to_owned();
        println!("{:?}", s);
        let cols = c.info_query_input_columns(&sc, &s.id).await?;
        println!("{:#?}", cols);
        let r = c.do_info_query(&sc, &s.id, [(&cols[0], "XXX")]).await?;
        println!("{:#?}", r);
        assert!(r.is_empty());
        Ok(())
    }
}
