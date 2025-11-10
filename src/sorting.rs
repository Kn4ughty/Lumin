/// Returns positive score, required to negate for use
pub fn score_element(user_input: &str, element_text: &str) -> i32 {
    let input = user_input.to_lowercase();
    let mut score = longest_common_substr(element_text, &input);
    if element_text.to_lowercase().starts_with(&input) {
        score += 2;
    }
    score
}

// https://www.geeksforgeeks.org/dsa/longest-common-substring-dp-29/
fn longest_common_substr(s1: &str, s2: &str) -> i32 {
    // Stringslice?

    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    let m = s1.len();
    let n = s2.len();

    let mut prev = vec![0; n + 1];

    let mut res: i32 = 0;

    for i in 1..=m {
        let mut curr = vec![0; n + 1];
        for j in 1..=n {
            if s1[i - 1] == s2[j - 1] {
                curr[j] = prev[j - 1] + 1;
                res = res.max(curr[j]);
            } else {
                curr[j] = 0;
            }
        }
        prev = curr;
    }

    res
}

#[test]
fn longest_common_substr_correct() {
    assert_eq!(longest_common_substr("hello world", "world"), 5);
    assert_eq!(
        longest_common_substr("geeksforgeeks", "ggeegeeksquizpractice"),
        5
    );
    assert_eq!(longest_common_substr("", ""), 0);
}
