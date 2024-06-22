use anyhow::Result;
use mwbot::{
	generators::{allpages::AllPages, FilterRedirect, Generator},
	Bot, Page, SaveOptions,
};
use tracing::info;

use crate::consts::uncatredir::*;

pub async fn mark_uncategorized_redirects() -> Result<()> {
	let bot = Bot::from_default_config().await?;
	let opts =
		SaveOptions::summary(SUMMARY).mark_as_minor(true);

	let mut allpages = AllPages::new()
		.filter_redirect(FilterRedirect::Redirects)
		.generate(&bot);
	while let Some(page) = allpages.recv().await {
		let page: Page = page?;
		if !page
			.templates(Some(vec![REDIRECT_PAGE_TEMPLATE.to_string()]))
			.await?
			.unwrap()
			.is_empty()
		{
			continue;
		}
		let mut wt = page.wikitext().await?;
		wt = format!("{}\n\n{}", wt.trim_end(), STUB);
		let (page, resp) = page.save(wt, &opts).await?;
		let oldrev = resp.oldrevid;
		let revid = resp.newrevid;
		info!(%page, oldrev, revid, "Added a stub {{{{Redirect page}}}} to redirect");
	}

	Ok(())
}
