use anyhow::Result;
use itertools::Itertools;
use mwbot::{
	generators::{
		allpages::AllPages, revisions::Revisions, FilterRedirect, Generator,
	},
	parsoid::WikinodeIterator,
	Bot, Page, SaveOptions,
};
use tracing::{info, warn};

use crate::consts::categorize_redir::*;

pub async fn categorize_redirects() -> Result<()> {
	let bot = Bot::from_default_config().await?;
	let opts = SaveOptions::summary(SUMMARY).mark_as_minor(true);
	let opts_uncat =
		SaveOptions::summary(SUMMARY_UNCATEGORIZED).mark_as_minor(true);
	let ns_template = bot.namespace_id("Template").unwrap();
	let ns_project = bot.namespace_id("Project").unwrap();

	let mut allredirects = AllPages::new()
		.filter_redirect(FilterRedirect::Redirects)
		.generate(&bot);
	while let Some(page) = allredirects.recv().await {
		let mut page: Page = page?;
		let html = page.html().await?.into_mutable();
		let categories = html
			.filter_categories()
			.into_iter()
			.map(|cat| cat.category())
			.collect_vec();
		let redirect = html.redirect().unwrap();
		if redirect.is_external() {
			warn!(%page, "Skipping external redirect");
		}
		let redirect_target = redirect.target();
		let redirect = bot.page(&redirect_target)?;

		let mut new_templates = Vec::new();

		// check R from moved
		if !categories.contains(&CATEGORY_MOVED.to_owned()) {
			let mut count = 0;
			let prefix = format!("[[{}]] moved to [[", page.title());
			let mut revs =
				Revisions::new(vec![page.title().to_string()]).generate(&bot);
			while let Some(rev) = revs.recv().await {
				let (_, rev) = rev?;
				let comment: String = rev.comment.unwrap_or_default();
				if comment.starts_with(&prefix) {
					new_templates.push(TEMPLATE_MOVED);
					break;
				}

				count += 1;
				if count > 5 {
					break;
				}
			}
		}

		// check template alias
		if page.namespace() == ns_template
			&& redirect.namespace() == ns_template
			&& !categories.contains(&CATEGORY_TEMPLATE_ALIAS.to_owned())
		{
			new_templates.push(TEMPLATE_TEMPLATE_ALIAS);
		}

		// check R to section
		if redirect_target.contains('#')
			&& !categories.contains(&CATEGORY_SECTION.to_owned())
		{
			new_templates.push(TEMPLATE_SECTION);
		}

		// check R to project page
		if redirect.namespace() == ns_project
			&& !categories.contains(&CATEGORY_PROJECT.to_owned())
		{
			new_templates.push(TEMPLATE_PROJECT);
		}

		// save changes
		let mut redirect_template = html
			.filter_templates()?
			.into_iter()
			.find(|t| t.name() == REDIRECT_PAGE_TEMPLATE);
		if redirect_template.is_none() {
			// add a stub template
			let mut wt = page.wikitext().await?;
			wt = format!("{}\n\n{}", wt.trim_end(), STUB);
			let (page1, resp) = page.save(wt, &opts_uncat).await?;
			page = page1;
			info!(
				%page,
				oldrev = resp.oldrevid,
				revid = resp.newrevid,
				"Added a stub to redirect"
			);
			redirect_template = page
				.html()
				.await?
				.into_mutable()
				.filter_templates()?
				.into_iter()
				.find(|t| t.name() == REDIRECT_PAGE_TEMPLATE);
		}
		if !new_templates.is_empty() {
			// add templates
			let redirect_template = redirect_template.unwrap();
			let param = format!(
				"\n{}\n",
				format!(
					"{}\n{}",
					redirect_template.param("1").unwrap_or_default().trim(),
					new_templates
						.iter()
						.map(|s| format!("{{{{{}}}}}", s))
						.join("\n")
				)
				.trim()
			);
			redirect_template.set_param("1", &param)?;
			let (page1, resp) = page.save(html, &opts).await?;
			page = page1;
			info!(%page, ?new_templates, oldrev=resp.oldrevid, revid=resp.newrevid, "Categorized a redirect");
		}
	}

	Ok(())
}
