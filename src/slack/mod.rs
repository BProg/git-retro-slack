use reqwest::blocking;

use crate::{DynErrResult, cli::log, git::{search_interval::SearchInterval, RetroCommit, WorkingBranch}};
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Message {
    text: String
}

pub struct MessageIngredients<C, B>
where
    C: AsRef<[RetroCommit]>,
    B: AsRef<[WorkingBranch]>,
{
    pub commits: C,
    pub branches: B,
    pub interval: SearchInterval,
}

impl<C, B> MessageIngredients<C, B>
where
    C: AsRef<[RetroCommit]>,
    B: AsRef<[WorkingBranch]>,
{
    pub fn format_slack(&self) -> String {
        let mut author_commit_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let format_commit = |commit: &RetroCommit| format!("[done] {}", commit.message);
        let format_branch = |branch: &WorkingBranch| format!("[in-progress] {}", branch.name);

        for commit in self.commits.as_ref() {
            if let Some(authors_commits) = author_commit_map.get_mut(&commit.author) {
                authors_commits.push(format_commit(commit));
            } else {
                author_commit_map.insert(commit.author.clone(), vec![format_commit(commit)]);
            }
        }

        for branch in self.branches.as_ref() {
            if let Some(authors_commits) = author_commit_map.get_mut(&branch.author) {
                authors_commits.push(format_branch(branch));
            } else {
                author_commit_map.insert(branch.author.clone(), vec![format_branch(branch)]);
            }
        }

        let mut message = format!("Team git-status from {} to {}\n", self.interval.from, self.interval.to);
        for (author, jobs) in author_commit_map {
            message.push_str(&format!("_{}_\n", author));
            message.push_str("```\n");
            for job in jobs {
                message.push_str(&format!("    {}\n", job));
            }
            message.push_str("```\n");
        }
        message
    }

    pub fn send_to_slack<T: AsRef<str>>(&self, hook: T, log: T) -> DynErrResult<blocking::Response> {
        let escaped = serde_json::ser::to_string(log.as_ref())?;
        let payload = format!("{{\"text\": {}}}", escaped);
        log::message(format!("Sending to slack \n{}", &payload));
        let client = blocking::Client::new();
        client
            .post(hook.as_ref())
            .body(payload)
            .send()
            .map_err(Box::from)
    }
}
