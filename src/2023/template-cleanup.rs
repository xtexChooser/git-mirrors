use anyhow::Result;
use log::info;
use mwbot::{
    generators::{allpages, FilterRedirect},
    SaveOptions,
};
use regex::Regex;
use wiki_bot::utils::{get_bot, init};

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    let bot = get_bot().await?;
    let comment_regex =
        Regex::new("<!--\\s*.*(/doc|categories|文档|跨语言|Document|分类).*\\s*-->")?;
    let noinclude_regex = Regex::new("<\\s*noinclude\\s*>(\\s|\\n|\\t)*<\\s*/noinclude\\s*\\s*>")?;
    let includeonly_regex =
        Regex::new("<\\s*includeonly\\s*>(\\s|\\n|\\t)*<\\s*/includeonly\\s*\\s*>")?;
    let mut pages = allpages(&bot, 10, FilterRedirect::Nonredirects);
    let mut counter = 0;
    while let Some(page_result) = pages.recv().await {
        let page = page_result?;
        info!("processing {} {}", page.title(), page.exists().await?);
        let wt = page.wikitext().await?;
        let wt1 = comment_regex.replace_all(wt.as_str(), "").to_owned();
        let wt2 = noinclude_regex.replace_all(&wt1, "").to_owned();
        let wt3 = includeonly_regex.replace_all(&wt2, "");
        let out = wt3.to_owned().to_string();
        if wt != out {
            info!("changed {}", page.title());
            for diff in diff::lines(wt.as_str(), out.as_str()) {
                match diff {
                    diff::Result::Left(l) => println!("-{}", l),
                    diff::Result::Both(l, _) => println!(" {}", l),
                    diff::Result::Right(r) => println!("+{}", r),
                }
            }
            info!("saving");
            let (_, resp) = page
                .save(out, &SaveOptions::summary("Template comment cleanup"))
                .await?;
            info!("uploaded: {:?}", resp);
            counter += 1;
        }
    }
    info!("formatted {} pages", counter);
    Ok(())
}
