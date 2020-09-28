use std::collections::BTreeMap;

use crate::git::{RetroCommit, WorkingBranch};

pub fn create_message<C, B>(commits: C, branches: B) -> String
where
    C: AsRef<[RetroCommit]>,
    B: AsRef<[WorkingBranch]>,
{
    let mut author_commit_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let format_commit = |commit: &RetroCommit| format!("[done] {}", commit.message);
    let format_branch = |branch: &WorkingBranch| format!("[in-progress] {}", branch.name);

    for commit in commits.as_ref() {
        if let Some(authors_commits) = author_commit_map.get_mut(&commit.author) {
            authors_commits.push(format_commit(commit));
        } else {
            author_commit_map.insert(commit.author.clone(), vec![format_commit(commit)]);
        }
    }

    for branch in branches.as_ref() {
        if let Some(authors_commits) = author_commit_map.get_mut(&branch.author) {
            authors_commits.push(format_branch(branch));
        } else {
            author_commit_map.insert(branch.author.clone(), vec![format_branch(branch)]);
        }
    }

    let mut message = String::new();
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
