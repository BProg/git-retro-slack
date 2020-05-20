const SUFFIX_EMOJIES: [char; 10] = ['🙌', '👍', '🙏', '🎉', '🚀', '🤘', '👏', '🙌', '👍', '🙏'];
const START_ROW: &str = r#"
A reminder on how cool you all are 😎
A year ago, this same day you've written history 📜
"#;

pub fn prettify(commits: &Vec<String>) -> String {
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

fn increase_index(i: usize) -> usize {
    let next = i + 1;
    if next > SUFFIX_EMOJIES.len() {
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
        let next_index = super::increase_index(super::SUFFIX_EMOJIES.len());
        assert_eq!(0, next_index);
    }
}
