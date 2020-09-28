use std::collections::BTreeMap;

use crate::git::{RetroCommit, WorkingBranch};

const SUFFIX_EMOJIES: [char; 10] = ['🙌', '👍', '🙏', '🎉', '🚀', '🤘', '👏', '🙌', '👍', '🙏'];
const START_ROW: &str = r#"
A reminder on how cool you all are 😎
A year ago, this same day you've written history 📜
"#;

pub fn prettify(commits: &[String]) -> String {
    let mut index = 0usize;
    let mut commits_count = 0u16;
    let mut pretty = commits
        .iter()
        .fold(String::from(START_ROW), |mut pretty, commit| {
            pretty.push_str(&format!("☞ {} {} \n", commit, SUFFIX_EMOJIES[index]));
            index = increase_index(index);
            commits_count += 1;
            pretty
        });
    pretty.push_str(&format!("{} merged commits in one day", commits_count));
    if commits_count == 0 {
        "".into()
    } else {
        pretty
    }
}

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

fn increase_index(i: usize) -> usize {
    let next = i + 1;
    if next >= SUFFIX_EMOJIES.len() {
        0
    } else {
        next
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_0_commits_message_is_empty() {
        let message = super::prettify(&vec![]);
        assert!(message.len() == 0);
    }

    #[test]
    fn test_increase_max_allowed_index() {
        let next_index = super::increase_index(super::SUFFIX_EMOJIES.len() - 1);
        assert_eq!(0, next_index);
    }
}
