use crate::log;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use git2::{Branch, BranchType, Commit, Repository, Time};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

pub mod commit;

pub struct GitRepo {
    today: NaiveDateTime,
    repo: Result<Repository, git2::Error>,
}

impl GitRepo {
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo: Repository::open(repo_path),
            today: today(),
        }
    }

    #[cfg(test)]
    fn new_test(repo_path: &str) -> Self {
        Self {
            repo: Repository::open(repo_path),
            today: day_with_commits(),
        }
    }

    pub fn get_log(&self) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(self.get_commits()?.iter().map(summarize).collect())
    }

    pub fn get_commits(&self) -> Result<Vec<Commit>, Box<dyn Error>> {
        let (from, to) = last_two_weeks(self.today);
        log::multiple(vec![
            log::Style::Message("Searching logs from: "),
            log::Style::Important(&from.to_string()),
            log::Style::Message(" to "),
            log::Style::Important(&to.to_string()),
        ]);
        let (from, to) = (Time::new(from.timestamp(), 0), Time::new(to.timestamp(), 0));
        let merged = self.get_merged(from, to)?;
        Ok(merged)
    }

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

fn last_two_weeks(today: NaiveDateTime) -> (NaiveDateTime, NaiveDateTime) {
    let two_weeks_ago = today - Duration::weeks(2);
    (two_weeks_ago, today)
}

/// If it fails to create a date time becuase of system time being incorrect on machine,
/// then, it will return the date time this function was created
fn today() -> NaiveDateTime {
    if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
        NaiveDateTime::from_timestamp(now.as_secs() as i64, 0)
    } else {
        NaiveDate::from_ymd(2020, 4, 28).and_hms(22, 51, 28)
    }
}

#[cfg(test)]
fn day_with_commits() -> NaiveDateTime {
    NaiveDate::from_ymd(2020, 05, 24).and_hms(22, 51, 28)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    #[test]
    fn get_time_range() {
        // given
        let a_day = NaiveDate::from_ymd(2020, 05, 01).and_hms(22, 22, 22);
        let two_weeks_ago_timestamp = NaiveDate::from_ymd(2020, 04, 17)
            .and_hms(22, 22, 22)
            .timestamp();

        // when
        let (from, _) = super::last_two_weeks(a_day);

        // then
        assert_eq!(two_weeks_ago_timestamp, from.timestamp());
    }

    #[test]
    fn test_get_log() {
        let repo = super::GitRepo::new_test("./");
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
        let repo = super::GitRepo::new_test("./");
        let commits = repo.get_commits();
        assert!(commits.is_ok());
        assert_eq!(commits.unwrap().iter().count(), 10);
    }

    #[test]
    fn test_get_branches() {
        use crate::git::{day_with_commits, last_two_weeks};
        use git2::Time;

        let repo = super::GitRepo::new_test("./");
        let (today, two_weeks_ago) = last_two_weeks(day_with_commits());
        let (from, to) = (
            Time::new(today.timestamp(), 0),
            Time::new(two_weeks_ago.timestamp(), 0),
        );
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
