use crate::commit::CommitType::*;
use crate::Rule;
use pest::iterators::Pair;

/// A commit type consist of a noun describing the kind of modification made.
/// In addition to the mandatory `fix` and `feat` type, common commit types taken from
/// [the angular convention](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)
/// as their own enum variant. Other type will be parser as [`CommitType::Custom`]
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum CommitType {
    /// *feat*: a commit of the type `feat` introduces a new feature to the codebase (this correlates with `MINOR` in Semantic Versioning).
    Feature,
    /// *fix*: a commit of the type `fix` patches a bug in your codebase (this correlates with `PATCH` in Semantic Versioning).
    BugFix,
    /// *chore*: Miscellaneous chores
    Chore,
    /// See [How does Conventional Commits handle revert commits?](https://www.conventionalcommits.org/en/v1.0.0/#how-does-conventional-commits-handle-revert-commits)
    Revert,
    /// *perf*: A code change that improves performance
    Performances,
    /// *docs*: Documentation only changes
    Documentation,
    /// *style*: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
    Style,
    /// *refactor*: A code change that neither fixes a bug nor adds a feature
    Refactor,
    /// *test*: Adding missing tests or correcting existing tests
    Test,
    /// *build*: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)
    Build,
    /// *ci*: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
    Ci,
    /// A custom commit type, can be anything
    Custom(String),
}

/// One or more footers MAY be provided one blank line after the body. Each footer MUST consist of
/// a word token, followed by either a :<space> or <space># separator, followed by a string value.
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Footer {
    /// The footer token, either BREAKING CHANGE or a work token
    pub token: String,
    /// A string value holding the footer message
    pub content: String,
}

/// A conventional commit compliant commit message produced by the [parse] function
///
/// [parse]: crate::ConventionalCommitParser::parse
#[derive(Debug, Eq, PartialEq)]
pub struct ConventionalCommit {
    /// The commit type, `fix`, `feat` etc.
    pub commit_type: CommitType,
    /// An optional scope
    pub scope: Option<String>,
    /// Commit description summary
    pub summary: String,
    /// An optional commit body
    pub body: Option<String>,
    /// A list of commit  footers
    pub footers: Vec<Footer>,
    /// A commit that has a footer `BREAKING CHANGE` or a `!` after the commit type and scope
    pub is_breaking_change: bool,
}

impl Default for ConventionalCommit {
    fn default() -> Self {
        ConventionalCommit {
            commit_type: Feature,
            scope: None,
            body: None,
            footers: vec![],
            summary: "".to_string(),
            is_breaking_change: false,
        }
    }
}

impl ConventionalCommit {
    pub(crate) fn set_breaking_change(&mut self, pair: Pair<Rule>) {
        if !pair.as_str().is_empty() {
            self.is_breaking_change = true
        }
    }

    pub(crate) fn set_summary_content(&mut self, pair: Pair<Rule>) {
        let summary = pair.as_str();
        self.summary = summary.to_string();
    }

    pub(crate) fn set_scope(&mut self, pair: Pair<Rule>) {
        if let Some(scope) = pair.into_inner().next() {
            let scope = scope.as_str();
            if !scope.is_empty() {
                self.scope = Some(scope.to_string())
            }
        };
    }

    pub(crate) fn set_commit_type(&mut self, pair: &Pair<Rule>) {
        let commit_type = pair.as_str();
        let commit_type = CommitType::from(commit_type);
        self.commit_type = commit_type;
    }

    pub(crate) fn set_commit_body(&mut self, pair: Pair<Rule>) {
        if let Some(body) = pair.into_inner().next() {
            let body = body.as_str();
            if !body.is_empty() {
                self.body = Some(body.to_string())
            }
        };
    }

    pub(crate) fn set_footers(&mut self, pair: Pair<Rule>) {
        for footer in pair.into_inner() {
            self.set_footer(footer);
        }
    }

    fn set_footer(&mut self, footer: Pair<Rule>) {
        let mut footer_pairs = footer.into_inner();
        let token = footer_pairs.next().unwrap().as_str().to_string();

        if token == "BREAKING CHANGE" || token == "BREAKING-CHANGE" {
            self.is_breaking_change = true;
        }

        let _separator = footer_pairs.next().unwrap();
        let content = footer_pairs.next().unwrap().as_str().to_string();

        self.footers.push(Footer { token, content });
    }
}

impl From<&str> for CommitType {
    fn from(commit_type: &str) -> Self {
        match commit_type.to_ascii_lowercase().as_str() {
            "feat" => Feature,
            "fix" => BugFix,
            "chore" => Chore,
            "revert" => Revert,
            "perf" => Performances,
            "docs" => Documentation,
            "style" => Style,
            "refactor" => Refactor,
            "test" => Test,
            "build" => Build,
            "ci" => Ci,
            other => Custom(other.to_string()),
        }
    }
}

impl Default for CommitType {
    fn default() -> Self {
        CommitType::Chore
    }
}
