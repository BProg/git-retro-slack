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
    pretty
}

fn increase_index(i: usize) -> usize {
    let next = i + 1;
    if next > 9 {
        0
    } else {
        next
    }
}
