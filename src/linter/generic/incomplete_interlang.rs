use serde_json::json;

use crate::{issues, linter::checker::prelude::*};

checker!(IncompleteInterlangLinkChecker, "incomplete_interlang");

#[async_trait]
impl CheckerTrait for IncompleteInterlangLinkChecker {
	fn possible_issues(&self) -> Vec<Issue> {
		issues![]
	}
	async fn check(&self, ctx: &mut CheckContext) -> CheckResult {
		ctx.issue(self, json!({}))?;
		Ok(())
	}
}
