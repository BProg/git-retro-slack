use crate::printer;
use chrono::{Duration, NaiveDate, NaiveDateTime};
use git2::{Commit, Repository, Time};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct GitRepo {
    today: NaiveDateTime,
    repo: Result<Repository, git2::Error>,
}

impl GitRepo {
    pub fn new(repo_path: &str) -> Self {
        Self {
            repo: Repository::open(repo_path),
            today: get_today_naive_date_time(),
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
        let (datetime_from, datetime_to) = last_two_weeks(&self.today);
        printer::print_time_range(&datetime_from, &datetime_to);
        let (time_from, time_to) = (
            Time::new(datetime_from.timestamp(), 0),
            Time::new(datetime_to.timestamp(), 0),
        );
        match &self.repo {
            Err(e) => Err(Box::new(clone_git2_error(e))),
            Ok(repo) => self.use_revwalk(repo, &time_from, &time_to),
        }
    }

    fn use_revwalk(
        &self,
        repo: &Repository,
        from: &Time,
        to: &Time,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        self.fold_commits(&mut revwalk, &repo, &from, &to)
    }

    fn fold_commits(
        &self,
        revwalk: &mut git2::Revwalk,
        repo: &Repository,
        from: &Time,
        to: &Time,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let summary = revwalk.fold(vec![], |mut summary, oid| {
            oid.and_then(|oid| repo.find_commit(oid))
                .and_then(|commit| {
                    if is_in_range(&commit, &from, &to) {
                        printer::print_commit(&commit);
                        summary.push(summarize(&commit));
                    }
                    Ok(summary)
                })
                .unwrap_or_default()
        });
        Ok(summary)
    }
}

fn clone_git2_error(error: &git2::Error) -> git2::Error {
    git2::Error::from_str(error.message())
}

fn is_in_range(commit: &Commit, from: &Time, to: &Time) -> bool {
    let commit_time_secs = commit.time().seconds();
    commit_time_secs > from.seconds() && commit_time_secs < to.seconds()
}

fn summarize(commit: &Commit) -> String {
    let author: String = commit.author().name().map_or("".into(), |name| name.into());
    let short = commit
        .summary()
        .map(|s| String::from(s))
        .unwrap_or("".into());
    format!("{} {}", author, short)
}

fn last_two_weeks(today: &NaiveDateTime) -> (NaiveDateTime, NaiveDateTime) {
    let two_weeks_ago = *today - Duration::weeks(2);
    (two_weeks_ago, today.clone())
}

/// If it fails to create a date time becuase of system time being incorrect on machine,
/// then, it will return the date time this function was created
fn get_today_naive_date_time() -> NaiveDateTime {
    if let Ok(now) = SystemTime::now().duration_since(UNIX_EPOCH) {
        NaiveDateTime::from_timestamp(now.as_secs() as i64, 0)
    } else {
        NaiveDate::from_ymd(2020, 04, 28).and_hms(22, 51, 28)
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
        let (from, _) = super::last_two_weeks(&a_day);

        // then
        assert_eq!(two_weeks_ago_timestamp, from.timestamp());
    }

    #[test]
    fn test_get_commits() {
        let repo = super::GitRepo::new_test("./");
        let commits = repo.get_log();
        assert!(commits.is_ok());
        assert_eq!(
            commits.as_ref().unwrap(),
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
}
