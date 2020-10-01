use crate::DynErrResult;
use chrono::Duration;
use git2::{BranchType, Commit, Cred, FetchOptions, FetchPrune, RemoteCallbacks, Repository, Time};
use std::{env, error::Error};

use super::search_interval::SearchInterval;

#[derive(Debug)]
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
    pub interval: SearchInterval,
    pub repo: Repository,
}

impl RepoAnalyzer {
    pub fn new(repo_path: &str) -> DynErrResult<Self> {
        let repo = Repository::open(repo_path)?;
        Ok(Self {
            repo,
            interval: SearchInterval::start_now(Duration::weeks(2)),
        })
    }

    #[allow(dead_code)]
    pub fn set_interval(&mut self, interval: SearchInterval) {
        self.interval = interval
    }

    pub fn get_commits(&self) -> Result<Vec<RetroCommit>, Box<dyn Error>> {
        let SearchInterval { from, to } = self.interval;
        let (from, to) = self.interval.get_git_time();
        let merged = self.get_merged(from, to)?;
        Ok(merged)
    }

    pub fn get_in_progress(&self) -> DynErrResult<Vec<WorkingBranch>> {
        self.fetch_all()?;
        let (from, to) = self.interval.get_git_time();
        let branch_iter = self.repo.branches(Some(BranchType::Remote))?;
        let working_branches: DynErrResult<Vec<WorkingBranch>> =
            branch_iter.fold(Ok(vec![]), |working_branches, branch| {
                let mut working_branches = working_branches?;
                let (branch, _) = branch?;
                let name = branch.name()?.map(String::from).unwrap_or_default();
                match &name[..] {
                    "origin/HEAD" => Ok(working_branches),
                    release if release.contains("release-") => Ok(working_branches),
                    rest => {
                        let reference = branch.get().resolve()?;
                        if let Some(oid) = reference.target() {
                            let commit = self.repo.find_commit(oid)?;
                            if self.is_commit_in_range(&commit, &from, &to) {
                                working_branches.push(WorkingBranch {
                                    author: commit
                                        .author()
                                        .name()
                                        .map(String::from)
                                        .unwrap_or_default(),
                                    name: rest.into(),
                                })
                            }
                        }

                        Ok(working_branches)
                    }
                }
            });
        working_branches
    }

    fn get_merged(&self, from: Time, to: Time) -> DynErrResult<Vec<RetroCommit>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        let commits = revwalk
            .filter_map(|oid| {
                if let Ok(oid) = oid {
                    self.repo.find_commit(oid).ok()
                } else {
                    None
                }
            })
            .skip_while(|commit| !self.is_commit_in_range(commit, &from, &to))
            .take_while(|commit| self.is_commit_in_range(commit, &from, &to))
            .map(RetroCommit::from)
            .collect::<Vec<RetroCommit>>();
        Ok(commits)
    }

    fn is_commit_in_range(&self, commit: &Commit, from: &Time, to: &Time) -> bool {
        let commit_time_secs = commit.time().seconds();
        commit_time_secs > from.seconds() && commit_time_secs < to.seconds()
    }

    fn fetch_all(&self) -> DynErrResult<()> {
        let mut cbs = RemoteCallbacks::new();
        cbs.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });
        let mut options = FetchOptions::new();
        options.prune(FetchPrune::On).remote_callbacks(cbs);
        self.repo
            .find_remote("origin")?
            .fetch(&["master"], Some(&mut options), None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::git::search_interval::SearchInterval;
    use chrono::{Duration, NaiveDate, NaiveDateTime};

    fn day_with_commits() -> NaiveDateTime {
        NaiveDate::from_ymd(2020, 05, 24).and_hms(22, 51, 28)
    }

    #[test]
    fn test_get_commits() {
        let mut repo = super::RepoAnalyzer::new("./").expect("Cant open path ./");
        repo.set_interval(SearchInterval::starting(
            day_with_commits(),
            Duration::days(1),
        ));
        let commits = repo.get_commits();
        assert!(commits.is_ok());
        assert_eq!(commits.as_ref().unwrap().iter().count(), 5);
        assert_eq!(
            commits.as_ref().unwrap()[4].message,
            String::from("Add dev and production environment")
        );
    }

    #[test]
    fn test_get_branches() {
        use super::{SearchInterval, WorkingBranch};
        use chrono::Duration;

        let mut repo = super::RepoAnalyzer::new("./").unwrap();
        repo.set_interval(SearchInterval::starting(
            day_with_commits(),
            Duration::weeks(2),
        ));
        let branches = repo.get_in_progress();
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
