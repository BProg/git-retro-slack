use crate::printer;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime};
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

    pub fn new_test(repo_path: &str) -> Self {
        Self {
            repo: Repository::open(repo_path),
            today: get_date_time_with_existing_commits(),
        }
    }

    pub fn get_log(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let (datetime_from, datetime_to) = create_naive_date_time_range(&self.today, 1);
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
        match repo.revwalk() {
            Err(e) => Err(e.into()),
            Ok(mut revwalk) => match revwalk.push_head() {
                Err(e) => Err(e.into()),
                Ok(_) => self.fold_revs(&mut revwalk, &repo, &from, &to),
            },
        }
    }

    fn fold_revs(
        &self,
        revwalk: &mut git2::Revwalk,
        repo: &Repository,
        from: &Time,
        to: &Time,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let summary = revwalk.fold(Vec::<String>::new(), |mut summary, oid| match oid {
            Err(_) => summary,
            Ok(oid) => match repo.find_commit(oid) {
                Err(_) => summary,
                Ok(commit) => {
                    if self.is_valid_commit(&commit, &from, &to) {
                        printer::print_commit(&commit);
                        summary.push(summarize(&commit));
                    }
                    summary
                }
            },
        });
        Ok(summary)
    }

    fn is_valid_commit(&self, commit: &Commit, from: &Time, to: &Time) -> bool {
        let commit_time_secs = commit.time().seconds();
        commit_time_secs > from.seconds() && commit_time_secs < to.seconds()
    }
}

fn clone_git2_error(error: &git2::Error) -> git2::Error {
    git2::Error::from_str(error.message())
}

fn summarize(commit: &Commit) -> String {
    let name: String = commit.author().name().map_or("".into(), |name| name.into());
    let short = commit
        .summary()
        .map(|s| String::from(s))
        .unwrap_or("".into());
    format!("{} {}", name, short)
}

fn create_naive_date_time_range(
    today: &NaiveDateTime,
    years_back: i32,
) -> (NaiveDateTime, NaiveDateTime) {
    let year_ago_date = NaiveDate::from_ymd(
        today.date().year() - years_back,
        today.date().month(),
        today.date().day(),
    );
    let year_ago_next_day_date = year_ago_date.succ();
    let year_ago_date_time = NaiveDateTime::new(year_ago_date, NaiveTime::from_hms(0, 0, 0));
    let year_ago_next_day_date_time =
        NaiveDateTime::new(year_ago_next_day_date, NaiveTime::from_hms(0, 0, 0));

    (year_ago_date_time, year_ago_next_day_date_time)
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

fn get_date_time_with_existing_commits() -> NaiveDateTime {
    NaiveDate::from_ymd(2020, 04, 22).and_hms(22, 51, 28)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    #[test]
    fn get_time_range() {
        // given
        let a_day = NaiveDate::from_ymd(2020, 05, 01).and_hms(22, 22, 22);
        let year_back_timestamp = NaiveDate::from_ymd(2019, 05, 01)
            .and_hms(0, 0, 0)
            .timestamp();
        let year_back_next_day_timestamp = NaiveDate::from_ymd(2019, 05, 02)
            .and_hms(0, 0, 0)
            .timestamp();

        // when
        let (from, to) = super::create_naive_date_time_range(&a_day, 1);

        // then
        assert_eq!(year_back_timestamp, from.timestamp());
        assert_eq!(year_back_next_day_timestamp, to.timestamp());
    }
}
