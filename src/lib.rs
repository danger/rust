#[macro_use]
extern crate serde_derive;

// The JSON Schema -> Types uses serde, to use the macros that this generates
// we have to declare that this entire bin will use macros

extern crate serde;
extern crate serde_json;

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerCommitAuthor {
  /// The display name of the commit author
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  /// The email of the commit author
  #[serde(rename = "emailAddress")]
  pub email_address: Option<String>,
  /// The id of the commit author
  pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerCommitAuthorCommitter {
  /// The display name of the commit committer
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  /// The email of the commit committer
  #[serde(rename = "emailAddress")]
  pub email_address: Option<String>,
  /// The id of the commit committer
  pub name: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerCommitAuthorCommitterItemParents {
  /// The simplified sha
  #[serde(rename = "displayId")]
  pub display_id: Option<String>,
  /// The full SHA
  pub id: Option<String>,
}
/// A BitBucketServer specific implementation of a git commit.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerCommit {
  /// The author of the commit, assumed to be the person who wrote the code.
  pub author: Option<BitBucketServerCommitAuthor>,
  /// The UNIX timestamp for when the commit was authored
  #[serde(rename = "authorTimestamp")]
  pub author_timestamp: Option<f64>,
  /// The author of the commit, assumed to be the person who commited/merged the code into a
  /// project.
  pub committer: Option<BitBucketServerCommitAuthorCommitter>,
  /// When the commit was commited to the project
  #[serde(rename = "committerTimestamp")]
  pub committer_timestamp: Option<f64>,
  /// The shortened SHA for the commit
  #[serde(rename = "displayId")]
  pub display_id: Option<String>,
  /// The SHA for the commit
  pub id: Option<String>,
  /// The commit's message
  pub message: Option<String>,
  /// The commit's parents
  pub parents: Option<Vec<BitBucketServerCommitAuthorCommitterItemParents>>,
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerJSONDSL {
  /// The PR metadata
  // pub pr: Option<BitBucketServerPRDSL>,
  /// The activities such as OPENING, CLOSING, MERGING or UPDATING a pull request
  // pub activities: Option<Vec<BitBucketServerPRActivity>>,
  /// The comments on the pull request
  // pub comments: Option<Vec<BitBucketServerPRActivity>>,
  /// The commits associated with the pull request
  pub commits: Option<Vec<BitBucketServerCommit>>,
  /// The related JIRA issues
  // pub issues: Option<Vec<Jiraissue>>,
  /// The pull request and repository metadata
  pub metadata: Option<RepoMetaData>,
}

// #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
/// of it for tooling's sake, but any extra metadata which BitBucket Server send
/// will still be inside the JS object.
// #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
// #[derive(Clone, PartialEq, Debug, Default)]
// pub struct BitBucketServerPRDSL {
//   /// The creator of the PR
//   pub author: Option<serde_json::Value>,
//   /// Is the PR closed?
//   pub closed: Option<bool>,
//   /// Date PR created as number of milliseconds since the unix epoch
//   #[serde(rename = "createdDate")]
//   pub created_date: Option<f64>,
//   /// The text describing the PR
//   pub description: Option<String>,
//   /// The PR submitter's reference
//   #[serde(rename = "fromRef")]
//   // pub from_ref: Option<BitBucketServerMergeRef>,
//   /// The PR's ID
//   pub id: Option<f64>,
//   /// Misc links for hypermedia conformance
//   pub links: Option<String>,
//   /// Was this PR locked?
//   pub locked: Option<bool>,
//   /// Is the PR open?
//   pub open: Option<bool>,
//   /// People who have participated in the PR
//   pub participants: Option<Vec<serde_json::Value>>,
//   /// People requested as reviewers
//   pub reviewers: Option<Vec<serde_json::Value>>,
//   /// The pull request's current status.
//   pub state: Option<String>,
//   /// Title of the pull request.
//   pub title: Option<String>,
//   /// The repo Danger is running on
//   #[serde(rename = "toRef")]
//   // pub to_ref: Option<BitBucketServerMergeRef>,
//   /// Date PR updated as number of milliseconds since the unix epoch
//   #[serde(rename = "updatedDate")]
//   pub updated_date: Option<f64>,
//   /// The API version
//   pub version: Option<f64>,
// }
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerPRParticipant {
  /// Did they approve of the PR?
  pub approved: Option<bool>,
  /// How did they contribute
  pub role: Option<String>,
  /// Their review feedback
  pub status: Option<String>,
  pub user: Option<BitBucketServerUser>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerRepoProject {
  /// The project unique id
  pub id: Option<f64>,
  /// The project's human readable project key
  pub key: Option<String>,
  /// Hyperlinks for the project
  pub links: Option<String>,
  /// The name of the project
  pub name: Option<String>,
  /// Is the project publicly available
  pub public: Option<bool>,
  /// The project's type
  #[serde(rename = "type")]
  pub type_: Option<String>,
}
/// A BitBucket Server Repo
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerRepo {
  /// Can someone fork this repo?
  pub forkable: Option<bool>,
  /// Links for the projects
  // pub links: Option<BitBucketServerLinks>,
  /// The repo name
  pub name: Option<String>,
  /// An abstraction for grouping repos
  pub project: Option<BitBucketServerRepoProject>,
  /// Is the repo public?
  pub public: Option<bool>,
  /// The type of SCM tool, probably "git"
  #[serde(rename = "scmId")]
  pub scm_id: Option<String>,
  /// The slug for the repo
  pub slug: Option<String>,
}
/// A BitBucketServer user account.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BitBucketServerUser {
  /// Is the account active
  pub active: Option<bool>,
  /// The name to use when referencing the user
  #[serde(rename = "displayName")]
  pub display_name: Option<String>,
  /// The email for the user
  #[serde(rename = "emailAddress")]
  pub email_address: Option<String>,
  /// The unique user ID
  pub id: Option<f64>,
  /// The name of the user
  pub name: Option<String>,
  /// The user's slug for URLs
  pub slug: Option<String>,
  /// The type of a user, "NORMAL" being a typical user3
  #[serde(rename = "type")]
  pub type_: Option<String>,
}
/// Describes the possible arguments that
/// could be used when calling the CLI
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct CliArgs {
  pub base: Option<String>,
  pub dangerfile: Option<String>,
  #[serde(rename = "externalCiProvider")]
  pub external_ci_provider: Option<String>,
  pub id: Option<String>,
  #[serde(rename = "textOnly")]
  pub text_only: Option<String>,
  pub verbose: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DangerDSLJSONTypeSettingsGithub {
  /// API token for the GitHub client to use
  #[serde(rename = "accessToken")]
  pub access_token: Option<String>,
  /// Optional headers to add to a request
  #[serde(rename = "additionalHeaders")]
  pub additional_headers: Option<serde_json::Value>,
  /// Optional URL for enterprise GitHub
  #[serde(rename = "baseURL")]
  pub base_url: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DangerDSLJSONTypeSettings {
  /// This is still a bit of a WIP, but this should
  /// pass args/opts from the original CLI call through
  /// to the process.
  #[serde(rename = "cliArgs")]
  pub cli_args: Option<CliArgs>,
  /// Saves each client re-implementing logic to grab these vars
  /// for their API clients
  pub github: Option<DangerDSLJSONTypeSettingsGithub>,
}
/// The root of the Danger JSON DSL.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DangerDSLJSONType {
  /// The data only version of BitBucket Server DSL
  pub bitbucket_server: Option<BitBucketServerJSONDSL>,
  /// The data only version of Git DSL
  pub git: GitJSONDSL,
  /// The data only version of GitHub DSL
  pub github: Option<GitHubDSL>,
  /// Used in the Danger JSON DSL to pass metadata between
  /// processes. It will be undefined when used inside the Danger DSL
  pub settings: DangerDSLJSONTypeSettings,
}
/// A platform agnostic reference to a Git commit
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitCommit {
  /// Who wrote the commit
  pub author: Option<GitCommitAuthor>,
  /// Who deployed the commit
  pub committer: Option<GitCommitAuthor>,
  /// The commit message
  pub message: Option<String>,
  /// SHAs for the commit's parents
  pub parents: Option<Vec<String>>,
  /// The SHA for the commit
  pub sha: Option<String>,
  /// Potential parent commits, and other assorted metadata
  pub tree: Option<serde_json::Value>,
  /// Link to the commit
  pub url: Option<String>,
}
/// An author of a commit
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitCommitAuthor {
  /// ISO6801 date string
  pub date: Option<String>,
  /// The authors email
  pub email: Option<String>,
  /// The display name for the author
  pub name: Option<String>,
}
/// Provides the current PR in an easily used way for params in `github.api` calls
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubAPIPR {
  /// The PR number
  pub number: Option<f64>,
  /// The repo owner
  pub owner: Option<String>,
  /// The repo name
  pub repo: Option<String>,
}
/// A GitHub specific implementation of a git commit, it has GitHub user names instead of an email.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubCommit {
  /// The GitHub user who wrote the code
  pub author: Option<GitHubUser>,
  /// The raw commit metadata
  pub commit: Option<GitCommit>,
  /// The GitHub user who shipped the code
  pub committer: Option<GitHubUser>,
  /// An array of parent commit shas
  pub parents: Option<Vec<serde_json::Value>>,
  /// The SHA for the commit
  pub sha: Option<String>,
  /// the url for the commit on GitHub
  pub url: Option<String>,
}
/// The GitHub metadata for your PR
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubDSL {
  /// An authenticated API so you can extend danger's behavior by using the [GitHub v3
  /// API](https://developer.github.com/v3/).
  ///
  /// A set up instance of the "github" npm module. You can get the full [API
  /// here](https://octokit.github.io/node-github/).
  pub api: Option<Github>,
  /// The github commit metadata for a code review session
  pub commits: Option<Vec<GitHubCommit>>,
  /// The issue metadata for a code review session
  pub issue: Option<GitHubIssue>,
  /// The PR metadata for a code review session
  pub pr: Option<GitHubPRDSL>,
  /// The people requested to review this PR
  // pub requested_reviewers: Option<Vec<GitHubUser>>,
  /// The reviews left on this pull request
  pub reviews: Option<Vec<GitHubReview>>,
  /// The PR metadata specifically formatted for using with the GitHub API client
  #[serde(rename = "thisPR")]
  pub this_pr: Option<GitHubAPIPR>,
  /// A scope for useful functions related to GitHub
  pub utils: Option<GitHubUtilsDSL>,
}
/// This is `danger.github.issue` It refers to the issue that makes up the Pull Request.
/// GitHub treats all pull requests as a special type of issue. This DSL contains only parts of the
/// issue that are
/// not found in the PR DSL, however it does contain the full JSON structure.
///
/// A GitHub Issue
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubIssue {
  /// The labels associated with this issue
  pub labels: Option<Vec<GitHubIssueLabel>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubIssueLabel {
  /// The color associated with this label
  pub color: Option<String>,
  /// The identifying number of this label
  pub id: Option<f64>,
  /// The name of the label
  pub name: Option<String>,
  /// The URL that links to this label
  pub url: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubMergeRef {
  /// The human display name for the merge reference, e.g. "artsy:master"
  pub label: Option<String>,
  /// The reference point for the merge, e.g. "master"
  pub refd: Option<String>,
  /// The repo from whch the reference comes from
  pub repo: Option<GitHubRepo>,
  /// The reference point for the merge, e.g. "704dc55988c6996f69b6873c2424be7d1de67bbe"
  pub sha: Option<String>,
  /// The user that owns the merge reference e.g. "artsy"
  pub user: Option<GitHubUser>,
}
/// An exact copy of the PR's reference JSON. This interface has type'd the majority
/// of it for tooling's sake, but any extra metadata which GitHub send will still be
/// inside the JS object.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubPRDSL {
  /// The number of additional lines in the PR
  pub additions: Option<f64>,
  /// The User who is assigned the PR
  pub assignee: Option<GitHubUser>,
  /// The Users who are assigned to the PR
  pub assignees: Option<Vec<GitHubUser>>,
  /// Merge reference for _this_ repo.
  pub base: Option<GitHubMergeRef>,
  /// The markdown body message of the PR
  pub body: Option<String>,
  /// The number of changed files in the PR
  pub changed_files: Option<f64>,
  /// optional ISO6801 Date string for when PR was closed
  #[serde(default)]
  pub closed_at: Option<String>,
  /// The number of comments on the PR
  pub comments: Option<f64>,
  /// The number of commits in the PR
  pub commits: Option<f64>,
  /// ISO6801 Date string for when PR was created
  pub created_at: Option<String>,
  /// The number of deleted lines in the PR
  pub deletions: Option<f64>,
  /// Merge reference for the _other_ repo.
  pub head: Option<GitHubMergeRef>,
  /// Has the PR been locked to contributors only?
  pub locked: Option<bool>,
  /// Has the PR been merged yet?
  pub merged: Option<bool>,
  /// Optional ISO6801 Date string for when PR was merged.
  /// Danger probably shouldn't be running in this state.
  #[serde(default)]
  pub merged_at: Option<String>,
  /// The UUID for the PR
  pub number: Option<f64>,
  /// The number of review-specific comments on the PR
  pub review_comments: Option<f64>,
  /// The state for the PR
  pub state: Option<String>,
  /// The title of the PR
  pub title: Option<String>,
  /// ISO6801 Date string for when PR was updated
  pub updated_at: Option<String>,
  /// The User who submitted the PR
  pub user: Option<GitHubUser>,
}
/// A GitHub Repo
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubRepo {
  /// Is someone assigned to this PR?
  pub assignee: Option<GitHubUser>,
  /// Are there people assigned to this PR?
  pub assignees: Option<Vec<GitHubUser>>,
  /// The textual description of the repo
  pub description: Option<String>,
  /// Is the repo a fork?
  pub fork: Option<bool>,
  /// The full name of the owner + repo, e.g. "Danger/Danger-JS"
  pub full_name: Option<String>,
  /// The root web URL for the repo, e.g. https://github.com/artsy/emission
  pub html_url: Option<String>,
  /// Generic UUID
  pub id: Option<f64>,
  /// The name of the repo, e.g. "Danger-JS"
  pub name: Option<String>,
  /// The owner of the repo
  pub owner: Option<GitHubUser>,
  /// Is the repo publicly accessible?
  pub private: Option<bool>,
}
/// GitHubReview
/// While a review is pending, it will only have a user.  Once a review is complete, the rest of
/// the review attributes will be present
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubReview {
  /// If there is a review, the body of the review
  pub body: Option<String>,
  /// If there is a review, the commit ID this review was made on
  pub commit_id: Option<String>,
  /// If there is a review, this provides the ID for it
  pub id: Option<f64>,
  /// The state of the review
  /// APPROVED, REQUEST_CHANGES, COMMENT or PENDING
  pub state: Option<String>,
  /// The user requested to review, or the user who has completed the review
  pub user: Option<GitHubUser>,
}
/// A GitHub user account.
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubUser {
  /// The url for a users's image
  pub avatar_url: Option<String>,
  /// Generic UUID
  pub id: Option<f64>,
  /// The handle for the user/org
  pub login: Option<String>,
  /// Whether the user is an org, or a user
  #[serde(rename = "type")]
  pub type_: Option<String>,
}
/// Useful functions for GitHub related work
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitHubUtilsDSL {
  /// An API for creating, or setting a label to an issue. Usable from Peril
  /// by adding an additional param for settings about a repo.
  #[serde(rename = "createOrAddLabel")]
  pub create_or_add_label: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  #[serde(rename = "createOrUpdatePR")]
  pub create_or_update_pr: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  /// An API for creating, updating and closing an issue. Basically
  /// this is really useful for reporting back via a separate
  /// issue that you may want to keep up to date at all times.
  #[serde(rename = "createUpdatedIssueWithID")]
  pub create_updated_issue_with_id: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
}
/// The Git Related Metadata which is available inside the Danger DSL JSON
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GitJSONDSL {
  /// The Git commit metadata
  pub commits: Vec<GitCommit>,
  /// Newly created filepaths relative to the git root
  pub created_files: Vec<String>,
  /// Removed filepaths relative to the git root
  pub deleted_files: Vec<String>,
  /// Filepaths with changes relative to the git root
  pub modified_files: Vec<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Github {
  pub activity: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub apps: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub authorization: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub checks: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub gists: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub gitdata: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub issues: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub migrations: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub misc: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub orgs: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub projects: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  #[serde(rename = "pullRequests")]
  pub pull_requests: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub reactions: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub repos: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub search: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
  pub users: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
}
/// This is `danger.bitbucket_server.issues` It refers to the issues that are linked to the Pull
/// Request.
// #[serde(rename = "JIRAIssue")]
// #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
// pub struct Jiraissue {
//   /// The unique key for the issue e.g. JRA-11
//   pub key: Option<String>,
//   /// The user-facing URL for that issue
//   pub url: Option<String>,
// }
/// Key details about a repo
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct RepoMetaData {
  /// The ID for the pull/merge request "11"
  #[serde(rename = "pullRequestID")]
  pub pull_request_id: Option<String>,
  /// A path like "artsy/eigen"
  #[serde(rename = "repoSlug")]
  pub repo_slug: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Schema {
  pub danger: FauxSchema,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct FauxSchema {
  pub danger: DangerDSLJSONType,
}

use std::io::{self, Read};

/// A function to grab the Danger DSL from the
/// DSL input
pub fn get_danger() -> DangerDSLJSONType {
  // Grab the incoming JSON
  let mut buffer = String::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  handle.read_to_string(&mut buffer).unwrap();

  let danger_dsl: Schema = serde_json::from_str(&buffer).unwrap();
  return danger_dsl.danger.danger;
}
