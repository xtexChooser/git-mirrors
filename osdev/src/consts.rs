// https://osdev.wiki/wiki/User:PsiBot/Tasks#6847-mark-uncat-redir
pub mod uncatredir {
	pub const REDIRECT_PAGE_TEMPLATE: &str = "Template:Redirect page";

	pub const STUB: &str = "{{Redirect page}}";

	pub const SUMMARY: &str =
	    "Bot: [[User:PsiBot/Tasks#6847-mark-uncat-redir|Mark uncategorized redirects]]";
}

// https://osdev.wiki/wiki/User:PsiBot/Tasks#6f76-categorize-redirs
pub mod categorize_redir {
	pub const REDIRECT_PAGE_TEMPLATE: &str = "Template:Redirect page";
	pub const UNCATEGORIZED_CATEGORIES: &str =
		"Category:Uncategorized redirects";

	pub const TEMPLATE_MOVED: &str = "R from move";
	pub const SUMMARY_MOVED: &str =
	    "Bot: [[User:PsiBot/Tasks#6f76-categorize-redirs|Categorize redirect page, from move]]";
}
