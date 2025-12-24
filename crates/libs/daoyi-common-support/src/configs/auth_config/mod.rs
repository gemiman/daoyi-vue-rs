use merge::Merge;
use serde::Deserialize;
use wax::{Glob, Pattern};

#[derive(Debug, Deserialize, Default, Merge)]
pub struct AuthConfig {
    #[merge(strategy = merge::option::overwrite_none)]
    ignored_urls: Option<Vec<String>>,
}
impl AuthConfig {
    pub fn ignored_urls(&self) -> Vec<&str> {
        self.ignored_urls
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|s| s.as_str())
            .collect()
    }

    pub fn is_ignored_url(&self, url: &str) -> bool {
        path_any_matches(&self.ignored_urls(), url).unwrap_or(false)
    }
}

fn path_matches(pattern: &str, target: &str) -> anyhow::Result<bool> {
    // 将通配符模式编译为 Glob 表达式
    let glob = Glob::new(pattern)?;
    // 判断目标路径是否匹配该模式
    Ok(glob.is_match(target))
}

fn path_any_matches<A: AsRef<str>>(patterns: &[A], target: &str) -> anyhow::Result<bool> {
    for pattern in patterns {
        if path_matches(pattern.as_ref(), target)? {
            return Ok(true);
        }
    }
    Ok(false)
}
