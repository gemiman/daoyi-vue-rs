use crate::error::ApiResult;
use regex::Regex;
use std::cell::LazyCell;
use std::collections::HashMap;

const PATTERN_PARAMS: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"\{(.*?)}").expect("Failed to compile params regex"));

pub fn parse_template_content_params(content: &str) -> Vec<String> {
    PATTERN_PARAMS
        .captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

pub async fn format_template_content(
    content: &str,
    params: &HashMap<String, String>,
) -> ApiResult<String> {
    Ok(strfmt::strfmt(content, params)?)
}
