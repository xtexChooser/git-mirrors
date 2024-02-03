use std::collections::{HashMap, VecDeque};

use crate::{issue, linter::checker::prelude::*};

issue!(IncompleteInterlang, "incomplete_interlang");
issue!(ConflictInterlang, "conflict_interlang");
issue!(BrokenInterlang, "broken_interlang");
checker!(IncompleteInterlangLink, "incomplete_interlang");
config!(LINTER_INTERLANG_FILTER, list str);

impl IssueTrait for IncompleteInterlangIssue {}
#[derive(
	Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct IncompleteInterlangIssueDetails {
	pub lang: String,
	pub title: String,
	pub path: Vec<(String, String)>,
}

impl IssueTrait for ConflictInterlangIssue {}
#[derive(
	Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ConflictInterlangIssueDetails {
	pub lang: String,
	pub page1: String,
	pub path1: Vec<(String, String)>,
	pub page2: String,
	pub path2: Vec<(String, String)>,
}

impl IssueTrait for BrokenInterlangIssue {}
#[derive(
	Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct BrokenInterlangIssueDetails {
	pub lang: String,
	pub title: String,
}

#[derive(Debug, Clone, Default)]
pub struct InterlangLinksGraph {
	pub selflang: String,
	/// Type: <language, (page, from_lang, out_edges<lang, page>)>
	pub links: HashMap<String, (String, String, HashMap<String, String>)>,
	/// Type: (language, (first_page, from_lang), (second_page, from_lang))
	#[allow(clippy::type_complexity)]
	pub conflict: Option<(String, (String, String), (String, String))>,
	/// Type: (language, title, from_lang)
	pub broken: Vec<(String, String, String)>,
}

#[async_trait]
impl ComputedResource for InterlangLinksGraph {
	async fn compute(ctx: Arc<CheckContext>) -> Result<Self> {
		let mut graph: InterlangLinksGraph = Self {
			selflang: ctx.lang.to_owned(),
			..Default::default()
		};
		let mut unresolved_pages = VecDeque::from([(
			ctx.lang.to_owned(),
			ctx.page.to_owned(),
			ctx.lang.to_owned(),
		)]);
		while let Some((lang, page, from_lang)) = unresolved_pages.pop_front() {
			// check conflict
			if let Some((prev_page, prev_page_from, _)) = graph.links.get(&lang)
			{
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
			if graph.broken.iter().any(|(brokenlang, brokentitle, _)| {
				brokenlang == &lang && brokentitle == page.title()
			}) {
				// skip known broken links
				continue;
			}

			// resolve language links
			let langlinks = page.language_links().await?;
			if let Some(langlinks) = langlinks {
				let mut links = HashMap::new();
				for (linklang, linktitle) in langlinks {
					// skip filtered languages
					if !CONFIG_LINTER_INTERLANG_FILTER
						.contains(&linklang.as_str())
					{
						continue;
					}
					if graph.links.get(&linklang).is_none()
						&& linklang != ctx.lang
					{
						// add to resolve queue
						let mut linkpage =
							ctx.app.mwbot(&linklang).await?.page(&linktitle)?;
						if let Some(page) = linkpage.redirect_target().await? {
							linkpage = page;
						}
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

impl InterlangLinksGraph {
	pub fn find_path(&self, lang: &str) -> Result<Vec<(String, String)>> {
		let mut path = Vec::from([self.links[lang].1.to_owned()]);
		loop {
			let lang = path.last().unwrap();
			if lang == &self.selflang {
				break;
			}
			path.push(self.links[lang].1.to_owned());
		}
		path.reverse();
		path.push(lang.to_string());
		Ok(path
			.into_iter()
			.map(|lang| {
				let page = self.links[&lang].0.to_owned();
				(lang, page)
			})
			.collect())
	}
}

#[async_trait]
impl CheckerTrait for IncompleteInterlangLinkChecker {
	fn possible_issues(&self) -> Vec<IssueType> {
		issues![IncompleteInterlang, ConflictInterlang, BrokenInterlang]
	}

	async fn check(&self, ctx: Arc<CheckContext>) -> CheckResult {
		if ctx.page.is_redirect().await? {
			return Ok(());
		}
		let graph = ctx.compute_resource::<InterlangLinksGraph>().await?;
		if let Some((lang, (page1, from1), (page2, from2))) =
			graph.conflict.to_owned()
		{
			ctx.found::<ConflictInterlangIssue, _>(
				ConflictInterlangIssueDetails {
					lang,
					page1,
					path1: graph.find_path(&from1)?,
					page2,
					path2: graph.find_path(&from2)?,
				},
			)?;
		} else {
			let selflinks = &graph.links[&graph.selflang].2;
			for (lang, (page, from_lang, _)) in &graph.links {
				if !selflinks.contains_key(lang) && lang != &graph.selflang {
					ctx.found::<IncompleteInterlangIssue, _>(
						IncompleteInterlangIssueDetails {
							lang: lang.to_owned(),
							title: page.to_owned(),
							path: graph.find_path(from_lang)?,
						},
					)?;
				}
			}
			for (lang, page, from_lang) in &graph.broken {
				if from_lang == &ctx.lang {
					ctx.found::<BrokenInterlangIssue, _>(
						BrokenInterlangIssueDetails {
							lang: lang.to_owned(),
							title: page.to_owned(),
						},
					)?;
				}
			}
		}
		Ok(())
	}
}
