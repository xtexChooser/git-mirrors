use anyhow::{anyhow, Result};
use mwbot::{
	generators::{
		categories::CategoryMembers, revisions::Revisions, Generator,
	},
	parsoid::WikinodeIterator,
	Bot, Page, SaveOptions,
};
use tracing::info;

use crate::consts::categorize_redir::*;

pub async fn categorize_redirects() -> Result<()> {
	let bot = Bot::from_default_config().await?;
	let opts_moved = SaveOptions::summary(SUMMARY_MOVED).mark_as_minor(true);

	let mut members =
		CategoryMembers::new(UNCATEGORIZED_CATEGORIES).generate(&bot);
	while let Some(page) = members.recv().await {
		let page: Page = page?;

		let mut result = None;

		// check R from moved
		if result.is_none() {
			let mut count = 0;
			let prefix = format!("[[{}]] moved to [[", page.title());
			let mut revs =
				Revisions::new(vec![page.title().to_string()]).generate(&bot);
			while let Some(rev) = revs.recv().await {
				let (_, rev) = rev?;
				let comment: String = rev.comment.unwrap_or_default();
				if comment.starts_with(&prefix) {
					result = Some((TEMPLATE_MOVED, &opts_moved));
					break;
				}

				count += 1;
				if count > 5 {
					break;
				}
			}
		}

		if let Some((template, opts)) = result {
			let html = page.html().await?.into_mutable();
			let tpl = html.filter_templates()?.into_iter().find(|tpl| {
				tpl.name().eq_ignore_ascii_case(REDIRECT_PAGE_TEMPLATE)
			}).ok_or_else(||anyhow!("{{{{Redirect page}}}} template not found in [[{}]] which is in [[:{}]]", page.title(), UNCATEGORIZED_CATEGORIES))?;
			let param = format!(
				"\n{}\n",
				format!(
					"{}\n{{{{{}}}}}",
					tpl.param("1").unwrap_or_default().trim(),
					template
				)
				.trim()
			);
			tpl.set_param("1", &param)?;

			let (page, resp) = page.save(html, opts).await?;
			let oldrev = resp.oldrevid;
			let revid = resp.newrevid;
			info!(%page, template, oldrev, revid, "Categorized a redirect");
		}
	}

	Ok(())
}
