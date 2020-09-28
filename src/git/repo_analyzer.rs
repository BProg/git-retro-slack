use crate::log;
use chrono::Duration;
use git2::{BranchType, Commit, Repository, Time};
use std::error::Error;

use super::search_interval::SearchInterval;

pub struct RetroCommit {
    pub author: String,
    pub message: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorkingBranch {
    pub author: String,
    pub name: String,
}

impl<'repo> From<git2::Commit<'repo>> for RetroCommit {
    fn from(commit: git2::Commit<'repo>) -> RetroCommit {
        RetroCommit {
            author: commit.author().name().map(String::from).unwrap_or_default(),
            message: commit.summary().map(String::from).unwrap_or_default(),
        }
    }
}

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

    pub fn get_commits(&self) -> Result<Vec<RetroCommit>, Box<dyn Error>> {
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

    pub fn get_in_progress(
        &self,
        from: Time,
        to: Time,
    ) -> Result<Vec<WorkingBranch>, Box<dyn Error>> {
        let repo = self
            .repo
            .as_ref()
            .map_err(|e| Box::new(clone_git2_error(e)))?;
            // requires authentication logic
        // repo.find_remote("origin")?.fetch(&["master"], None, None)?;
        let branch_iter = repo.branches(Some(BranchType::Remote))?;
        let working_branches: Result<Vec<WorkingBranch>, Box<dyn Error>> =
            branch_iter.fold(Ok(vec![]), |working_branches, branch| {
                let mut working_branches = working_branches?;
                let (branch, _) = branch?;
                let reference = branch.get().resolve()?;
                if let Some(oid) = reference.target() {
                    let commit = repo.find_commit(oid)?;
                    if self.is_commit_in_range(&commit, &from, &to) {
                        working_branches.push(WorkingBranch {
                            author: commit.author().name().map(String::from).unwrap_or_default(),
                            name: branch.name()?.map(String::from).unwrap_or_default(),
                        })
                    }
                }

                Ok(working_branches)
            });
        working_branches
    }

    fn get_merged(&self, from: Time, to: Time) -> Result<Vec<RetroCommit>, Box<dyn Error>> {
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
            .map(RetroCommit::from)
            .collect::<Vec<RetroCommit>>();
        Ok(commits)
    }

    fn is_commit_in_range(&self, commit: &Commit, from: &Time, to: &Time) -> bool {
        let commit_time_secs = commit.time().seconds();
        commit_time_secs > from.seconds() && commit_time_secs < to.seconds()
    }
}

fn clone_git2_error(error: &git2::Error) -> git2::Error {
    git2::Error::from_str(error.message())
}

fn summarize(commit: &RetroCommit) -> String {
    format!("{} {}", commit.author, commit.message)
}

#[cfg(test)]
mod tests {
    use crate::git::search_interval::SearchInterval;
    use chrono::{Duration, NaiveDate, NaiveDateTime};

    fn day_with_commits() -> NaiveDateTime {
        NaiveDate::from_ymd(2020, 05, 24).and_hms(22, 51, 28)
    }

    #[test]
    fn test_get_log() {
        let mut repo = super::RepoAnalyzer::new("./");
        repo.set_interval(SearchInterval::starting(
            day_with_commits(),
            Duration::weeks(2),
        ));
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
        repo.set_interval(SearchInterval::starting(
            day_with_commits(),
            Duration::weeks(2),
        ));
        let commits = repo.get_commits();
        assert!(commits.is_ok());
        assert_eq!(commits.unwrap().iter().count(), 10);
    }

    #[test]
    fn test_get_branches() {
        use super::{SearchInterval, WorkingBranch};
        use chrono::Duration;

        let repo = super::RepoAnalyzer::new("./");
        let (from, to) =
            SearchInterval::starting(day_with_commits(), Duration::weeks(2)).get_git_time();
        let branches = repo.get_in_progress(from, to);
        assert!(branches.is_ok());
        assert_eq!(
            branches.as_ref().unwrap(),
            &vec![
                WorkingBranch {
                    name: "origin/do_not_delete_used_for_tests_1".into(),
                    author: "Ion Ostafi".into()
                },
                WorkingBranch {
                    name: "origin/do_not_delete_used_for_tests_2".into(),
                    author: "Ion Ostafi".into()
                }
            ]
        );
    }
}
