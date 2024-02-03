use std::ops::Deref;

use crate::{issue, linter::checker::prelude::*};

issue!(DoubleRedirect, "double_redirect");
checker!(DoubleRedirect, "double_redirect");

impl IssueTrait for DoubleRedirectIssue {}
#[derive(
	Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct DoubleRedirectIssueDetails {
	pub path: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RedirectPath(pub Vec<String>);

impl Deref for RedirectPath {
	type Target = Vec<String>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[async_trait]
impl ComputedResource for RedirectPath {
	async fn compute(ctx: Arc<CheckContext>) -> Result<Self> {
		if let Some(page) = ctx.page.redirect_target().await? {
			let mut path = vec![page];
			while let Some(page) =
				path.last().unwrap().redirect_target().await?
			{
				path.push(page);
			}
			Ok(Self(
				path.into_iter().map(|p| p.title().to_string()).collect(),
			))
		} else {
			Ok(Self(Vec::new()))
		}
	}
}

#[async_trait]
impl CheckerTrait for DoubleRedirectChecker {
	fn possible_issues(&self) -> Vec<IssueType> {
		issues![DoubleRedirect]
	}

	async fn check(&self, ctx: Arc<CheckContext>) -> CheckResult {
		if !ctx.page.is_redirect().await? {
			return Ok(());
		}
		let path = ctx.compute_resource::<RedirectPath>().await?;
		if path.len() >= 2 {
			ctx.found::<DoubleRedirectIssue, _>(DoubleRedirectIssueDetails {
				path: path.0.to_owned(),
			})?;
		}
		Ok(())
	}
}
