pub mod page;
pub mod issue;

pub use page::Model as DbPage;
pub use page::ActiveModel as ActiveDbPage;

pub use issue::Model as DbIssue;
pub use issue::ActiveModel as ActiveDbIssue;
