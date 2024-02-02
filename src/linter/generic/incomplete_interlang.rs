use serde_json::json;

use crate::{issue, issues, linter::checker::{prelude::*, ComputedResource}};

checker!(IncompleteInterlangLinkChecker, "incomplete_interlang");
issue!(IncompleteInterlangLink, "incomplete_interlang");
issue!(ConflictInterlang, "conflict_interlang");

impl IssueTrait for IncompleteInterlangLinkIssue {}
impl IssueTrait for ConflictInterlangIssue {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct IncompleteInterlangLinkDetails {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConflictInterlangIssueDetails {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterlangLinksGraph {

}

impl ComputedResource for InterlangLinksGraph {
    fn compute(ctx: Arc<CheckContext>) -> Result<Self> {
        todo!()
    }
}

#[async_trait]
impl CheckerTrait for IncompleteInterlangLinkChecker {
	fn possible_issues(&self) -> Vec<IssueType> {
		issues![IncompleteInterlangLink, ConflictInterlang]
	}
	async fn check(&self, ctx: Arc<CheckContext>) -> CheckResult {
		ctx.found::<ConflictInterlangIssue, _>(json!({}))?;
		let _graph = ctx.compute_resource::<InterlangLinksGraph>();
		Ok(())
	}
}
