// https://www.geeksforgeeks.org/dsa/longest-common-substring-dp-29/
pub fn longest_common_substr(s1: &str, s2: &str) -> i32 {
    // Stringslice?

    let s1: Vec<char> = s1.chars().collect();
    let s2: Vec<char> = s2.chars().collect();

    let m = s1.len(); // This will have problems because of difference of chars and bytes
    let n = s2.len();

    let mut prev = vec![0; n + 1];

    let mut res: i32 = 0;

    for i in 1..m + 1 {
        let mut curr = vec![0; n + 1];
        for j in 1..n + 1 {
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
