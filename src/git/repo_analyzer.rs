use crate::log;
use chrono::{Duration};
use git2::{Branch, BranchType, Commit, Repository, Time};
use std::error::Error;

use super::search_interval::SearchInterval;

pub struct RepoAnalyzer {
    interval: SearchInterval,
    repo: Result<Repository, git2::Error>,
}

impl RepoAnalyzer {
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo: Repository::open(repo_path),
            interval: SearchInterval::start_now(Duration::weeks(2)),
        }
    }

    #[allow(dead_code)]
    pub fn set_interval(&mut self, interval: SearchInterval) {
        self.interval = interval
    }

    pub fn get_log(&self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self.get_commits()?.iter().map(summarize).collect())
    }

    pub fn get_commits(&self) -> Result<Vec<Commit>, Box<dyn Error>> {
        let SearchInterval { from, to } = self.interval;
        log::multiple(vec![
            log::Style::Message("Searching logs from: "),
            log::Style::Important(&from.to_string()),
            log::Style::Message(" to "),
            log::Style::Important(&to.to_string()),
        ]);
        let (from, to) = self.interval.get_git_time();
        let merged = self.get_merged(from, to)?;
        Ok(merged)
    }

    #[allow(dead_code)]
    pub fn get_in_progress(&self, from: Time, to: Time) -> Result<Vec<String>, Box<dyn Error>> {
        let repo = self
            .repo
            .as_ref()
            .map_err(|e| Box::new(clone_git2_error(e)))?;
        let branches = repo.branches(Some(BranchType::Remote))?;
        let names = branches
            .filter_map(|branch_and_type| {
                if let Ok((branch, _)) = branch_and_type {
                    match self.is_branch_in_range(&branch, &from, &to) {
                        Ok(in_range) if in_range => return Some(branch),
                        _ => return None,
                    }
                }
                None
            })
            .filter_map(|branch| branch.name().ok().flatten().map(String::from))
            .collect();
        Ok(names)
    }

    fn get_merged(&self, from: Time, to: Time) -> Result<Vec<Commit>, Box<dyn Error>> {
        let repo = self
            .repo
            .as_ref()
            .map_err(|e| Box::new(clone_git2_error(e)))?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        let commits = revwalk
            .map(|oid| repo.find_commit(oid?))
            .skip_while(|commit| {
                commit
                    .as_ref()
                    .map(|commit| !self.is_commit_in_range(commit, &from, &to))
                    .unwrap_or(true)
            })
            .take_while(|commit| {
                commit
                    .as_ref()
                    .map(|commit| self.is_commit_in_range(commit, &from, &to))
                    .unwrap_or(false)
            })
            .filter_map(|commit| commit.ok())
            .collect::<Vec<Commit>>();
        Ok(commits)
    }

    fn is_branch_in_range(
        &self,
        branch: &Branch,
        from: &Time,
        to: &Time,
    ) -> Result<bool, Box<dyn Error>> {
        let reference = branch.get().resolve()?;
        if let Some(oid) = reference.target() {
            let repo = self.repo.as_ref().map_err(clone_git2_error)?;
            let commit = repo.find_commit(oid)?;
            return Ok(self.is_commit_in_range(&commit, &from, &to));
        }
        Ok(false)
    }

    fn is_commit_in_range(&self, commit: &Commit, from: &Time, to: &Time) -> bool {
        let commit_time_secs = commit.time().seconds();
        commit_time_secs > from.seconds() && commit_time_secs < to.seconds()
    }
}

fn clone_git2_error(error: &git2::Error) -> git2::Error {
    git2::Error::from_str(error.message())
}

fn summarize(commit: &Commit) -> String {
    let author: String = commit.author().name().map_or("".into(), |name| name.into());
    let short = commit
        .summary()
        .map(String::from)
        .unwrap_or_else(|| "".into());
    format!("{} {}", author, short)
}


#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate, NaiveDateTime};
    use crate::git::search_interval::SearchInterval;

    fn day_with_commits() -> NaiveDateTime {
        NaiveDate::from_ymd(2020, 05, 24).and_hms(22, 51, 28)
    }

    #[test]
    fn test_get_log() {
        let mut repo = super::RepoAnalyzer::new("./");
        repo.set_interval(SearchInterval::starting(day_with_commits(), Duration::weeks(2)));
        let log = repo.get_log();
        assert!(log.is_ok());
        assert_eq!(
            log.as_ref().unwrap(),
            &vec![
                "Ion Ostafi Add TODO in the readme",
                "Ion Ostafi Fix programming language for README usage",
                "Ion Ostafi Update readme and remove authore from usage",
                "Ion Ostafi Add usage message when using --help option",
                "Ion Ostafi Add dev and production environment",
                "Ion Ostafi Add basic readme",
                "Ion Ostafi cover with some tests the message module",
                "Ion Ostafi Fix 0 commits message",
                "Ion Ostafi Rename app to girretro and add invalid command output",
                "Ion Ostafi Initial commit with basic functionality"
            ]
        );
    }

    #[test]
    fn test_get_commits() {
        let mut repo = super::RepoAnalyzer::new("./");
        repo.set_interval(SearchInterval::starting(day_with_commits(), Duration::weeks(2)));
        let commits = repo.get_commits();
        assert!(commits.is_ok());
        assert_eq!(commits.unwrap().iter().count(), 10);
    }

    #[test]
    fn test_get_branches() {
        use super::{SearchInterval};
        use chrono::Duration;

        let repo = super::RepoAnalyzer::new("./");
        let (from, to) = SearchInterval::starting(day_with_commits(), Duration::weeks(2)).get_git_time();
        let names = repo.get_in_progress(from, to);
        assert!(names.is_ok());
        assert_eq!(
            names.as_ref().unwrap(),
            &vec![
                "origin/do_not_delete_used_for_tests_1",
                "origin/do_not_delete_used_for_tests_2"
            ]
        );
    }
}
