use std::collections::{HashMap, VecDeque};

use crate::{
	issue, issues,
	linter::checker::{prelude::*, ComputedResource},
};

checker!(IncompleteInterlangLinkChecker, "incomplete_interlang");

issue!(IncompleteInterlangLink, "incomplete_interlang");
impl IssueTrait for IncompleteInterlangLinkIssue {}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncompleteInterlangLinkDetails {
	pub language: String,
	pub title: String,
	pub path: Vec<(String, String)>,
}

issue!(ConflictInterlang, "conflict_interlang");
impl IssueTrait for ConflictInterlangIssue {}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConflictInterlangIssueDetails {
	pub language: String,
	pub first_page: String,
	pub first_path: Vec<(String, String)>,
	pub second_page: String,
	pub second_path: Vec<(String, String)>,
}

issue!(BrokenInterlang, "conflict_interlang");
impl IssueTrait for BrokenInterlangIssue {}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BrokenInterlangIssueDetails {
	pub language: String,
	pub title: String,
	pub path: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct InterlangLinksGraph {
	/// Type: <language, (page, from_lang, out_edges<lang, page>)>
	pub links: HashMap<String, (String, String, HashMap<String, String>)>,
	/// Type: (language, (first_page, from_lang), (second_page, from_lang))
	pub conflict: Option<(String, (String, String), (String, String))>,
	/// Type: (language, title, from_lang)
	pub broken: Vec<(String, String, String)>,
}

#[async_trait]
impl ComputedResource for InterlangLinksGraph {
	async fn compute(ctx: Arc<CheckContext>) -> Result<Self> {
		let mut graph: InterlangLinksGraph = Self::default();
		let mut unresolved_pages = VecDeque::from([(
			ctx.lang.to_owned(),
			ctx.page.to_owned(),
			ctx.lang.to_owned(),
		)]);
		while let Some((lang, page, from_lang)) = unresolved_pages.pop_front() {
			// check conflict
			if let Some((prev_page, prev_page_from, _)) = graph.links.get(&lang) {
				if prev_page != page.title() {
					graph.conflict = Some((
						lang,
						(prev_page.to_owned(), prev_page_from.to_owned()),
						(page.title().to_string(), from_lang),
					));
					break;
				} else {
					// skip resolved page
					continue;
				}
			}
			if graph
				.broken
				.iter()
				.find(|(brokenlang, brokentitle, _)| {
					brokenlang == &lang && brokentitle == page.title()
				})
				.is_some()
			{
				// skip known broken links
				continue;
			}

			// resolve language links
			let langlinks = page.language_links().await?;
			if let Some(langlinks) = langlinks {
				let mut links = HashMap::new();
				for (linklang, linktitle) in langlinks {
					let linkpage = ctx.app.mwbot(&linklang).await?.page(&linktitle)?;
					if graph.links.get(&linklang).is_none() {
						// add to resolve queue
						unresolved_pages.push_back((
							linklang.clone(),
							linkpage.clone(),
							lang.clone(),
						));
					}
					links.insert(linklang, linktitle);
				}
				graph
					.links
					.insert(lang, (page.title().to_string(), from_lang, links));
			} else {
				// page not found
				graph
					.broken
					.push((lang, page.title().to_string(), from_lang));
			}
		}
		Ok(graph)
	}
}

#[async_trait]
impl CheckerTrait for IncompleteInterlangLinkChecker {
	fn possible_issues(&self) -> Vec<IssueType> {
		issues![IncompleteInterlangLink, ConflictInterlang, BrokenInterlang]
	}

	async fn check(&self, ctx: Arc<CheckContext>) -> CheckResult {
		// ctx.found::<ConflictInterlangIssue, _>(json!({}))?;
		let _graph = ctx.compute_resource::<InterlangLinksGraph>().await?;
		Ok(())
	}
}
