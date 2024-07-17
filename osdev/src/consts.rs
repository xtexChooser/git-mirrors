// https://osdev.wiki/wiki/User:PsiBot/Tasks#6f76-categorize-redirs
pub mod categorize_redir {
	pub const SUMMARY: &str =
	    "Bot: [[User:PsiBot/Tasks#6f76-categorize-redirs|Categorize redirect page]]";
	// https://osdev.wiki/wiki/User:PsiBot/Tasks#6847-mark-uncat-redir
	pub const SUMMARY_UNCATEGORIZED: &str = "Bot: [[User:PsiBot/Tasks#6847-mark-uncat-redir|Mark uncategorized redirects]]";

	pub const REDIRECT_PAGE_TEMPLATE: &str = "Template:Redirect page";
	pub const STUB: &str = "{{Redirect page}}";
	pub const UNCATEGORIZED_CATEGORIES: &str =
		"Category:Uncategorized redirects";

	pub const TEMPLATE_MOVED: &str = "R from move";
	pub const CATEGORY_MOVED: &str = "Category:Redirects from moves";

	pub const TEMPLATE_TEMPLATE_ALIAS: &str = "Template alias";
	pub const CATEGORY_TEMPLATE_ALIAS: &str = "Category:Template alias";

	pub const TEMPLATE_SECTION: &str = "R to section";
	pub const CATEGORY_SECTION: &str = "Category:Redirects to section";

	pub const TEMPLATE_PROJECT: &str = "R to project page";
	pub const CATEGORY_PROJECT: &str = "Category:Redirects to project page";
}
