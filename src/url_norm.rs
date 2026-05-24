use anyhow::{Context, Result};
use url::Url;

pub fn normalize(input: &str) -> Result<String> {
    let mut u = Url::parse(input).with_context(|| format!("parsing URL: {input}"))?;
    if let Some(host) = u.host_str() {
        let lower = host.to_ascii_lowercase();
        u.set_host(Some(&lower)).ok();
    }
    u.set_fragment(None);
    let mut s = u.to_string();
    if s.ends_with('/') && u.path() == "/" {
        // leave bare-host URL alone
    } else if s.ends_with('/') {
        s.pop();
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercases_host() {
        assert_eq!(
            normalize("https://LeetCode.com/problems/two-sum").unwrap(),
            "https://leetcode.com/problems/two-sum"
        );
    }

    #[test]
    fn strips_trailing_slash() {
        assert_eq!(
            normalize("https://leetcode.com/problems/two-sum/").unwrap(),
            "https://leetcode.com/problems/two-sum"
        );
    }

    #[test]
    fn strips_fragment() {
        assert_eq!(
            normalize("https://leetcode.com/problems/two-sum/#code").unwrap(),
            "https://leetcode.com/problems/two-sum"
        );
    }
}
