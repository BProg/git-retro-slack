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
        for commit in self.commits.as_ref() {
            let commits = author_commit_map.entry(commit.author.clone()).or_default();
            commits.push(format!("[done] {}", commit.message));
        }
        for branch in self.branches.as_ref() {
            let commits = author_commit_map.entry(branch.author.clone()).or_default();
            commits.push(format!("[in-progress] {}", branch.name));
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
        let msg = Message {
            text: log.as_ref().to_string()
        };
        let payload = serde_json::ser::to_string(&msg)?;
        log::message(format!("Sending to slack \n{}", &payload));
        let client = blocking::Client::new();
        client
            .post(hook.as_ref())
            .body(payload)
            .send()
            .map_err(Box::from)
    }
}
